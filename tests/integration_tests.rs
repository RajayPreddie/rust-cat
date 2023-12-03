use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;


fn compare_rustcat_and_cat(test_args: &Vec<&str>) {
    let test_files = fs::read_dir("test_data").unwrap();
    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    for file in test_files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        println!("Testing file: {}", file_name);
        let file_path = format!("test_data/{}", file_name);
        for arg in test_args {
            let mut rustcat_cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();
            let mut cat_cmd = Command::new("cat");
            let rustcat_output = rustcat_cmd.arg(arg).arg(file_path.clone()).unwrap().stdout;
            let cat_output = cat_cmd.arg(arg).arg(file_path.clone()).unwrap().stdout;

            let rustcat_output_str = String::from_utf8_lossy(&rustcat_output);
            let cat_output_str = String::from_utf8_lossy(&cat_output);
            assert_eq!(
                rustcat_output_str, cat_output_str,
                "Mismatch in file {:?}",
                file_path
            );
        }
    }
}

fn compare_rustcat_and_cat_for_linux(rustcat_args: &Vec<&str>, cat_args: &Vec<&str>) {
    let test_files = fs::read_dir("test_data").unwrap();
    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    for file in test_files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        println!("Testing file: {}", file_name);
        let file_path = format!("test_data/{}", file_name);
      
            let mut rustcat_cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();
            let mut cat_cmd = Command::new("cat");
            let rustcat_output = rustcat_cmd
                .args(rustcat_args)
                .arg(file_path.clone())
                .unwrap()
                .stdout;
            let cat_output = cat_cmd.args(cat_args).arg(file_path.clone()).unwrap().stdout;

            let rustcat_output_str = String::from_utf8_lossy(&rustcat_output);
            let cat_output_str = String::from_utf8_lossy(&cat_output);
            assert_eq!(
                rustcat_output_str, cat_output_str,
                "Mismatch in file {:?}",
                file_path
            );
        }
    }

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
fn test_directory_does_not_exist() {
    let mut rustcat_cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();

    rustcat_cmd
        .arg("hello")
        .assert()
        .failure()
        .stderr("rustcat: hello: No such file or directory\n");
}
#[test]
fn test_file_exists() {
    let mut rustcat_cmd = assert_cmd::Command::cargo_bin("rustcat").unwrap();

    rustcat_cmd
        .arg("hello.txt")
        .assert()
        .failure()
        .stderr("rustcat: hello.txt: No such file or directory\n");
}

#[test]
fn test_show_nonprinting() {
    let test_args = vec!["-v"];

    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_nonprinting_and_tabs() {
    let test_args = vec!["-t"];

    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_line_numbers() {
    let test_args = vec!["-n"];

    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat(&test_args);
}
#[test]
fn test_show_non_blank_line() {
    let test_args = vec!["-b"];

    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_squeeze_blank() {
    let test_args = vec!["-s"];

    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_nonprinting_and_ends() {
    let test_args = vec!["-e"];

    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_nonprinting_and_tabs_for_linux() {
    let rustcat_args = vec![ "-v", "-T"];
    let cat_args = vec!["-t"];
    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}

#[test]
fn test_show_nonprinting_and_ends_for_linux() {
    let rustcat_args = vec![ "-v","-E"];
    let cat_args = vec!["-e"];
    // vec![ "-v","-t","-n", "-b",  "-e", "-s" ];
    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}

#[test]
fn test_show_all_for_linux() {
    let rustcat_args = vec![ "-A"];
    let cat_args = vec!["-e", "-t", "-v"];
    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}


#[test]
fn test_show_all_for_linux_multiple_files() {
    let rustcat_args = vec![ "-A", "test_data/test1.txt"];
    let cat_args = vec!["-e", "-t", "-v", "test_data/test1.txt"];
    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}

// TODO: add additional test files and test with different combinations of args
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
