use chrono::{Datelike, NaiveDate};
use clap::{Arg, Command};
use command_utils::MyResult;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("calr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust calendar")
        .arg(
            Arg::new("YEAR")
                .value_name("YEAR")
                .help("Year (1-9999)")
                .num_args(1),
        )
        .arg(
            Arg::new("month")
                .value_name("MONTH")
                .help("Month name or number (1-12)")
                .short('m')
                .long("month")
                .num_args(1),
        )
        .arg(
            Arg::new("year")
                .value_name("year")
                .help("Show whole current year")
                .short('y')
                .long("year")
                .num_args(0)
                .conflicts_with("month"),
        )
        .get_matches();

    let today = chrono::Local::now().date_naive();
    let year = if let Some(y) = matches.get_one::<String>("YEAR") {
        y.parse().unwrap()
    } else {
        today.year()
    };

    Ok(Config {
        month: if matches.get_flag("year") {
            None
        } else {
            matches
                .get_one::<String>("month")
                .map(|m| m.parse().unwrap())
        },
        year: matches
            .get_flag("year")
            .then(|| today.year())
            .unwrap_or(year),
        today,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    Ok(())
}
