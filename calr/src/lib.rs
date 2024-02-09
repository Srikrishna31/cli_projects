use chrono::NaiveDate;
use clap::{Arg, Command};
use command_utils::{open, MyResult};

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
 }

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust calendar")
        .get_matches();

    Ok(Config {
        month: None,
        year: 0,
        today: Default::default(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    Ok(())
}
