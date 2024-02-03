use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use utils::{gen_bad_file, TestResult};

const PRG: &str = "commr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FILE1: &str = "tests/inputs/file1.txt";
const FILE2: &str = "tests/inputs/file2.txt";
const BLANK: &str = "tests/inputs/blank.txt";

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[rstest]
#[case(&[FILE1], &gen_bad_file(), ".* [(]os error 2[)]", true)]
#[case(&[FILE2], &gen_bad_file(), ".* [(]os error 2[)]", false)]
#[case(&["-"], "-", "Both input files cannot be STDIN (\"-\")", true)]
fn dies(
    #[case] args: &[&str],
    #[case] bad: &str,
    #[case] expected: &str,
    #[case] concat: bool,
) -> TestResult {
    let new_args = if concat {
        [args, &[bad]].concat()
    } else {
        [&[bad], args].concat()
    };
    let expected = format!("{bad} {expected}");
    Command::cargo_bin(PRG)?
        .args(&new_args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[rstest]
#[case(&[EMPTY, EMPTY], "tests/expected/empty_empty.out")]
#[case(&[FILE1, FILE1], "tests/expected/file1_file1.out")]
#[case(&[FILE1, FILE2], "tests/expected/file1_file2.out")]
#[case(&[FILE1, EMPTY], "tests/expected/file1_empty.out")]
#[case(&[EMPTY, FILE2], "tests/expected/empty_file2.out")]
#[case(&["-1", FILE1, FILE2], "tests/expected/file1_file2.1.out")]
#[case(&["-2", FILE1, FILE2], "tests/expected/file1_file2.2.out")]
#[case(&["-3", FILE1, FILE2], "tests/expected/file1_file2.3.out")]
#[case(&["-12", FILE1, FILE2], "tests/expected/file1_file2.12.out")]
#[case(&["-13", FILE1, FILE2], "tests/expected/file1_file2.13.out")]
#[case(&["-23", FILE1, FILE2], "tests/expected/file1_file2.23.out")]
#[case(&["-123", FILE1, FILE2], "tests/expected/file1_file2.123.out")]
#[case(&["-1", "-i", FILE1, FILE2], "tests/expected/file1_file2.1.i.out")]
#[case(&["-2", "-i", FILE1, FILE2], "tests/expected/file1_file2.2.i.out")]
#[case(&["-3", "-i", FILE1, FILE2], "tests/expected/file1_file2.3.i.out")]
#[case(&["-12", "-i", FILE1, FILE2], "tests/expected/file1_file2.12.i.out")]
#[case(&["-13", "-i", FILE1, FILE2], "tests/expected/file1_file2.13.i.out")]
#[case(&["-23", "-i", FILE1, FILE2], "tests/expected/file1_file2.23.i.out")]
#[case(&["-123", "-i", FILE1, FILE2], "tests/expected/file1_file2.123.i.out")]
#[case(&[FILE1, FILE2, "-d", ":"], "tests/expected/file1_file2.delim.out")]
#[case(&[FILE1, FILE2, "-1", "-d", ":"], "tests/expected/file1_file2.1.delim.out")]
#[case(&[FILE1, FILE2, "-2", "-d", ":"], "tests/expected/file1_file2.2.delim.out")]
#[case(&[FILE1, FILE2, "-3", "-d", ":"], "tests/expected/file1_file2.3.delim.out")]
#[case(&[FILE1, FILE2, "-12", "-d", ":"], "tests/expected/file1_file2.12.delim.out")]
#[case(&[FILE1, FILE2, "-13", "-d", ":"], "tests/expected/file1_file2.13.delim.out")]
#[case(&[FILE1, FILE2, "-23", "-d", ":"], "tests/expected/file1_file2.23.delim.out")]
#[case(&[FILE1, FILE2, "-123", "-d", ":"], "tests/expected/file1_file2.123.delim.out")]
#[case(&[BLANK, FILE1], "tests/expected/blank_file1.out")]
fn run(#[case] args: &[&str], #[case] expected: &str) -> TestResult {
    let expected = fs::read_to_string(expected)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[rstest]
#[case(&["-123", "-i", "-", FILE2], FILE1, "tests/expected/file1_file2.123.i.out")]
#[case(&["-123", "-i", "-", FILE1], FILE2, "tests/expected/file1_file2.123.i.out")]
fn run_stdin(
    #[case] args: &[&str],
    #[case] input_file: &str,
    #[case] expected: &str,
) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
