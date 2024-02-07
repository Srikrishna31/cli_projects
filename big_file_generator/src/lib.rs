use clap::{Arg, Command};
use command_utils::MyResult;
use rand::Rng;
use thousands::Separable;
use utils::random_string;

#[derive(Debug)]
pub struct Config {
    outfile: String,
    lines: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("big_file_generator")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust big file generator")
        .arg(
            Arg::new("outfile")
                .value_name("OUTFILE")
                .help("Output file")
                .short('o')
                .long("outfile")
                .num_args(1)
                .default_value("1M.txt"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("Number of lines")
                .short('n')
                .long("lines")
                .num_args(1)
                .default_value("1000000"),
        )
        .get_matches();

    let lines = matches
        .get_one::<String>("lines")
        .map(|l| parse_positive_int(l))
        .transpose()
        .map_err(|e| format!("-- lines \"{e}\" must be greater than 0"))?;

    Ok(Config {
        lines: lines.unwrap(),
        outfile: matches.get_one::<String>("outfile").unwrap().to_string(),
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    use std::fs::File;
    use std::io::{BufWriter, Write};

    let file = File::create(&config.outfile)?;
    let mut writer = BufWriter::new(file);

    for _ in 0..config.lines {
        let num_words = rand::thread_rng().gen_range(7..15);
        let words = (0..num_words)
            .fold(Vec::with_capacity(num_words), |mut acc, _| {
                acc.push(random_string(None));
                acc
            })
            .join(" ");

        writeln!(writer, "{words}")?;
    }

    println!(
        "Done, wrote {} line{} to \"{}\".",
        config.lines.separate_with_commas(),
        if config.lines == 1 { "" } else { "s" },
        config.outfile
    );

    Ok(())
}
