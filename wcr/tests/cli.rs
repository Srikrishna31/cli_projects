use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use utils::{gen_bad_file, TestResult};

const PRG: &str = "wcr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";

#[test]
fn dies_chars_and_bytes() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["-m", "-c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the argument '--chars' cannot be used with '--bytes'",
        ));

    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = &gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[rstest]
#[case(&[EMPTY], "tests/expected/empty.txt.out")]
#[case(&[FOX], "tests/expected/fox.txt.out")]
#[case(&["--bytes", FOX], "tests/expected/fox.txt.c.out")]
#[case(&["--chars", FOX], "tests/expected/fox.txt.m.out")]
#[case(&["--lines", FOX], "tests/expected/fox.txt.l.out")]
#[case(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")]
#[case(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")]
#[case(&["-l", "-c", FOX], "tests/expected/fox.txt.cl.out")]
#[case(&[ATLAMAL], "tests/expected/atlamal.txt.out")]
#[case(&["--bytes", ATLAMAL], "tests/expected/atlamal.txt.c.out")]
#[case(&["--chars", ATLAMAL], "tests/expected/atlamal.txt.m.out")]
#[case(&["--lines", ATLAMAL], "tests/expected/atlamal.txt.l.out")]
#[case(&["-w", "-c", ATLAMAL], "tests/expected/atlamal.txt.wc.out")]
#[case(&["-w", "-l", ATLAMAL], "tests/expected/atlamal.txt.wl.out")]
#[case(&["-l", "-c", ATLAMAL], "tests/expected/atlamal.txt.cl.out")]
#[case(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.txt.out")]
#[case(&["--bytes", EMPTY, FOX, ATLAMAL], "tests/expected/all.txt.c.out")]
#[case(&["--chars", EMPTY, FOX, ATLAMAL], "tests/expected/all.txt.m.out")]
#[case(&["--lines", EMPTY, FOX, ATLAMAL], "tests/expected/all.txt.l.out")]
#[case(&["-w", "-c", EMPTY, FOX, ATLAMAL], "tests/expected/all.txt.wc.out")]
#[case(&["-w", "-l", EMPTY, FOX, ATLAMAL], "tests/expected/all.txt.wl.out")]
#[case(&["-l", "-c", EMPTY, FOX, ATLAMAL], "tests/expected/all.txt.cl.out")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn altamal_stdin() -> TestResult {
    let input = fs::read_to_string(ATLAMAL)?;
    let expected = fs::read_to_string("tests/expected/atlamal.txt.stdin.out")?;
    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .assert()
        .stdout(expected);

    Ok(())
}
