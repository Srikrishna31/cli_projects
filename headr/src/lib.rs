use clap::{value_parser, Arg, Command};
use command_utils::{open, MyResult};
use std::io::{BufRead, Read};

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
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(mut f) => {
                if num_files > 1 {
                    println!(
                        "{}===> {} <===",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    )
                }

                if let Some(num) = config.bytes {
                    let bytes: Result<Vec<_>, _> = f.bytes().take(num).collect();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    let mut line = String::new();
                    // Use BufRead::read_line so that the newline character is not removed,
                    // Which will be useful in dealing with OS specific end line characters.
                    for _ in 0..config.lines {
                        let bytes = f.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}
