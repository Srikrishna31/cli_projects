use assert_cmd::Command;
use predicates::prelude::*;
use std::fmt::format;
use std::{
    error::Error,
    fs::{self, File},
    io::prelude::*,
};
use utils::{gen_bad_file, random_string};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";

const TWO: &str = "./tests/inputs/two.txt";

const THREE: &str = "./tests/inputs/three.txt";

const TEN: &str = "./tests/inputs/ten.txt";

#[test]
fn dies_bad_bytes() -> TestResult {
    let bad = random_string(None);
    let expected = format!("invalid value '{}' for '--bytes <bytes>", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}
