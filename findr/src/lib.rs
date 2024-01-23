use command_utils::{MyResult, open};

pub struct Config {

}

pub fn get_args() -> MyResult<Config> {
    Ok(Config{})
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);

    Ok(())
}