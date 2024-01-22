use clap::{Arg, Command};
use command_utils::{open, MyResult};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufWriter, Write};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .num_args(1)
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("count")
                .value_name("COUNT")
                .num_args(0)
                .help("Show counts")
                .short('c')
                .long("count")
                .required(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.get_one::<String>("in_file").unwrap().to_owned(),
        out_file: matches.get_one::<String>("out_file").map(|f| f.to_owned()),
        count: matches.get_flag("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    match open(&config.in_file) {
        Err(e) => {
            eprintln!("{}: {e}", config.in_file);
            return Err(e);
        }
        Ok(mut f) => {
            let mut prev_line = String::new();
            let mut count = 0;
            let mut line = String::new();
            let mut out_file = if let Some(out_path) = config.out_file {
                File::create(out_path).map(|f| Box::new(f) as Box<dyn Write>)?
            } else {
                Box::new(io::stdout())
            };
            loop {
                let bytes = f.read_line(&mut line)?;
                if bytes == 0 {
                    break;
                }
                if line != prev_line {
                    writeln!(
                        &mut out_file,
                        "{}{}",
                        if config.count {
                            format!(" {count}")
                        } else {
                            "".to_string()
                        },
                        prev_line
                    )?;
                    prev_line.clear();
                    prev_line.push_str(&line);
                    count = 0;
                } else {
                    count += 1;
                }
            }
        }
    }
    Ok(())
}
