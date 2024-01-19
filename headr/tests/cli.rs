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

#[test]
fn dies_bad_lines() -> TestResult {
    let bad = random_string(None);
    let expected = format!("invalid value '{}' for '--lines <count>", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bytes_and_lines() -> TestResult {
    let msg = "the argument '--lines <count>' cannot be used with '--bytes <bytes>'";
    Command::cargo_bin(PRG)?
        .args(&["-n", "1", "-c", "2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(msg));

    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", &bad);
    Command::cargo_bin(PRG)?
        .args([EMPTY, &bad, ONE])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}
