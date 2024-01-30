use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use utils::{gen_bad_file, random_string, TestResult};

const PRG: &str = "cutr";
const CSV: &str = "tests/inputs/movies1.csv";
const TSV: &str = "tests/inputs/movies1.tsv";

const BOOKS: &str = "tests/inputs/books.tsv";

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args(&["-f", "1", "-d", ",", CSV, &bad, TSV])
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[rstest]
#[case(&[CSV], "Must have --fields, --bytes, or --chars")]
#[case(&[CSV, "-f", "1", "-d", ""], "--delim \"\" must be a single byte")]
#[case(&[CSV, "-f", "1", "-d", ",,"], "--delim \",,\" must be a single byte")]
#[case(&[CSV, "-c", "1", "-f", "1", "-b", "1"], "")]
#[case(&[CSV, "-f", "1", "-b", "1"], "")]
#[case(&[CSV, "-c", "1", "-f", "1"], "")]
#[case(&[CSV, "-c", "1", "-b", "1"], "")]
fn dies(#[case] args: &[&str], #[case] expected: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[rstest]
#[case(&[CSV, "-f", &random_string(None)])]
#[case(&[CSV, "-b", &random_string(None)])]
#[case(&[CSV, "-c", &random_string(None)])]
fn dies_bad_string(#[case] args: &[&str]) -> TestResult {
    let bad = *args.last().unwrap();

    dies(args, &format!("illegal list value: \"{}\"", bad))?;

    Ok(())
}

#[rstest]
#[case(&[TSV, "-f", "1"], "tests/expected/movies1.tsv.f1.out")]
#[case(&[TSV, "-f", "2"],"tests/expected/movies1.tsv.f2.out")]
#[case(&[TSV, "-f", "3"], "tests/expected/movies1.tsv.f3.out")]
#[case(&[TSV, "-f", "1-2"], "tests/expected/movies1.tsv.f1-2.out")]
#[case(&[TSV, "-f", "2-3"], "tests/expected/movies1.tsv.f2-3.out")]
#[case(&[TSV, "-f", "1-3"], "tests/expected/movies1.tsv.f1-3.out")]
#[case(&[CSV, "-f", "1", "-d", ","], "tests/expected/movies1.csv.f1.dcomma.out")]
#[case(&[CSV, "-f", "2", "-d", ","], "tests/expected/movies1.csv.f2.dcomma.out")]
#[case(&[CSV, "-f", "3", "-d", ","], "tests/expected/movies1.csv.f3.dcomma.out")]
#[case(&[CSV, "-f", "1-2", "-d", ","], "tests/expected/movies1.csv.f1-2.dcomma.out")]
#[case(&[CSV, "-f", "2-3", "-d", ","], "tests/expected/movies1.csv.f2-3.dcomma.out")]
#[case(&[CSV, "-f", "1-3", "-d", ","], "tests/expected/movies1.csv.f1-3.dcomma.out")]
#[case(&[TSV, "-b", "1"], "tests/expected/movies1.tsv.b1.out")]
#[case(&[TSV, "-b", "2"], "tests/expected/movies1.tsv.b2.out")]
#[case(&[TSV, "-b", "1-2"], "tests/expected/movies1.tsv.b1-2.out")]
#[case(&[TSV, "-b", "2-3"], "tests/expected/movies1.tsv.b2-3.out")]
#[case(&[TSV, "-c", "1"], "tests/expected/movies1.tsv.c1.out")]
#[case(&[TSV, "-c", "2"], "tests/expected/movies1.tsv.c2.out")]
#[case(&[TSV, "-c", "8"], "tests/expected/movies1.tsv.c8.out")]
#[case(&[TSV, "-c", "1-2"], "tests/expected/movies1.tsv.c1-2.out")]
#[case(&[TSV, "-c", "2-3"], "tests/expected/movies1.tsv.c2-3.out")]
#[case(&[BOOKS, "-c", "1,1"], "tests/expected/books.c1,1.out")]
#[case(&[TSV, "-c", "1-8"], "tests/expected/movies1.tsv.c1-8.out")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    println!("expected {expected_file}");
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[rstest]
#[case(&[TSV, "-b", "8"], "tests/expected/movies1.tsv.b8.out")]
#[case(&[TSV, "-b", "1-8"], "tests/expected/movies1.tsv.b1-8.out")]
fn run_lossy(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    let contents = fs::read(expected_file)?;
    let expected = String::from_utf8_lossy(&contents);
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));

    Ok(())
}
