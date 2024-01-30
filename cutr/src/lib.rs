mod data_extractor;
mod range_parser;

use crate::data_extractor::{extract_bytes, extract_chars};
use clap::{Arg, ArgMatches, Command};
use command_utils::{open, MyResult};
use range_parser::parse_pos;
use std::io::BufRead;
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
                .num_args(1..),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
                .short('b')
                .long("bytes")
                .num_args(0..)
                .conflicts_with_all(&["chars", "fields"]),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .short('c')
                .long("chars")
                .num_args(0..)
                .conflicts_with_all(&["bytes", "fields"]),
        )
        .arg(
            Arg::new("fields")
                .value_name("FIELDS")
                .help("Selected fields")
                .short('f')
                .long("fields")
                .num_args(0..)
                .conflicts_with_all(&["chars", "bytes"]),
        )
        .arg(
            Arg::new("delim")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .short('d')
                .long("delim")
                .num_args(1)
                .default_value("\t"),
        )
        .get_matches();

    let delimiter = parse_delimiter(&matches)?;
    let extract = parse_fields_bytes_or_chars(&matches)?;
    Ok(Config {
        files: matches
            .get_many::<String>("file")
            .unwrap()
            .map(|f| f.to_owned())
            .collect(),
        delimiter,
        extract,
    })
}

fn parse_fields_bytes_or_chars(matches: &ArgMatches) -> MyResult<Extract> {
    let fields = matches.get_one::<String>("fields");
    let bytes = matches.get_one::<String>("bytes");
    let chars = matches.get_one::<String>("chars");

    if fields.is_none() && bytes.is_none() && chars.is_none() {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    }

    let extract = if fields.is_some() && bytes.is_none() && chars.is_none() {
        Extract::Fields(parse_pos(fields.unwrap())?)
    } else if fields.is_none() && bytes.is_some() && chars.is_none() {
        Extract::Bytes(parse_pos(bytes.unwrap())?)
    } else if fields.is_none() && bytes.is_none() && chars.is_some() {
        Extract::Chars(parse_pos(chars.unwrap())?)
    } else {
        return Err(From::from(
            "Only one option of --fields, --bytes, or --chars is accepted",
        ));
    };

    Ok(extract)
}

#[inline]
fn parse_delimiter(matches: &ArgMatches) -> MyResult<u8> {
    let delimiter = matches.get_one::<String>("delim").unwrap();
    if delimiter.len() != 1 {
        return Err(From::from(format!(
            "--delim \"{delimiter}\" must be a single byte"
        )));
    }
    Ok(delimiter.as_bytes()[0])
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(f) => {
                for line in f.lines() {
                    let line = line?;
                    let res = match &config.extract {
                        Extract::Chars(pos_list) => extract_chars(&line, &pos_list),
                        Extract::Bytes(pos_list) => extract_bytes(&line, &pos_list),
                        Extract::Fields(_) => "".to_string(),
                    };
                    println!("{res}");
                }
            }
        }
    }
    Ok(())
}
