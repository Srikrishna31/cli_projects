use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use utils::{gen_bad_file, TestResult};

const PRG: &str = "lsr";
const HIDDEN: &str = "tests/inputs/.hidden";
const EMPTY: &str = "tests/inputs/empty.txt";
const BUSTLE: &str = "tests/inputs/bustle.txt";
const FOX: &str = "tests/inputs/fox.txt";

#[test]
fn bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: No such file or directory (os error 2)", &bad);
    Command::cargo_bin(PRG)?
        .args(&[&bad])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn no_args() -> TestResult {
    // Uses current directory by default
    Command::cargo_bin(PRG)?
        .assert()
        .success()
        .stdout(predicate::str::contains("Cargo.toml"));

    Ok(())
}

#[rstest]
#[case(EMPTY)]
#[case(BUSTLE)]
#[case(FOX)]
#[case(HIDDEN)]
fn run_short(#[case] arg: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .arg(arg)
        .assert()
        .success()
        .stdout(format!("{}\n", arg));

    Ok(())
}

#[rstest]
#[case(EMPTY, "-rw-r--r--", "0")]
#[case(BUSTLE, "-rw-r--r--", "193")]
#[case(FOX, "-rw-r--r--", "45")]
#[case(HIDDEN, "-rw-r--r--", "0")]
fn run_long(#[case] filename: &str, #[case] permissions: &str, #[case] size: &str) -> TestResult {
    let cmd = Command::cargo_bin(PRG)?
        .args(&["--long", filename])
        .assert()
        .success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let parts: Vec<_> = stdout.split_whitespace().collect();
    assert_eq!(parts[0], permissions);
    assert_eq!(parts[4], size);
    assert_eq!(parts.last().unwrap(), &filename);

    Ok(())
}

#[rstest]
#[case(&["tests/inputs"], &["tests/inputs/empty.txt",
                            "tests/inputs/bustle.txt",
                            "tests/inputs/fox.txt",
                            "tests/inputs/dir"])]
#[case(&["tests/inputs", "--all"], &["tests/inputs/empty.txt",
                                     "tests/inputs/bustle.txt",
                                     "tests/inputs/fox.txt",
                                     "tests/inputs/.hidden",
                                     "tests/inputs/dir"])]
#[case(&["tests/inputs/dir"], &["tests/inputs/dir/spiders.txt"])]
#[case(&["-a", "tests/inputs/dir"], &["tests/inputs/dir",
                                      "tests/inputs/dir/spiders.txt",
                                      "tests/inputs/dir/.gitkeep"])]
fn dir_short(#[case] args: &[&str], #[case] expected: &[&str]) -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    assert_eq!(lines.len(), expected.len());
    let _ = expected.iter().for_each(|e| assert!(lines.contains(e)));

    Ok(())
}

#[rstest]
#[case(&["-l", "tests/inputs"], &[
    ("tests/inputs/empty.txt", "-rw-r--r--", "0"),
    ("tests/inputs/bustle.txt", "-rw-r--r--", "193"),
    ("tests/inputs/fox.txt", "-rw-------", "45"),
    ("tests/inputs/dir", "drwxr-xr-x", "")])]
#[case(&["-la", "tests/inputs"], &[
    ("tests/inputs/empty.txt", "-rw-r--r--", "0"),
    ("tests/inputs/bustle.txt", "-rw-r--r--", "193"),
    ("tests/inputs/fox.txt", "-rw-------", "45"),
    ("tests/inputs/.hidden", "-rw-r--r--", "0"),
    ("tests/inputs/dir", "drwxr-xr-x", "")])]
#[case(&["--long", "tests/inputs/dir"], &[("tests/inputs/dir/spiders.txt", "-rw-r--r--", "45")])]
#[case(&["tests/inputs/dir", "--long", "--all"], &[("tests/inputs/dir/spiders.txt", "-rw-r--r--", "45"),
                                                   ("tests/inputs/dir/.gitkeep", "-rw-r--r--", "0")])]
fn dir_long(#[case] args: &[&str], #[case] expected: &[(&str, &str, &str)]) -> TestResult {
    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let stdout = String::from_utf8(cmd.get_output().stdout.clone())?;
    let lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    assert_eq!(lines.len(), expected.len());

    let check = lines.iter().fold(vec![], |mut acc, line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let filename = parts.last().unwrap().clone();
        let perms = parts.get(0).unwrap().clone();
        let size = match perms.chars().next() {
            Some('d') => "",
            _ => parts.get(4).unwrap().clone(),
        };
        acc.push((filename, perms, size));
        acc
    });

    let _ = expected.iter().for_each(|e| assert!(check.contains(e)));

    Ok(())
}
