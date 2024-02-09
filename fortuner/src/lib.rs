use clap::{Arg, Command};
use command_utils::MyResult;
use itertools::Itertools;
use regex::{Regex, RegexBuilder};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("fortuner")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust fortune")
        .arg(
            Arg::new("sources")
                .value_name("FILE")
                .help("Fortune sources")
                .default_value("fortune")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Pattern")
                .short('m')
                .long("pattern")
                .num_args(1),
        )
        .arg(
            Arg::new("seed")
                .value_name("SEED")
                .help("Random seed")
                .short('s')
                .long("seed")
                .num_args(1),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case-insensitive pattern matching")
                .short('i')
                .long("insensitive")
                .num_args(0),
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .map(|p| {
            RegexBuilder::new(p.as_str())
                .case_insensitive(matches.get_flag("insensitive"))
                .build()
                .map_err(|e| format!("Invalid --pattern \"{e}\""))
        })
        .transpose()?;

    Ok(Config {
        sources: matches
            .get_many::<String>("sources")
            .unwrap()
            .map(|s| s.to_owned())
            .collect(),
        pattern,
        seed: matches
            .get_one::<String>("seed")
            .map(|s| parse_u64(s.as_str()))
            .transpose()?,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

fn parse_u64(val: &str) -> MyResult<u64> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(format!("\"{val}\" not a valid integer"))),
    }
}

fn find_files<'a, F>(
    paths: &'a [String],
    filter_func: F,
) -> MyResult<Box<dyn Iterator<Item = PathBuf> + 'a>>
where
    F: Fn(&PathBuf) -> bool + 'a + Copy,
{
    Ok(Box::new(paths.iter().flat_map(move |p| {
        walkdir::WalkDir::new(p)
            .into_iter()
            .filter_map(|e| {
                if e.is_err() {
                    eprintln!("{}", &e.unwrap_err());
                    return None;
                }
                e.ok().map(|e| e.path().to_path_buf())
            })
            .filter(move |e| e.is_file() && filter_func(e))
    })))
}

fn find_files_by_extension(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    let res = find_files(paths, |p| {
        p.exists() && p.extension().map_or(true, |e| e != "dat")
    });
    res.map(|f| f.sorted().unique().collect())
}

#[cfg(test)]
mod tests {
    use super::{find_files, find_files_by_extension, parse_u64};

    #[test]
    fn test_parse_u64() {
        let res = parse_u64("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");

        let res = parse_u64("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);

        let res = parse_u64("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let paths = ["./tests/inputs/jokes".to_string()];
        let res = find_files_by_extension(&paths);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // Fails to find a bad file
        let paths = ["/path/does/not/exist".to_string()];
        let res = find_files_by_extension(&paths);
        // assert!(res.is_err());

        // Finds all the input files, excludes "*.dat"
        let paths = ["./tests/inputs".to_string()];
        let res = find_files_by_extension(&paths);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        // Test for multiple sources, path must be unique and sorted
        let paths = [
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ];
        let res = find_files_by_extension(&paths);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string());
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string());
        }
    }
}
