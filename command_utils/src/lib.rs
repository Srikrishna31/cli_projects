use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

/// Returns a buffered reader opened on the file if the file exists or
/// if '-' is passed, then returns a buffered reader to standard input.
/// If the file doesn't exist or if there is any other problem in opening
/// file, then returns the corresponding error object.
pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
