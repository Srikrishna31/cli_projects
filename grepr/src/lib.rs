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
