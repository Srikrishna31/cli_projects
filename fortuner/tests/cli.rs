use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use utils::{gen_bad_file, random_string, TestResult};

const PRG: &str = "fortuner";
const FORTUNE_DIR: &str = "tests/inputs";
const EMPTY_DIR: &str = "tests/inputs/empty";
const JOKES: &str = "tests/inputs/jokes";

const LITERATURE: &str = "tests/inputs/literature";
const QUOTES: &str = "tests/inputs/quotes";

#[test]
fn dies_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", &bad);
    Command::cargo_bin(PRG)?
        .args(&[LITERATURE, &bad])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn dies_bad_seed() -> TestResult {
    let bad = random_string(None);
    let expected = format!("\"{}\" not a valid integer", &bad);
    Command::cargo_bin(PRG)?
        .args(&[LITERATURE, "--seed", &bad])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

// The line endings have to be changed for Windows and Linux. Currently they
// are set for Windows.
#[rstest]
#[case(&[EMPTY_DIR], "No fortunes found\n")]
#[case(&[QUOTES, "-s", "1"], "You can observe a lot just by watching.\r\n-- Yogi Berra\n")]
#[case(&[JOKES, "-s", "1"], "Q: What happens when frogs park illegally?\r\nA: They get toad.\n")]
#[case(&[FORTUNE_DIR, "-s", "10"], "Q: Why did the fungus and the alga marry?\r\nA: Because they took a lichen to each other!\n")]
fn run(#[case] args: &[&str], #[case] expected: &'static str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[rstest]
#[case(&["--pattern", "Yogi Berra", FORTUNE_DIR], "tests/expected/berra_cap.out", "tests/expected/berra_cap.err")]
#[case(&["-m", "Mark Twain", FORTUNE_DIR], "tests/expected/twain_cap.out", "tests/expected/twain_cap.err")]
#[case(&["--pattern", "yogi berra", FORTUNE_DIR], "tests/expected/berra_lower.out", "tests/expected/berra_lower.err")]
#[case(&["-m", "will twain", FORTUNE_DIR], "tests/expected/twain_lower.out", "tests/expected/twain_lower.err")]
#[case(&["--insensitive", "--pattern", "yogi berra", FORTUNE_DIR], "tests/expected/berra_lower_i.out", "tests/expected/berra_lower_i.err")]
#[case(&["-i", "-m", "mark twain", FORTUNE_DIR], "tests/expected/twain_lower_i.out", "tests/expected/twain_lower_i.err")]
fn run_outfiles(
    #[case] args: &[&str],
    #[case] out_file: &str,
    #[case] err_file: &str,
) -> TestResult {
    let out = fs::read_to_string(out_file)?;
    let err = fs::read_to_string(err_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(out)
        .stderr(err);

    Ok(())
}
