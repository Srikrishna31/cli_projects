use clap::{Arg, Command};
use command_utils::MyResult;
use std::ops::Range;

type PositionList = Vec<Range<usize>>;
#[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("cutr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust cut")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(0..),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
                .short('b')
                .long("bytes")
                .num_args(0..),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .short('c')
                .long("chars")
                .num_args(0..),
        )
        .arg(
            Arg::new("delim")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .short('d')
                .long("delim")
                .num_args(0..1)
                .default_value("\t"),
        )
        .arg(
            Arg::new("fields")
                .value_name("FIELDS")
                .help("Selected fields")
                .short('f')
                .long("fields")
                .num_args(0..),
        )
        .get_matches();

    Ok(Config {
        files: vec![],
        delimiter: ',' as u8,
        extract: Extract::Fields(vec![0..1]),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}
