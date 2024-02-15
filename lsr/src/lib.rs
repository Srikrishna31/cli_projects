use clap::{Arg, Command};
use command_utils::{open, MyResult};

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
    dbg!(config);
    Ok(())
}
