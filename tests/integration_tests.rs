use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_help() {
    let mut cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();

    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_version() {
    let mut cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();

    cmd.arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_file_does_not_exist() {
    let mut cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();

    cmd.arg("test_data/does_not_exist.txt")
        .assert()
        .failure()
        .stderr("rustcat: test_data/does_not_exist.txt: No such file or directory\n");
}

#[test]
fn test_directory_does_not_exist()
{
    let mut cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();

    cmd.arg("test_data/does_not_exist")
        .assert()
        .failure()
        .stderr("rustcat: test_data/does_not_exist: No such file or directory\n");
}
#[test]
fn test_file_exists() {
    let mut cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();

    cmd.arg("test_data/test1.txt")
        .assert()
        .success()
        .stdout("Hello World\nHi There\nHello There\n");
}


/* 
const TEXT: &str = "Hello, world!\nHello, world!\nHello, world!\n";
const LINES: usize = 3;
const WORDS: usize = 6;
const CHARS: usize = 42;

#[test]
fn test_all() {
    let test_file = assert_fs::NamedTempFile::new("test.txt").unwrap();
    test_file.write_str(TEXT).unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("rustwc").unwrap();

    let expected = format!("{:8}{:8}{:8}", LINES, WORDS, CHARS);
    cmd.arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_stdin() {
    let mut cmd = assert_cmd::Command::cargo_bin("rustwc").unwrap();

    let expected = format!("{:8}{:8}{:8}", LINES, WORDS, CHARS);
    cmd.write_stdin(TEXT)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_lines() {
    let test_file = assert_fs::NamedTempFile::new("test.txt").unwrap();
    test_file.write_str(TEXT).unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("rustwc").unwrap();

    let expected = format!("{:8}", LINES);
    cmd.arg("-l")
        .arg(test_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}
*/