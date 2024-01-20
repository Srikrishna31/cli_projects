use clap::{value_parser, Arg, Command};
use command_utils::{open, MyResult};
use std::io::{BufRead, ErrorKind, Read};

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
    let mut buf = if let Some(bytes) = config.bytes {
        vec![0u8; bytes]
    } else {
        vec![0u8; 0]
    };
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(mut f) => {
                if config.bytes.is_some() {
                    match f.read_exact(&mut buf) {
                        Ok(_) => println!("{}", String::from_utf8_lossy(&buf)),
                        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                            println!("{}", String::from_utf8_lossy(&buf))
                        }
                        Err(e) => eprintln!("{e}"),
                    }
                    buf.clear();
                } else {
                    for line in f.lines().take(config.lines) {
                        println!("{}", line?);
                    }
                }
            }
        }
    }

    Ok(())
}
