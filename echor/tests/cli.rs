use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("asdf").assert()
        .success()
        .stdout(predicate::str::contains("asdf"));
}

#[test]
fn runs_multi() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("asdf").arg("woah").assert()
        .success()
        .stdout(predicate::str::contains("asdf woah"));
}

#[test]
fn hello1_old() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

// time to write better tests using a type alias that allows us to write tests that can fail instead of panic
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args2() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?; //? unpacks Ok or propagates error
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}


fn run(args : &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}


#[test]
fn hello1() -> TestResult {
    // no need for this - we got the fancy helper :)
    // let expected = fs::read_to_string("tests/expected/hello2.txt")?;
    // let mut cmd = Command::cargo_bin("echor")?;
    // cmd.args(vec!["Hello", "there"])
    //     .assert()
    //     .success()
    //     .stdout(expected);
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello",  "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["-n", "Hello  there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n",  "Hello", "there"], "tests/expected/hello2.n.txt")
}