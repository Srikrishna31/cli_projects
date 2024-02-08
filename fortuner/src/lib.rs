use clap::{Arg, Command};
use command_utils::MyResult;
use regex::{Regex, RegexBuilder};

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("fortuner")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust fortune")
        .arg(
            Arg::new("sources")
                .value_name("FILE")
                .help("Fortune sources")
                .default_value("fortune")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Pattern")
                .short('m')
                .long("pattern")
                .num_args(1),
        )
        .arg(
            Arg::new("seed")
                .value_name("SEED")
                .help("Random seed")
                .short('s')
                .long("seed")
                .num_args(1),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case-insensitive pattern matching")
                .short('i')
                .long("insensitive")
                .num_args(0),
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .map(|p| {
            RegexBuilder::new(p.as_str())
                .case_insensitive(matches.get_flag("insensitive"))
                .build()
                .map_err(|e| format!("Invalid --pattern \"{e}\""))
        })
        .transpose()?;

    Ok(Config {
        sources: matches
            .get_many::<String>("sources")
            .unwrap()
            .map(|s| s.to_owned())
            .collect(),
        pattern,
        seed: matches
            .get_one::<String>("seed")
            .map(|s| parse_u64(s.as_str()))
            .transpose()?,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

fn parse_u64(val: &str) -> MyResult<u64> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(format!("\"{val}\" not a valid integer"))),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_u64;

    #[test]
    fn test_parse_u64() {
        let res = parse_u64("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");

        let res = parse_u64("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);

        let res = parse_u64("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }
}
