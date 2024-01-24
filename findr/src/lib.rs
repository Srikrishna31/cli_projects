use command_utils::{open, MyResult};

#[derive(Debug)]
pub struct Config {}

pub fn get_args() -> MyResult<Config> {
    Ok(Config {})
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);

    Ok(())
}
