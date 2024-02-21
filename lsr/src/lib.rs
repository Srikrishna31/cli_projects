use clap::{Arg, Command};
use command_utils::{open, MyResult};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("lsr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Files and/or directories")
                .num_args(0..)
                .default_value("."),
        )
        .arg(
            Arg::new("all")
                .help("Show all files")
                .short('a')
                .long("all")
                .num_args(0),
        )
        .arg(
            Arg::new("long")
                .help("Long listing")
                .short('l')
                .long("long")
                .num_args(0),
        )
        .get_matches();

    Ok(Config {
        paths: matches
            .get_many::<String>("files")
            .unwrap()
            .map(String::clone)
            .collect(),
        long: matches.get_flag("long"),
        show_hidden: matches.get_flag("all"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    let paths = find_dir_entries(&config.paths, config.show_hidden)?;
    for path in paths {
        println!("{}", path.display());
    }
    Ok(())
}

fn find_dir_entries(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut entries = vec![];
    for name in paths {
        match fs::metadata(name) {
            Err(e) => eprintln!("{name}: {e}"),
            Ok(meta) => {
                if meta.is_dir() {
                    for entry in fs::read_dir(name)? {
                        let entry = entry?;
                        let name = entry.file_name();
                        let name = name.to_string_lossy();
                        if show_hidden || !name.starts_with('.') {
                            entries.push(entry.path());
                        }
                    }
                } else {
                    entries.push(PathBuf::from(name));
                }
            }
        }
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use crate::find_dir_entries;

    #[test]
    fn test_find_dir_entries() {
        // Find all nonhidden entries in a directory
        let res = find_dir_entries(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt"
            ]
        );

        // Find all entries in a directory
        let res = find_dir_entries(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt"
            ]
        );

        // Any existing file should be found even if hidden
        let res = find_dir_entries(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // Test multiple path arguments
        let res = find_dir_entries(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }
}
