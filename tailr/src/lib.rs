use circular_queue::CircularQueue;
use clap::{Arg, Command};
use command_utils::{open, LineIterator, MyResult};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    bytes: Option<TakeValue>,
    lines: TakeValue,
    quiet: bool,
}

static NUM_RE: OnceCell<Regex> = OnceCell::new();

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("tailr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                // .default_value("-")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .short('c')
                .long("bytes")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("Number of lines")
                .short('n')
                .long("lines")
                .num_args(1)
                .default_value("10")
                .required(false)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::new("quiet")
                .help("Suppress headers")
                .short('q')
                .long("quiet")
                .num_args(0)
                .required(false),
        )
        .get_matches();
    let bytes = matches
        .get_one::<String>("bytes")
        .map(|f| parse_num(f))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {e}"))?;

    let lines = matches
        .get_one::<String>("lines")
        .map(|f| parse_num(f))
        .transpose()
        .map_err(|e| format!("illegal line count -- {e}"))?;

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(String::to_owned)
            .collect::<Vec<String>>(),
        bytes,
        lines: lines.unwrap(),
        quiet: matches.get_flag("quiet"),
    })
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    let re = NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());
    match re.captures(val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let num = format!("{}{}", sign, caps.get(2).unwrap().as_str());
            if let Ok(v) = num.parse() {
                if sign == "+" && v == 0 {
                    Ok(TakeValue::PlusZero)
                } else {
                    Ok(TakeValue::TakeNum(v))
                }
            } else {
                Err(From::from(val))
            }
        }
        None => Err(From::from(val)),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let print_file_name = config.files.len() > 1;
    for (filenum, file) in config.files.iter().enumerate() {
        match open(file) {
            Ok(f) => {
                if !config.quiet && print_file_name {
                    println!("{}==> {file} <==", if filenum > 0 { "\n" } else { "" });
                }
                match config.bytes {
                    Some(TakeValue::TakeNum(n)) => match n.cmp(&0) {
                        Ordering::Less => {
                            let mut buf = CircularQueue::with_capacity(n.unsigned_abs() as usize);
                            f.bytes().for_each(|b| {
                                buf.push(b.unwrap());
                            });
                            print!(
                                "{}",
                                String::from_utf8_lossy(
                                    &buf.asc_iter().copied().collect::<Vec<u8>>()
                                )
                            );
                        }
                        Ordering::Greater => {
                            let mut v = Vec::new();
                            f.bytes().enumerate().for_each(|(i, b)| {
                                if i >= (n - 1) as usize {
                                    v.push(b.unwrap());
                                }
                            });
                            print!("{}", String::from_utf8_lossy(&v));
                        }
                        _ => {}
                    },
                    Some(TakeValue::PlusZero) => {
                        LineIterator::new(f).for_each(|l| print!("{}", l.unwrap().1))
                    }
                    None => match config.lines {
                        TakeValue::TakeNum(n) => match n.cmp(&0) {
                            Ordering::Less => {
                                let mut buf =
                                    CircularQueue::with_capacity(n.unsigned_abs() as usize);
                                LineIterator::new(f).for_each(|l| {
                                    buf.push(l.unwrap().1);
                                });
                                buf.asc_iter().for_each(|l| print!("{l}"));
                            }
                            Ordering::Greater => {
                                LineIterator::new(f).enumerate().for_each(|(i, l)| {
                                    if i >= (n - 1) as usize {
                                        print!("{}", l.unwrap().1);
                                    }
                                });
                            }
                            _ => {}
                        },
                        TakeValue::PlusZero => {
                            LineIterator::new(f).for_each(|l| print!("{}", l.unwrap().1))
                        }
                    },
                }
            }
            Err(e) => eprintln!("{file}:{e}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_num, TakeValue};

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num(&"3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num(&"+3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num(&"-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(-3));

        // Zero is zero
        let res = parse_num(&"0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(0));

        //Plus zero is special
        let res = parse_num(&"+0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::PlusZero);

        // Test boundaries
        let res = parse_num(&i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MIN + 1));

        let res = parse_num(&(i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MIN + 1));

        let res = parse_num(&format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MAX));

        let res = parse_num(&i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeValue::TakeNum(i64::MIN));

        // A floating-point value is invalid
        let res = parse_num(&"3.14".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any noninteger string is invalid
        let res = parse_num(&"foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
