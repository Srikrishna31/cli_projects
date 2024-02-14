use clap::{Arg, Command};
use command_utils::{open, MyResult};
use regex::{Regex, RegexBuilder};
use std::fmt::Debug;
use std::io::BufRead;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust grep")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Pattern to search for")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s) separated by spaces")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("count")
                .help("Count occurrences")
                .short('c')
                .long("count")
                .num_args(0),
        )
        .arg(
            Arg::new("invert_match")
                .help("Invert match")
                .short('v')
                .long("invert-match")
                .num_args(0),
        )
        .arg(
            Arg::new("recursive")
                .help("Recursive search")
                .short('r')
                .long("recursive")
                .num_args(0),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case insensitive search")
                .short('i')
                .long("insensitive")
                .num_args(0),
        )
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(matches.get_flag("insensitive"))
        .build()
        .map_err(|_| format!("Invalid pattern \"{pattern}\""))?;

    Ok(Config {
        pattern,
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|f| f.to_owned())
            .collect(),
        recursive: matches.get_flag("recursive"),
        count: matches.get_flag("count"),
        invert_match: matches.get_flag("invert_match"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let print_file = config.files.len() > 1 || config.recursive;
    let entries = find_files(&config.files, config.recursive);
    for entry in entries {
        match entry {
            Err(e) => eprintln!("{e}"),
            Ok(f) => {
                let file = open(&f)?;
                let lines = find_lines(file, &config.pattern, config.invert_match);
                if config.count {
                    if print_file {
                        println!("{f}:{}", lines.count());
                    } else {
                        println!("{}", lines.count());
                    }
                } else {
                    lines.for_each(|l| {
                        if print_file {
                            println!("{f}:{l}")
                        } else {
                            println!("{l}");
                        }
                    });
                }
            }
        }
    }
    Ok(())
}

fn find_files<'a>(
    paths: &'a [String],
    recursive: bool,
) -> Box<dyn Iterator<Item = MyResult<String>> + 'a> {
    // I have to use Box::new twice here since the types from each branch of if are different.
    // Wrapping in box ensures that the type is unified and the function signature is satisfied.
    if recursive {
        Box::new(paths.iter().flat_map(|p| {
            walkdir::WalkDir::new(p)
                .into_iter()
                .filter_map(|e| {
                    if e.is_err() {
                        eprintln!("{}", &e.unwrap_err());
                        return None;
                    }
                    e.ok()
                })
                .filter(|e| e.file_type().is_file())
                .map(|e| Ok(e.path().to_string_lossy().to_string()))
        }))
    } else {
        Box::new(paths.iter().map(|p| {
            let p = std::path::Path::new(p);
            p.try_exists().map_err(From::from).and_then(|b| {
                if b {
                    if p.is_file() {
                        Ok(String::from(p.to_str().unwrap()))
                    } else {
                        Err(From::from(format!(
                            "{} is a directory",
                            p.to_str().unwrap()
                        )))
                    }
                } else {
                    let p = p.to_str().unwrap();
                    open(p)
                        .map(|_| String::from(p))
                        .map_err(|e| From::from(format!("{p}: {e}")))
                }
            })
        }))
    }
}

fn find_lines<'a, T: BufRead + 'a>(
    file: T,
    pattern: &'a Regex,
    invert_match: bool,
) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(file.lines().filter_map(move |line| match line {
        Ok(l) if pattern.is_match(&l) != invert_match => Some(l),
        _ => None,
    }))
}

#[cfg(test)]
mod tests {
    use super::{find_files, find_lines};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;
    use utils::random_string;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let paths = &["./tests/inputs/fox.txt".to_string()];
        let files = find_files(paths, false).collect::<Vec<_>>();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let paths = &["./tests/inputs".to_string()];
        let files = find_files(paths, false).collect::<Vec<_>>();

        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recursively finds files in a directory
        let paths = &["./tests/inputs".to_string()];
        let res = find_files(paths, true);

        let mut files = res
            .map(|f| f.as_ref().unwrap().replace("\\", "/"))
            .collect::<Vec<String>>();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt"
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = random_string(None);

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false).collect::<Vec<_>>();

        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let mut file = Cursor::new(&text);
        let matches = find_lines(&mut file, &re1, false);
        assert_eq!(matches.collect::<Vec<String>>(), vec!["Lorem"]);

        // When inverted, the function should match the other two lines
        let mut file = Cursor::new(&text);
        let matches = find_lines(&mut file, &re1, true);
        assert_eq!(matches.collect::<Vec<String>>(), vec!["Ipsum", "DOLOR"]);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let mut file = Cursor::new(&text);
        let matches = find_lines(&mut file, &re2, false);
        assert_eq!(matches.collect::<Vec<String>>(), vec!["Lorem", "DOLOR"]);

        // When inverted, the one remaining line should match
        let mut file = Cursor::new(&text);
        let matches = find_lines(&mut file, &re2, true);
        assert_eq!(matches.collect::<Vec<String>>(), vec!["Ipsum"]);
    }
}
