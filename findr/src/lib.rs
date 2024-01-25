use clap::{Arg, Command};
use command_utils::MyResult;
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .num_args(0..),
        )
        .arg(
            Arg::new("name")
                .value_name("NAME")
                .help("Name")
                .short('n')
                .long("name")
                .num_args(0..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("type")
                .value_name("TYPE")
                .help("Entry type")
                .short('t')
                .long("type")
                .num_args(0..)
                .value_parser(["f", "d", "l"])
                // .default_value("f")
                .action(clap::ArgAction::Append),
        )
        .get_matches();

    let entry_types = matches
        .get_many::<String>("type")
        .map(|fs| {
            fs.map(|f| match f.as_str() {
                "d" => EntryType::Dir,
                "f" => EntryType::File,
                "l" => EntryType::Link,
                _ => unreachable!("Invalid type"),
            })
            .collect()
        })
        .unwrap_or_default();

    let names = matches
        .get_many::<String>("name")
        .map(|ns| {
            ns.map(|n| Regex::new(n).map_err(|_| format!("Invalid --name \"{n}\"")))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    Ok(Config {
        paths: matches
            .get_many::<String>("paths")
            .unwrap()
            .map(|f| f.to_owned())
            .collect(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        WalkDir::new(path)
            .into_iter()
            .filter(|en| match en {
                Err(e) => {
                    eprintln!("{e}");
                    false
                }
                Ok(entry) => {
                    config.entry_types.is_empty()
                        || config.entry_types.iter().any(|t| match &t {
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::Link => entry.file_type().is_symlink(),
                            EntryType::File => entry.file_type().is_file(),
                        })
                }
            })
            .filter(|en| match en {
                Ok(en) => {
                    config.names.is_empty()
                        || config.names.iter().any(|re| match en.file_name().to_str() {
                            Some(f) => re.is_match(f),
                            _ => {
                                eprintln!(
                                    "Couldnot convert filename to string: {:?}",
                                    en.file_name()
                                );
                                false
                            }
                        })
                }
                _ => unreachable!("Invalid code path"),
            })
            .for_each(|e| match e {
                Ok(en) => println!("{}", en.path().display()),
                _ => unreachable!("Invalid code path"),
            });
    }
    Ok(())
}
