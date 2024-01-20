use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fmt::format;
use std::{
    error::Error,
    fs::{self, File},
    io::prelude::*,
};
use utils::{gen_bad_file, random_string, TestResult};

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";

const TWO: &str = "./tests/inputs/two.txt";

const THREE: &str = "./tests/inputs/three.txt";

const TEN: &str = "./tests/inputs/ten.txt";

#[rstest]
#[case(&["-c", &random_string(None), EMPTY], "invalid value '{}' for '--bytes <bytes>")]
#[case(&["-n", &random_string(None), EMPTY], "invalid value '{}' for '--lines <count>")]
fn dies_bad_arguments(#[case] args: &[&str], #[case] expected: &str) -> TestResult {
    let bad = args[1];
    let expected = expected.replace("{}", &bad);
    Command::cargo_bin(PRG)?
        .args(args)
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

fn run_stdin(args: &[&str], input_file: &str, expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    // let mut buffer = Vec::new();

    Ok(())
}
