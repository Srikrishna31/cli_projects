use clap::{parser::ValueSource, Arg, Command};
use command_utils::{open, MyResult};
use std::io::BufRead;

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
    let mut total_lines: usize = 0;
    let mut total_words: usize = 0;
    let mut total_bytes: usize = 0;
    let mut total_chars: usize = 0;

    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(f) => {
                let res = count(f)?;
                println!(
                    "{}{}{}{}{}",
                    format_field(res.num_lines, config.lines),
                    format_field(res.num_words, config.words),
                    format_field(res.num_bytes, config.bytes),
                    format_field(res.num_chars, config.chars),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {filename}")
                    }
                );
                total_lines += res.num_lines;
                total_words += res.num_words;
                total_bytes += res.num_bytes;
                total_chars += res.num_chars;
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        );
    }
    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buf = String::new();
    loop {
        match file.read_line(&mut buf) {
            Ok(0) => break,
            Ok(b) => {
                num_lines += 1;
                num_bytes += b;
                num_words += buf.split_whitespace().count();
                num_chars += buf.chars().count();
                buf.clear();
            }
            Err(e) => {
                num_lines += 1;
                num_bytes += buf.len();
                num_words += buf.split_whitespace().count();
                num_chars += buf.chars().count();
                eprintln!("{e}");
                break;
            }
        }
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
