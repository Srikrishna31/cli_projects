use clap::builder::PossibleValue;
use clap::{Arg, Command, ValueEnum};
use command_utils::{open, MyResult};
use regex::Regex;

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

    let mut names = vec![];
    for res in matches.get_many::<String>("name") {
        for re in res {
            names.push(Regex::new(re)?);
        }
    }
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
    dbg!(config);

    Ok(())
}
