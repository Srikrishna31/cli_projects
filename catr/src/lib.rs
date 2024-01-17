use clap::{App, Arg};
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
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("input_files")
                .value_name("FILE")
                .help("Input files separated by spaces")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Number the output lines, starting at 1.")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Number the non-blank output lines, starting at 1.")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("input_files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}
pub fn run(config: Config) -> MyResult<()> {
    config.files.iter().for_each(|f| match open(&f) {
        Err(e) => eprintln!("Failed to open {f}: {e}"),
        Ok(_) => println!("Opened {f}"),
    });
    // dbg!(config);
    Ok(())
}

/// Open stdin if a "-" is passed for file name. otherwise try to open the passed in filename.
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "_" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
