use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

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

pub fn find_files<'a, F>(
    paths: &'a [String],
    filter_func: F,
) -> MyResult<Box<dyn Iterator<Item = PathBuf> + 'a>>
where
    F: Fn(&PathBuf) -> bool + 'a + Copy,
{
    Ok(Box::new(paths.iter().flat_map(move |p| {
        walkdir::WalkDir::new(p)
            .into_iter()
            .filter_map(move |e| {
                if e.is_err() {
                    eprintln!("{p}: {}", &e.unwrap_err());
                    return None;
                }
                e.ok().map(|e| e.path().to_path_buf())
            })
            .filter(move |e| e.is_file() && filter_func(e))
    })))
}

pub fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(format!("Invalid integer \"{val}\""))),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_int;
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
}
