use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use rstest::rstest;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult{
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[rstest]
#[case(&["Hello there"], "tests/expected/hello1.txt")]
#[case(&["Hello", "there"], "tests/expected/hello2.txt")]
#[case(&["Hello   there", "-n"], "tests/expected/hello1.n.txt")]
#[case(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
