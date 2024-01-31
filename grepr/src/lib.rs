use clap::builder::TypedValueParser;
use clap::{Arg, Command};
use command_utils::{open, MyResult};
use regex::{Regex, RegexBuilder};
use std::fmt::Debug;
use std::str::FromStr;

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
    dbg!(config);
    Ok(())
}
