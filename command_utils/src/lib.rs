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
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{filename}: {e}"))?,
        ))),
    }
}

pub struct LineIterator<T: BufRead> {
    file: T,
}

impl<T: BufRead> LineIterator<T> {
    pub fn new(file: T) -> LineIterator<T> {
        LineIterator { file }
    }
}

impl<T> Iterator for LineIterator<T>
where
    T: BufRead,
{
    type Item = MyResult<(usize, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.file.read_line(&mut line) {
            Ok(0) => None,
            Ok(b) => Some(Ok((b, line))),
            Err(e) => Some(Err(From::from(e))),
        }
    }
}
