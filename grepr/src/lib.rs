use clap::{Arg, Command};
use command_utils::{open, MyResult};
use regex::{Regex, RegexBuilder};

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
                .num_args(1),
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
                .num_args(0..=1),
        )
        .arg(
            Arg::new("invert_match")
                .help("Invert match")
                .short('v')
                .long("invert-match")
                .num_args(0..=1),
        )
        .arg(
            Arg::new("recursive")
                .help("Recursive search")
                .short('r')
                .long("recursive")
                .num_args(0..=1),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case insensitive search")
                .short('i')
                .long("insensitive")
                .num_args(0..=1),
        )
        .get_matches();

    Ok(Config {
        pattern: Regex::new("")?,
        files: vec![],
        recursive: false,
        count: false,
        invert_match: false,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
