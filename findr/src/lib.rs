use clap::{Arg, Command};
use command_utils::{open, MyResult};
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
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
                .action(clap::ArgAction::Append),
        )
        .get_matches();

    Ok(Config {
        paths: vec![],
        names: vec![],
        entry_types: vec![],
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);

    Ok(())
}
