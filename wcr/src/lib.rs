use clap::{Arg, Command};
use command_utils::{open, MyResult};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::new("input_files")
                .value_name("FILE")
                .help("Input files separated by spaces")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("Show byte count")
                .short('c')
                .long("bytes")
                .num_args(0)
                .required(false),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("Show line count")
                .short('l')
                .long("lines")
                .num_args(0)
                .required(false),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .help("Show character count")
                .short('m')
                .long("chars")
                .num_args(0)
                .required(false),
        )
        .arg(
            Arg::new("words")
                .value_name("WORDS")
                .help("Show word count")
                .short('w')
                .long("words")
                .num_args(0)
                .required(false),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("input_files")
            .unwrap()
            .map(|f| f.to_owned())
            .collect(),
        lines: matches.get_flag("lines"),
        words: matches.get_flag("words"),
        bytes: matches.get_flag("bytes"),
        chars: matches.get_flag("chars"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);

    Ok(())
}
