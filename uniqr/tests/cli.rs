use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use tempfile::NamedTempFile;
use utils::{gen_bad_file, TestResult};

struct Test {
    input: &'static str,
    out: &'static str,
    out_count: &'static str,
}

const PRG: &str = "uniqr";

const EMPTY: Test = Test {
    input: "tests/inputs/empty.txt",
    out: "tests/expected/empty.txt.out",
    out_count: "tests/expected/empty.txt.c.out",
};

const ONE: Test = Test {
    input: "tests/inputs/one.txt",
    out: "tests/expected/one.txt.out",
    out_count: "tests/expected/one.txt.c.out",
};

const TWO: Test = Test {
    input: "tests/inputs/two.txt",
    out: "tests/expected/two.txt.out",
    out_count: "tests/expected/two.txt.c.out",
};

const THREE: Test = Test {
    input: "tests/inputs/three.txt",
    out: "tests/expected/three.txt.out",
    out_count: "tests/expected/three.txt.c.out",
};

const SKIP: Test = Test {
    input: "tests/inputs/skip.txt",
    out: "tests/expected/skip.txt.out",
    out_count: "tests/expected/skip.txt.c.out",
};

const T1: Test = Test {
    input: "tests/inputs/t1.txt",
    out: "tests/expected/t1.txt.out",
    out_count: "tests/expected/t1.txt.c.out",
};

const T2: Test = Test {
    input: "tests/inputs/t2.txt",
    out: "tests/expected/t2.txt.out",
    out_count: "tests/expected/t2.txt.c.out",
};

const T3: Test = Test {
    input: "tests/inputs/t3.txt",
    out: "tests/expected/t3.txt.out",
    out_count: "tests/expected/t3.txt.c.out",
};

const T4: Test = Test {
    input: "tests/inputs/t4.txt",
    out: "tests/expected/t4.txt.out",
    out_count: "tests/expected/t4.txt.c.out",
};

const T5: Test = Test {
    input: "tests/inputs/t5.txt",
    out: "tests/expected/t5.txt.out",
    out_count: "tests/expected/t5.txt.c.out",
};

const T6: Test = Test {
    input: "tests/inputs/t6.txt",
    out: "tests/expected/t6.txt.out",
    out_count: "tests/expected/t6.txt.c.out",
};

#[test]
fn dies_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", &bad);
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[rstest]
#[case(&[EMPTY.input], EMPTY.out)]
#[case(&[EMPTY.input, "-c"], EMPTY.out_count)]
fn run(#[case] args: &[&str], #[case] out: &str) -> TestResult {
    let expected = fs::read_to_string(out)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
