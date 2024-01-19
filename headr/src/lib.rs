use clap::{value_parser, Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .arg(
            Arg::new("input_files")
                .value_name("FILE")
                .help("Input files separated by spaces")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("count")
                .short('n')
                .long("lines")
                .help("Print the first K lines instead of the first 10")
                .num_args(1)
                .required(false)
                .default_value("10")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Print the first K bytes of each file")
                .num_args(1)
                .required(false)
                .conflicts_with("count")
                .value_parser(value_parser!(usize)),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("input_files")
            .unwrap()
            .map(|f| f.to_owned())
            .collect(),
        lines: matches
            .get_one::<usize>("count")
            .map(|f| f.to_owned())
            .unwrap_or(10),
        bytes: matches.get_one::<usize>("bytes").map(|f| f.to_owned()),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);

    Ok(())
}
