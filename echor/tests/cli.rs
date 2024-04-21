use std::fs;

use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn single_arg() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn multiple_args() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn single_arg_no_newline() -> Result<()> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt") // n.b. 2 spaces
}

#[test]
fn multiple_args_no_newline() -> Result<()> {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
