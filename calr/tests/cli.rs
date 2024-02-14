use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use utils::TestResult;

const PRG: &str = "calr";

#[rstest]
#[case(&[], "0", "year \"{}\" not in the range 1-9999")]
#[case(&[], "10000", "year \"{}\" not in the range 1-9999")]
#[case(&[], "foo", "Invalid integer \"{}\"")]
#[case(&["-m", ], "0", "month \"0\" not in the range 1-12")]
#[case(&["-m"], "13", "month \"13\" not in the range 1-12")]
#[case(&["-m"], "foo", "Invalid month \"foo\"")]
#[case(&["-y", "-m", ], "1", "the argument '--year' cannot be used with '--month <MONTH>'")]
#[case(&["-y"], "2000", "the argument '--year' cannot be used with '[YEAR]'")]
fn dies(#[case] args: &[&str], #[case] bad: &str, #[case] expected: &str) -> TestResult {
    let new_args = [args, &[bad]].concat();
    Command::cargo_bin(PRG)?
        .args(&new_args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(str::replace(expected, "{}", bad)));
    Ok(())
}

#[rstest]
#[case("1", "January")]
#[case("2", "February")]
#[case("3", "March")]
#[case("4", "April")]
#[case("5", "May")]
#[case("6", "June")]
#[case("7", "July")]
#[case("8", "August")]
#[case("9", "September")]
#[case("10", "October")]
#[case("11", "November")]
#[case("12", "December")]
#[case("ja", "January")]
#[case("f", "February")]
#[case("mar", "March")]
#[case("ap", "April")]
#[case("may", "May")]
#[case("jun", "June")]
#[case("jul", "July")]
#[case("au", "August")]
#[case("s", "September")]
#[case("o", "October")]
#[case("n", "November")]
#[case("d", "December")]
fn test_month(#[case] month: &str, #[case] expected: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["-m", month])
        .assert()
        .success()
        .stdout(predicates::str::contains(expected.to_string()));
    Ok(())
}

#[rstest]
#[case(&["-m", "2", "2020"], "tests/expected/2-2020.txt")]
#[case(&["-m", "4", "2020"], "tests/expected/4-2020.txt")]
#[case(&["2020", "-m", "april"], "tests/expected/4-2020.txt")]
#[case(&["2020"], "tests/expected/2020.txt")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?.trim().to_string();
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn default_one_month() -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').collect();
    assert_eq!(lines.len(), 9);
    assert_eq!(lines[0].len(), 22);
    Ok(())
}

#[test]
fn year() -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.args(&["-y"]).assert().success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<_> = stdout.split('\n').collect();
    assert_eq!(lines.len(), 37);
    Ok(())
}
