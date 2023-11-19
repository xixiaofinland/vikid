use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const CMD: &str = "vikid";

#[test]
fn run_without_param() -> TestResult {
    Command::cargo_bin(CMD)?
        .assert()
        .success()
        .stdout(predicate::str::contains("without douban"));
    Ok(())
}

#[test]
fn run_with_param() -> TestResult {
    Command::cargo_bin(CMD)?
        .args(&["--douban"])
        .assert()
        .success()
        .stdout(predicate::str::contains("with douban"));
    Ok(())
}

#[test]
fn run_with_short_param() -> TestResult {
    Command::cargo_bin(CMD)?
        .args(&["-d"])
        .assert()
        .success()
        .stdout(predicate::str::contains("with douban"));
    Ok(())
}
