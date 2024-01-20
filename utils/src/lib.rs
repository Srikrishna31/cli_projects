use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

pub type TestResult = Result<(), Box<dyn Error>>;

pub fn random_string(length: Option<usize>) -> String {
    let length = match length {
        Some(n) => n,
        _ => 7,
    };

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn gen_bad_file() -> String {
    loop {
        let filename = random_string(None);
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}
