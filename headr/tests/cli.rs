use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::{
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

#[rstest]
#[case(&[EMPTY], "tests/expected/empty.txt.out")]
#[case(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")]
#[case(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")]
#[case(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")]
#[case(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")]
#[case(&[ONE], "tests/expected/one.txt.out")]
#[case(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")]
#[case(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")]
#[case(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")]
#[case(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")]
#[case(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")]
#[case(&[TWO], "tests/expected/two.txt.out")]
#[case(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")]
#[case(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")]
#[case(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")]
#[case(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")]
#[case(&[THREE], "tests/expected/three.txt.out")]
#[case(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")]
#[case(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")]
#[case(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")]
#[case(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")]
#[case(&[TEN], "tests/expected/ten.txt.out")]
#[case(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")]
#[case(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")]
#[case(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")]
#[case(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}

#[rstest]
#[case(&[], ONE, "tests/expected/one.txt.out")]
#[case(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")]
#[case(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")]
#[case(&["-c", "1"], ONE, "tests/expected/one.txt.c1.out")]
#[case(&["-c", "2"], ONE, "tests/expected/one.txt.c2.out")]
#[case(&["-c", "4"], ONE, "tests/expected/one.txt.c4.out")]
#[case(&[], TWO, "tests/expected/two.txt.out")]
#[case(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")]
#[case(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")]
#[case(&["-c", "2"], TWO, "tests/expected/two.txt.c2.out")]
#[case(&["-c", "4"], TWO, "tests/expected/two.txt.c4.out")]
#[case(&[], THREE, "tests/expected/three.txt.out")]
#[case(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")]
#[case(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")]
#[case(&["-c", "2"], THREE, "tests/expected/three.txt.c2.out")]
#[case(&["-c", "4"], THREE, "tests/expected/three.txt.c4.out")]
#[case(&[], TEN, "tests/expected/TEN.txt.out")]
#[case(&["-n", "2"], TEN, "tests/expected/ten.txt.n2.out")]
#[case(&["-n", "4"], TEN, "tests/expected/ten.txt.n4.out")]
#[case(&["-c", "2"], TEN, "tests/expected/ten.txt.c2.out")]
#[case(&["-c", "4"], TEN, "tests/expected/ten.txt.c4.out")]
fn run_stdin(
    #[case] args: &[&str],
    #[case] input_file: &str,
    #[case] expected_file: &str,
) -> TestResult {
    // Extra work here due to lossy UTF
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;

    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .assert()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}
