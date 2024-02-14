use clap::{Arg, Command};
use command_utils::{open, MyResult};

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("lsr")
        .version("0.1.0")
        .author("Krishna Addepalli <coolkrishna31@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::new("all")
                .help("Show hidden files")
                .short('a')
                .long("all")
                .num_args(0),
        );
    Ok(Config {
        paths: vec![],
        long: false,
        show_hidden: false,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
