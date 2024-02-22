use chrono::Local;
use clap::{Arg, Command};
use command_utils::MyResult;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use tabular::{Row, Table};

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
    let paths = find_dir_entries(&config.paths, config.show_hidden)?;
    if config.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
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

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    //                 1  2    3    4    5    6    7    8
    let fmt = "{:<}{:<} {:>} {:<} {:<} {:>} {:<} {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        table.add_row(
            Row::new()
                .with_cell(path.metadata().map_or("-".to_string(), |m| {
                    if m.is_dir() {
                        "d".to_string()
                    } else {
                        "-".to_string()
                    }
                }))
                .with_cell(
                    path.metadata()
                        .map_or("?".to_string(), |m| format_mode(m.mode()).to_string()),
                )
                .with_cell(path.metadata().map_or(0, |m| m.nlink()))
                .with_cell(path.metadata().map_or("?".to_string(), |m| {
                    users::get_user_by_uid(m.uid())
                        .unwrap()
                        .name()
                        .to_string_lossy()
                        .to_string()
                }))
                .with_cell(path.metadata().map_or("?".to_string(), |m| {
                    users::get_group_by_gid(m.gid())
                        .unwrap()
                        .name()
                        .to_string_lossy()
                        .to_string()
                }))
                .with_cell(
                    path.metadata()
                        .map_or("?".to_string(), |m| m.len().to_string()),
                )
                .with_cell(path.metadata().map_or("?".to_string(), |m| {
                    chrono::DateTime::<Local>::from(m.modified().unwrap())
                        .format("%b %e %y %H:%M")
                        .to_string()
                }))
                .with_cell(path.metadata().map_or("?".to_string(), |_| {
                    path.as_path().to_string_lossy().to_string()
                })),
        );
    }

    Ok(format!("{}", table))
}

// Given a file mode in octal format like 0o755, return a string like "rwxr-xr-x"
fn format_mode(mode: u32) -> String {
    // println!("{}", &mode);
    let mut s = String::with_capacity(9);
    for (perm, c) in [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ]
    .iter()
    {
        if mode & perm != 0 {
            s.push(*c);
        } else {
            s.push('-');
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::{find_dir_entries, format_mode, format_output};
    use std::path::PathBuf;

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

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o644), "rw-r--r--");
        assert_eq!(format_mode(0o421), "r---w---x");
        assert_eq!(format_mode(0o777), "rwxrwxrwx");
    }

    fn long_match(
        line: &str,
        expected_name: &str,
        expected_perms: &str,
        expected_size: Option<&str>,
    ) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() > 0 && parts.len() <= 10);

        let perms = parts.get(0).unwrap();
        assert_eq!(perms, &expected_perms);

        if let Some(size) = expected_size {
            let file_size = parts.get(4).unwrap();
            assert_eq!(file_size, &size);
        }

        let display_name = parts.last().unwrap();
        assert_eq!(display_name, &expected_name);
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let res = format_output(&[bustle]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let lines: Vec<&str> = out.split('\n').filter(|s| !s.is_empty()).collect();
        assert_eq!(lines.len(), 1);

        let line1 = lines.first().unwrap();
        long_match(&line1, bustle_path, "-rw-rw-rw-", Some("202"));
    }

    #[test]
    fn test_format_output_two() {
        let res = format_output(&[
            PathBuf::from("tests/inputs/dir"),
            PathBuf::from("tests/inputs/empty.txt"),
        ]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let mut lines: Vec<&str> = out.split('\n').filter(|s| !s.is_empty()).collect();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(
            &empty_line,
            "tests/inputs/empty.txt",
            "-rw-rw-rw-",
            Some("0"),
        );

        let dir_line = lines.remove(0);
        long_match(&dir_line, "tests/inputs/dir", "drwxrwxrwx", None);
    }
}
