use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::error::Error;
use std::{fs, path::Path};
use sys_info::os_type;
use utils::{gen_bad_file, random_string, TestResult};

const PRG: &str = "grepr";
const BUSTLE: &str = "tests/inputs/bustle.txt";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const NOBODY: &str = "tests/inputs/nobody.txt";
const INPUTS_DIR: &str = "tests/inputs";

#[rstest]
#[case(&[], "Usage")]
#[case(&["*foo", FOX], "Invalid pattern \"*foo\"")]
fn dies(#[case] args: &[&str], #[case] expected: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn warns_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);

    Command::cargo_bin(PRG)?
        .args(&["foo", &bad])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[rstest]
#[case(&["foo", EMPTY], "tests/expected/empty.foo")]
#[case(&["", FOX], "tests/expected/empty_regex.fox.txt")]
#[case(&["The", BUSTLE], "tests/expected/bustle.txt.the.capitalized")]
#[case(&["the", BUSTLE], "tests/expected/bustle.txt.the.lowercase")]
#[case(&["--insensitive", "the", BUSTLE], "tests/expected/bustle.txt.the.lowercase.insensitive")]
#[case(&["nobody", NOBODY], "tests/expected/nobody.txt")]
#[case(&["-i", "nobody", NOBODY], "tests/expected/nobody.txt.insensitive")]
#[case(&["The", BUSTLE, EMPTY, FOX, NOBODY], "tests/expected/all.the.capitalized")]
#[case(&["-i", "the", BUSTLE, EMPTY, FOX, NOBODY], "tests/expected/all.the.lowercase.insensitive")]
#[case(&["--recursive", "dog", INPUTS_DIR], "tests/expected/dog.insensitive")]
#[case(&["-ri", "then", INPUTS_DIR], "tests/expected/the.recursive.insensitive")]
#[case(&["--count", "The", BUSTLE], "tests/expected/bustle.txt.the.capitalized.count")]
#[case(&["--count", "the", BUSTLE], "tests/expected/bustle.txt.the.lowercase.count")]
#[case(&["-ci", "the", BUSTLE], "tests/expected/bustle.txt.the.lowercase.insensitive.count")]
#[case(&["-c", "nobody", NOBODY], "tests/expected/nobody.txt.count")]
#[case(&["-ci", "nobody", NOBODY], "tests/expected/nobody.txt.insensitive.count")]
#[case(&["-c", "The", BUSTLE, EMPTY, FOX, NOBODY], "tests/expected/all.the.capitalized.count")]
#[case(&["-ic", "the", BUSTLE, EMPTY, FOX, NOBODY], "tests/expected/all.the.lowercase.insensitive.count")]
fn run(#[case] args: &[&str], #[case] expected_file: &str) -> TestResult {
    let windows_file = format!("{expected_file}.windows");
    let expected_file = if os_type().unwrap() == "Windows" && Path::new(&windows_file).is_file() {
        &windows_file
    } else {
        expected_file
    };
    let expected = fs::read_to_string(&expected_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .stdout(expected);

    Ok(())
}

#[test]
fn warns_dir_not_recursive() -> TestResult {
    let stdout = "tests/inputs/fox.txt:\
        The quick brown fox jumps over the lazy dog.";

    Command::cargo_bin(PRG)?
        .args(&["fox", INPUTS_DIR, FOX])
        .assert()
        .stderr(predicate::str::contains("tests/inputs is a directory"))
        .stdout(predicate::str::contains(stdout));

    Ok(())
}

#[rstest]
#[case(&[BUSTLE], &[], "tests/expected/bustle.txt.the.capitalized")]
#[case(&[BUSTLE, EMPTY, FOX, NOBODY], &["-ci", "the", "-"], "tests/expected/the.recursive.insensitive.count.stdin")]
fn stdin(#[case] files: &[&str], #[case] args: &[&str], #[case] expected: &str) -> TestResult {
    let input = files.iter().try_fold(
        "".to_string(),
        |mut acc, f| -> Result<String, Box<dyn Error>> {
            fs::read_to_string(f)
                .map(|s| {
                    acc.push_str(&s);
                    acc
                })
                .map_err(Box::from)
        },
    )?;

    let expected = fs::read_to_string(expected)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .stdout(expected);

    Ok(())
}
