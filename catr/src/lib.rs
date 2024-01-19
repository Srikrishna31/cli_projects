use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}
type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("input_files")
                .value_name("FILE")
                .help("Input files separated by spaces")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number the output lines, starting at 1.")
                .num_args(0)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number_nonblank")
                .help("Number the non-blank output lines, starting at 1.")
                .num_args(0),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("input_files")
            .unwrap()
            .map(|f| f.to_owned())
            .collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}
pub fn run(config: Config) -> MyResult<()> {
    for f in config.files {
        match open(&f) {
            Err(e) => eprintln!("{f}: {e}"),
            Ok(b) => {
                let mut counter = 1;
                for (number, line) in b.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{line}", number + 1)
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            println!("{:>6}\t{line}", counter);
                            counter += 1;
                        } else {
                            println!("{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

/// Open stdin if a "-" is passed for file name. otherwise try to open the passed in filename.
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
