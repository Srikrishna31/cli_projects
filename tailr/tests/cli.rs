use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use utils::{gen_bad_file, random_string, TestResult};

const PRG: &str = "tailr";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const TEN: &str = "tests/inputs/ten.txt";

#[test]
fn dies_no_args() {
    Command::cargo_bin(PRG)
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[rstest]
#[case(&[EMPTY, "-c", ], &random_string(None), "illegal byte count -- ")]
#[case(&[EMPTY, "-n", ], &random_string(None), "illegal line count -- ")]
fn dies(#[case] args: &[&str], #[case] bad: &str, #[case] expected_message: &str) -> TestResult {
    let args_new = [args, &[bad]].concat();
    let expected = format!("{}{}", expected_message, bad);
    Command::cargo_bin(PRG)?
        .args(args_new)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args(&[ONE, &bad, TWO])
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[rstest]
#[case(&[EMPTY], "tests/expected/empty.txt.out")]
#[case(&[EMPTY, "-n", "0"], "tests/expected/empty.txt.n0.out")]
#[case(&[EMPTY, "-n", "1"], "tests/expected/empty.txt.n1.out")]
#[case(&[EMPTY, "-n=-1"], "tests/expected/empty.txt.n1.out")]
#[case(&[EMPTY, "-n", "3"], "tests/expected/empty.txt.n3.out")]
#[case(&[EMPTY, "-n=-3"], "tests/expected/empty.txt.n3.out")]
#[case(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")]
#[case(&[EMPTY, "-n", "200"], "tests/expected/empty.txt.n200.out")]
#[case(&[EMPTY, "-n=-200"], "tests/expected/empty.txt.n200.out")]
#[case(&[EMPTY, "-n=-4"], "tests/expected/empty.txt.n4.out")]
#[case(&[EMPTY, "-n", "+0"], "tests/expected/empty.txt.n+0.out")]
#[case(&[EMPTY, "-n", "+1"], "tests/expected/empty.txt.n+1.out")]
#[case(&[EMPTY, "-n", "+2"], "tests/expected/empty.txt.n+2.out")]
#[case(&[EMPTY, "-c", "3"], "tests/expected/empty.txt.c3.out")]
#[case(&[EMPTY, "-c=-3"], "tests/expected/empty.txt.c3.out")]
#[case(&[EMPTY, "-c", "8"], "tests/expected/empty.txt.c8.out")]
#[case(&[EMPTY, "-c=-8"], "tests/expected/empty.txt.c8.out")]
#[case(&[EMPTY, "-c=-12"], "tests/expected/empty.txt.c12.out")]
#[case(&[EMPTY, "-c", "200"], "tests/expected/empty.txt.c200.out")]
#[case(&[EMPTY, "-c=-200"], "tests/expected/empty.txt.c200.out")]
#[case(&[EMPTY, "-c", "+0"], "tests/expected/empty.txt.c+0.out")]
#[case(&[EMPTY, "-c", "+1"], "tests/expected/empty.txt.c+1.out")]
#[case(&[EMPTY, "-c", "+2"], "tests/expected/empty.txt.c+2.out")]
#[case(&[ONE], "tests/expected/one.txt.out")]
#[case(&[ONE, "-n", "0"], "tests/expected/one.txt.n0.out")]
#[case(&[ONE, "-n", "1"], "tests/expected/one.txt.n1.out")]
#[case(&[ONE, "-n=-1"], "tests/expected/one.txt.n1.out")]
#[case(&[ONE, "-n", "3"], "tests/expected/one.txt.n3.out")]
#[case(&[ONE, "-n=-3"], "tests/expected/one.txt.n3.out")]
#[case(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")]
#[case(&[ONE, "-n", "200"], "tests/expected/one.txt.n200.out")]
#[case(&[ONE, "-n=-200"], "tests/expected/one.txt.n200.out")]
#[case(&[ONE, "-n=-4"], "tests/expected/one.txt.n4.out")]
#[case(&[ONE, "-n", "+0"], "tests/expected/one.txt.n+0.out")]
#[case(&[ONE, "-n", "+1"], "tests/expected/one.txt.n+1.out")]
#[case(&[ONE, "-n", "+2"], "tests/expected/one.txt.n+2.out")]
#[case(&[ONE, "-c", "3"], "tests/expected/one.txt.c3.out")]
#[case(&[ONE, "-c=-3"], "tests/expected/one.txt.c3.out")]
#[case(&[ONE, "-c", "8"], "tests/expected/one.txt.c8.out")]
#[case(&[ONE, "-c=-8"], "tests/expected/one.txt.c8.out")]
#[case(&[ONE, "-c=-12"], "tests/expected/one.txt.c12.out")]
#[case(&[ONE, "-c", "200"], "tests/expected/one.txt.c200.out")]
#[case(&[ONE, "-c=-200"], "tests/expected/one.txt.c200.out")]
#[case(&[ONE, "-c", "+0"], "tests/expected/one.txt.c+0.out")]
#[case(&[ONE, "-c", "+1"], "tests/expected/one.txt.c+1.out")]
#[case(&[ONE, "-c", "+2"], "tests/expected/one.txt.c+2.out")]
#[case(&[TWO], "tests/expected/two.txt.out")]
#[case(&[TWO, "-n", "0"], "tests/expected/two.txt.n0.out")]
#[case(&[TWO, "-n", "1"], "tests/expected/two.txt.n1.out")]
#[case(&[TWO, "-n=-1"], "tests/expected/two.txt.n1.out")]
#[case(&[TWO, "-n", "3"], "tests/expected/two.txt.n3.out")]
#[case(&[TWO, "-n=-3"], "tests/expected/two.txt.n3.out")]
#[case(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")]
#[case(&[TWO, "-n", "200"], "tests/expected/two.txt.n200.out")]
#[case(&[TWO, "-n=-200"], "tests/expected/two.txt.n200.out")]
#[case(&[TWO, "-n=-4"], "tests/expected/two.txt.n4.out")]
#[case(&[TWO, "-n", "+0"], "tests/expected/two.txt.n+0.out")]
#[case(&[TWO, "-n", "+1"], "tests/expected/two.txt.n+1.out")]
#[case(&[TWO, "-n", "+2"], "tests/expected/two.txt.n+2.out")]
#[case(&[TWO, "-c", "3"], "tests/expected/two.txt.c3.out")]
#[case(&[TWO, "-c=-3"], "tests/expected/two.txt.c3.out")]
#[case(&[TWO, "-c", "8"], "tests/expected/two.txt.c8.out")]
#[case(&[TWO, "-c=-8"], "tests/expected/two.txt.c8.out")]
#[case(&[TWO, "-c=-12"], "tests/expected/two.txt.c12.out")]
#[case(&[TWO, "-c", "200"], "tests/expected/two.txt.c200.out")]
#[case(&[TWO, "-c=-200"], "tests/expected/two.txt.c200.out")]
#[case(&[TWO, "-c", "+0"], "tests/expected/two.txt.c+0.out")]
#[case(&[TWO, "-c", "+1"], "tests/expected/two.txt.c+1.out")]
#[case(&[TWO, "-c", "+2"], "tests/expected/two.txt.c+2.out")]
#[case(&[THREE], "tests/expected/three.txt.out")]
#[case(&[THREE, "-n", "0"], "tests/expected/three.txt.n0.out")]
#[case(&[THREE, "-n", "1"], "tests/expected/three.txt.n1.out")]
#[case(&[THREE, "-n=-1"], "tests/expected/three.txt.n1.out")]
#[case(&[THREE, "-n", "3"], "tests/expected/three.txt.n3.out")]
#[case(&[THREE, "-n=-3"], "tests/expected/three.txt.n3.out")]
#[case(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")]
#[case(&[THREE, "-n", "200"], "tests/expected/three.txt.n200.out")]
#[case(&[THREE, "-n=-200"], "tests/expected/three.txt.n200.out")]
#[case(&[THREE, "-n=-4"], "tests/expected/three.txt.n4.out")]
#[case(&[THREE, "-n", "+0"], "tests/expected/three.txt.n+0.out")]
#[case(&[THREE, "-n", "+1"], "tests/expected/three.txt.n+1.out")]
#[case(&[THREE, "-n", "+2"], "tests/expected/three.txt.n+2.out")]
#[case(&[THREE, "-c", "3"], "tests/expected/three.txt.c3.out")]
#[case(&[THREE, "-c=-3"], "tests/expected/three.txt.c3.out")]
#[case(&[THREE, "-c", "8"], "tests/expected/three.txt.c8.out")]
#[case(&[THREE, "-c=-8"], "tests/expected/three.txt.c8.out")]
#[case(&[THREE, "-c=-12"], "tests/expected/three.txt.c12.out")]
#[case(&[THREE, "-c", "200"], "tests/expected/three.txt.c200.out")]
#[case(&[THREE, "-c=-200"], "tests/expected/three.txt.c200.out")]
#[case(&[THREE, "-c", "+0"], "tests/expected/three.txt.c+0.out")]
#[case(&[THREE, "-c", "+1"], "tests/expected/three.txt.c+1.out")]
#[case(&[THREE, "-c", "+2"], "tests/expected/three.txt.c+2.out")]
#[case(&[TEN], "tests/expected/ten.txt.out")]
#[case(&[TEN, "-n", "0"], "tests/expected/ten.txt.n0.out")]
#[case(&[TEN, "-n", "1"], "tests/expected/ten.txt.n1.out")]
#[case(&[TEN, "-n=-1"], "tests/expected/ten.txt.n1.out")]
#[case(&[TEN, "-n", "3"], "tests/expected/ten.txt.n3.out")]
#[case(&[TEN, "-n=-3"], "tests/expected/ten.txt.n3.out")]
#[case(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")]
#[case(&[TEN, "-n", "200"], "tests/expected/ten.txt.n200.out")]
#[case(&[TEN, "-n=-200"], "tests/expected/ten.txt.n200.out")]
#[case(&[TEN, "-n=-4"], "tests/expected/ten.txt.n4.out")]
#[case(&[TEN, "-n", "+0"], "tests/expected/ten.txt.n+0.out")]
#[case(&[TEN, "-n", "+1"], "tests/expected/ten.txt.n+1.out")]
#[case(&[TEN, "-n", "+2"], "tests/expected/ten.txt.n+2.out")]
#[case(&[TEN, "-c", "3"], "tests/expected/ten.txt.c3.out")]
#[case(&[TEN, "-c=-3"], "tests/expected/ten.txt.c3.out")]
#[case(&[TEN, "-c", "8"], "tests/expected/ten.txt.c8.out")]
#[case(&[TEN, "-c=-8"], "tests/expected/ten.txt.c8.out")]
#[case(&[TEN, "-c=-12"], "tests/expected/ten.txt.c12.out")]
#[case(&[TEN, "-c", "200"], "tests/expected/ten.txt.c200.out")]
#[case(&[TEN, "-c=-200"], "tests/expected/ten.txt.c200.out")]
#[case(&[TEN, "-c", "+0"], "tests/expected/ten.txt.c+0.out")]
#[case(&[TEN, "-c", "+1"], "tests/expected/ten.txt.c+1.out")]
#[case(&[TEN, "-c", "+2"], "tests/expected/ten.txt.c+2.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO], "tests/expected/all.txt.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "0"], "tests/expected/all.txt.n0.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "1"], "tests/expected/all.txt.n1.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n=-1"], "tests/expected/all.txt.n1.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "3"], "tests/expected/all.txt.n3.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n=-3"], "tests/expected/all.txt.n3.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "4"], "tests/expected/all.txt.n4.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "200"], "tests/expected/all.txt.n200.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n=-200"], "tests/expected/all.txt.n200.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n=-4"], "tests/expected/all.txt.n4.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "+0"], "tests/expected/all.txt.n+0.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "+1"], "tests/expected/all.txt.n+1.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-n", "+2"], "tests/expected/all.txt.n+2.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "3"], "tests/expected/all.txt.c3.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "-3"], "tests/expected/all.txt.c3.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "8"], "tests/expected/all.txt.c8.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "-8"], "tests/expected/all.txt.c8.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "-12"], "tests/expected/all.txt.c12.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "200"], "tests/expected/all.txt.c200.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "-200"], "tests/expected/all.txt.c200.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "+0"], "tests/expected/all.txt.c+0.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "+1"], "tests/expected/all.txt.c+1.out")]
#[case(&[TEN, EMPTY, ONE, THREE, TWO, "-c", "+2"], "tests/expected/all.txt.c+2.out")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
