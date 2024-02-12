use chrono::{Datelike, NaiveDate};
use clap::{Arg, Command};
use command_utils::MyResult;
use std::str::FromStr;

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
                .num_args(1)
                .conflicts_with("year"),
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
        parse_year(y)?
    } else {
        today.year()
    };

    Ok(Config {
        month: if matches.get_flag("year") {
            None
        } else {
            matches
                .get_one::<String>("month")
                .map(|v| parse_month(v))
                .transpose()?
        },
        year: matches
            .get_flag("year")
            .then(|| today.year())
            .unwrap_or(year),
        today,
    })
}

fn parse_year(year: &str) -> MyResult<i32> {
    parse_int::<i32>(year).and_then(|y| {
        if y < 1 || y > 9999 {
            Err(From::from(format!(
                "year \"{year}\" not in the range 1-9999"
            )))
        } else {
            Ok(y)
        }
    })
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(format!("Invalid integer \"{val}\""))),
    }
}

fn parse_month(month: &str) -> MyResult<u32> {
    let res = parse_int::<u32>(month);
    match parse_int::<u32>(month) {
        Ok(m) if m >= 1 && m <= 12 => Ok(m),
        Ok(_) => Err(From::from(format!(
            "month \"{month}\" not in the range 1-12"
        ))),
        _ => match month.to_lowercase().as_str() {
            "ja" | "jan" | "january" => Ok(1),
            "f" | "feb" | "february" => Ok(2),
            "mar" | "march" => Ok(3),
            "ap" | "apr" | "april" => Ok(4),
            "may" => Ok(5),
            "jun" | "june" => Ok(6),
            "jul" | "july" => Ok(7),
            "au" | "aug" | "august" => Ok(8),
            "s" | "sep" | "september" => Ok(9),
            "o" | "oct" | "october" => Ok(10),
            "n" | "nov" | "november" => Ok(11),
            "d" | "dec" | "december" => Ok(12),
            _ => Err(From::from(format!("Invalid month \"{month}\""))),
        },
    }
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    unimplemented!();
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    unimplemented!();
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{format_month, last_day_in_month, parse_int, parse_month, parse_year};
    use chrono::NaiveDate;

    #[test]
    fn test_parse_int() {
        // Parse positive int as usize
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        // Parse negative int as i32
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        // Fail on a string
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);

        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);

        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"0\" not in the range 1-9999"
        );

        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"10000\" not in the range 1-9999"
        );

        let res = parse_year("foo");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("january");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1-12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1-12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let leap_february = vec![
            "   February 2020      ",
            "Su Mo Tu We Th Fr Sa  ",
            "                   1  ",
            " 2  3  4  5  6  7  8  ",
            " 9 10 11 12 13 14 15  ",
            "16 17 18 19 20 21 22  ",
            "23 24 25 26 27 28 29  ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 2, true, today), leap_february);

        let may = vec![
            "       May            ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, false, today), may);

        let april_hl = vec![
            "      April 2021      ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6  7  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];
        let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
        assert_eq!(format_month(2021, 4, true, today), april_hl);
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(
            last_day_in_month(2020, 1),
            NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
        );

        assert_eq!(
            last_day_in_month(2020, 2),
            NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
        );

        assert_eq!(
            last_day_in_month(2020, 4),
            NaiveDate::from_ymd_opt(2020, 4, 30).unwrap()
        );
    }
}
