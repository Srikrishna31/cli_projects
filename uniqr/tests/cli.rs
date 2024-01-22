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
#[case(&[ONE.input], ONE.out)]
#[case(&[TWO.input], TWO.out)]
#[case(&[THREE.input], THREE.out)]
#[case(&[SKIP.input], SKIP.out)]
#[case(&[T1.input], T1.out)]
#[case(&[T2.input], T2.out)]
#[case(&[T3.input], T3.out)]
#[case(&[T4.input], T4.out)]
#[case(&[T5.input], T5.out)]
#[case(&[T6.input], T6.out)]
#[case(&[ONE.input, "-c"], ONE.out_count)]
#[case(&[TWO.input, "-c"], TWO.out_count)]
#[case(&[THREE.input, "-c"], THREE.out_count)]
#[case(&[SKIP.input, "-c"], SKIP.out_count)]
#[case(&[T1.input, "-c"], T1.out_count)]
#[case(&[T2.input, "-c"], T2.out_count)]
#[case(&[T3.input, "-c"], T3.out_count)]
#[case(&[T4.input, "-c"], T4.out_count)]
#[case(&[T5.input, "-c"], T5.out_count)]
#[case(&[T6.input, "-c"], T6.out_count)]
fn run(#[case] args: &[&str], #[case] expected_out: &str) -> TestResult {
    let expected = fs::read_to_string(expected_out)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[rstest]
#[case(&EMPTY, &[], EMPTY.out_count)]
#[case(&ONE, &[], ONE.out_count)]
#[case(&TWO, &[], TWO.out_count)]
#[case(&THREE, &[], THREE.out_count)]
#[case(&SKIP, &[], SKIP.out_count)]
#[case(&T1, &[], T1.out_count)]
#[case(&T2, &[], T2.out_count)]
#[case(&T3, &[], T3.out_count)]
#[case(&T4, &[], T4.out_count)]
#[case(&T5, &[], T5.out_count)]
#[case(&T6, &[], T6.out_count)]
#[case(&EMPTY, &["-c"], EMPTY.out_count)]
#[case(&ONE, &["-c"], ONE.out_count)]
#[case(&TWO, &["-c"], TWO.out_count)]
#[case(&THREE, &["-c"], THREE.out_count)]
#[case(&SKIP, &["-c"], SKIP.out_count)]
#[case(&T1, &["-c"], T1.out_count)]
#[case(&T2, &["-c"], T2.out_count)]
#[case(&T3, &["-c"], T3.out_count)]
#[case(&T4, &["-c"], T4.out_count)]
#[case(&T5, &["-c"], T5.out_count)]
#[case(&T6, &["-c"], T6.out_count)]
fn run_stdin(#[case] test: &Test, #[case] args: &[&str], #[case] expected_out: &str) -> TestResult {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(expected_out)?;
    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[rstest]
#[case(&[EMPTY.input], EMPTY.out)]
#[case(&[EMPTY.input, "-c"], EMPTY.out_count)]
#[case(&[ONE.input], ONE.out)]
#[case(&[TWO.input], TWO.out)]
#[case(&[THREE.input], THREE.out)]
#[case(&[SKIP.input], SKIP.out)]
#[case(&[T1.input], T1.out)]
#[case(&[T2.input], T2.out)]
#[case(&[T3.input], T3.out)]
#[case(&[T4.input], T4.out)]
#[case(&[T5.input], T5.out)]
#[case(&[T6.input], T6.out)]
#[case(&[ONE.input, "-c"], ONE.out_count)]
#[case(&[TWO.input, "-c"], TWO.out_count)]
#[case(&[THREE.input, "-c"], THREE.out_count)]
#[case(&[SKIP.input, "-c"], SKIP.out_count)]
#[case(&[T1.input, "-c"], T1.out_count)]
#[case(&[T2.input, "-c"], T2.out_count)]
#[case(&[T3.input, "-c"], T3.out_count)]
#[case(&[T4.input, "-c"], T4.out_count)]
#[case(&[T5.input, "-c"], T5.out_count)]
#[case(&[T6.input, "-c"], T6.out_count)]
fn run_outfile(#[case] args: &[&str], #[case] expected_out: &str) -> TestResult {
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();
    let expected = fs::read_to_string(expected_out)?;
    let mut args_copy = args.to_owned();
    args_copy.push(outpath);
    Command::cargo_bin(PRG)?
        .args(args_copy)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[rstest]
#[case(&EMPTY, &[], EMPTY.out_count)]
#[case(&ONE, &[], ONE.out_count)]
#[case(&TWO, &[], TWO.out_count)]
#[case(&THREE, &[], THREE.out_count)]
#[case(&SKIP, &[], SKIP.out_count)]
#[case(&T1, &[], T1.out_count)]
#[case(&T2, &[], T2.out_count)]
#[case(&T3, &[], T3.out_count)]
#[case(&T4, &[], T4.out_count)]
#[case(&T5, &[], T5.out_count)]
#[case(&T6, &[], T6.out_count)]
#[case(&EMPTY, &["-c"], EMPTY.out_count)]
#[case(&ONE, &["-c"], ONE.out_count)]
#[case(&TWO, &["-c"], TWO.out_count)]
#[case(&THREE, &["-c"], THREE.out_count)]
#[case(&SKIP, &["-c"], SKIP.out_count)]
#[case(&T1, &["-c"], T1.out_count)]
#[case(&T2, &["-c"], T2.out_count)]
#[case(&T3, &["-c"], T3.out_count)]
#[case(&T4, &["-c"], T4.out_count)]
#[case(&T5, &["-c"], T5.out_count)]
#[case(&T6, &["-c"], T6.out_count)]
fn run_stdin_outfile(
    #[case] test: &Test,
    #[case] args: &[&str],
    #[case] expected_out: &str,
) -> TestResult {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(expected_out)?;
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();
    let mut args_copy = args.to_owned();
    args_copy.push(outpath);
    Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args_copy)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
