use command_utils::MyResult;
use std::ops::Range;

type PositionList = Vec<Range<usize>>;
#[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}
pub fn get_args() -> MyResult<Config> {
    Ok(Config {
        files: vec![],
        delimiter: ',' as u8,
        extract: Extract::Fields(vec![0..1]),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}
