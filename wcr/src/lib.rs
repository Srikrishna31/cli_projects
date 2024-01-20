use clap::{parser::ValueSource, Arg, Command};
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
                .required(false)
                .conflicts_with("bytes"),
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

    // if any of the flags came from command line then others should be false. If none of the options
    // came from command line then make them true.
    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    if ["lines", "words", "bytes", "chars"]
        .iter()
        .all(|v| matches.value_source(v) != Some(ValueSource::CommandLine))
    {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches
            .get_many::<String>("input_files")
            .unwrap()
            .map(|f| f.to_owned())
            .collect(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(_) => println!("Opened {filename}"),
        }
    }

    Ok(())
}
