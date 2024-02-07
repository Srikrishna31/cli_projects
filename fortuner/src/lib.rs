use clap::{Arg, Command};
use command_utils::MyResult;
use regex::{Regex, RegexBuilder};

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
                .value_name("SOURCE")
                .help("Fortune sources")
                .default_value("fortune")
                .num_args(0..)
        )
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Pattern")
                .short('p')
                .long("pattern")
                .num_args(1)
        )
        .arg(
            Arg::new("seed")
                .value_name("SEED")
                .help("Seed")
                .short('s')
                .long("seed")
                .num_args(1)
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .map(|p| RegexBuilder::new())
        .transpose()
        .map_err(|e| format!("-- pattern \"{e}\" is not a valid regex"))?;
