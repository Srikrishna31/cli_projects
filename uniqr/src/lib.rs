use clap::{Arg, Command};
use command_utils::{open, MyResult};

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
        .arg(Arg::new(""))
        .get_matches();

    Ok(Config {
        in_file: "".to_string(),
        out_file: None,
        count: false,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
