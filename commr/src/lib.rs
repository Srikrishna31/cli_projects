use clap::{Arg, Command};
use command_utils::{open, LineIterator, MyResult};
use std::io::BufRead;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust comm")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("show_col1")
                .help("Suppress printing of column 1")
                .short('1')
                .num_args(0),
        )
        .arg(
            Arg::new("show_col2")
                .help("Suppress printing of column 2")
                .short('2')
                .num_args(0),
        )
        .arg(
            Arg::new("show_col3")
                .help("Suppress printing of column 3")
                .short('3')
                .num_args(0),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case-insensitive comparison of lines")
                .short('i')
                .long("insensitive")
                .num_args(0),
        )
        .arg(
            Arg::new("delimiter")
                .value_name("DELIMITER")
                .help("Output delimiter")
                .short('d')
                .long("delim")
                .num_args(1)
                .default_value("\t"),
        )
        .get_matches();

    Ok(Config {
        file1: matches.get_one::<String>("file1").unwrap().to_string(),
        file2: matches.get_one::<String>("file2").unwrap().to_string(),
        show_col1: !matches.get_flag("show_col1"),
        show_col2: !matches.get_flag("show_col2"),
        show_col3: !matches.get_flag("show_col3"),
        insensitive: matches.get_flag("insensitive"),
        delimiter: matches.get_one::<String>("delimiter").unwrap().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(&config);
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let mut file1 = LineIterator::new(open(file1)?);
    let mut file2 = LineIterator::new(open(file2)?);

    loop {
        let line1 = file1.next();
        let line2 = file2.next();
        match (line1, line2) {
            (Some(l1), Some(l2)) => {
                let (l1, l2) = (l1?, l2?);

                if l1.1 == l2.1 {
                    if config.show_col3 {
                        print!("\t\t{}", l1.1);
                    }
                } else {
                    if config.show_col1 {
                        print!("{}", l1.1);
                    }
                    if config.show_col2 {
                        print!("\t{}", l2.1);
                    }
                }
            }
            (Some(l1), None) => {
                if config.show_col1 {
                    print!("{}", l1?.1);
                }
            }
            (None, Some(l2)) => {
                if config.show_col2 {
                    print!("\t{}", l2?.1);
                }
            }
            (None, None) => {
                break;
            }
        }
        println!();
    }

    Ok(())
}
