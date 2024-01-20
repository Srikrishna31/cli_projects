use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::error::Error;
use std::fs;
use utils::{gen_bad_file, TestResult};

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";

const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }

    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);

    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[rstest]
#[case(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")]
#[case(BUSTLE, &["-n", "-"], "tests/expected/the-bustle.txt.n.stdin.out")]
#[case(BUSTLE, &["-b", "-"], "tests/expected/the-bustle.txt.b.stdin.out")]
fn run_stdin(
    #[case] input_file: &str,
    #[case] args: &[&str],
    #[case] expected_file: &str,
) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[rstest]
#[case(&[EMPTY], "tests/expected/empty.txt.out")]
#[case(&["-n", EMPTY], "tests/expected/empty.txt.n.out")]
#[case(&["-b", EMPTY], "tests/expected/empty.txt.b.out")]
#[case(&[FOX], "tests/expected/fox.txt.out")]
#[case(&["-n", FOX], "tests/expected/fox.txt.n.out")]
#[case(&["-b", FOX], "tests/expected/fox.txt.b.out")]
#[case(&[SPIDERS], "tests/expected/spiders.txt.out")]
#[case(&["-n", SPIDERS], "tests/expected/spiders.txt.n.out")]
#[case(&["-b", SPIDERS], "tests/expected/spiders.txt.b.out")]
#[case(&[BUSTLE], "tests/expected/the-bustle.txt.out")]
#[case(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")]
#[case(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")]
#[case(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")]
#[case(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")]
#[case(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
