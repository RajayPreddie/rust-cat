use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::env;
use std::fs::{self};
use std::process::Command;

fn compare_rustcat_and_cat(test_args: &Vec<&str>) {
    let test_files = fs::read_dir("test_data").unwrap();

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
        let cat_output = cat_cmd
            .args(cat_args)
            .arg(file_path.clone())
            .unwrap()
            .stdout;

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

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_nonprinting_and_tabs() {
    let test_args = vec!["-t"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_line_numbers() {
    let test_args = vec!["-n"];

    compare_rustcat_and_cat(&test_args);
}
#[test]
fn test_show_non_blank_line() {
    let test_args = vec!["-b"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_squeeze_blank() {
    let test_args = vec!["-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_nonprinting_and_ends() {
    let test_args = vec!["-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_show_nonprinting_and_tabs_for_linux() {
    let rustcat_args = vec!["-v", "-T"];
    let cat_args = vec!["-t"];

    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}

#[test]
fn test_show_nonprinting_and_ends_for_linux() {
    let rustcat_args = vec!["-v", "-E"];
    let cat_args = vec!["-e"];

    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}

#[test]
fn test_show_all_for_linux() {
    let rustcat_args = vec!["-A"];
    let cat_args = vec!["-e", "-t", "-v"];
    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}

#[test]
fn test_show_all_for_linux_multiple_files() {
    let rustcat_args = vec!["-A", "test_data/large_test_1.txt"];
    let cat_args = vec!["-e", "-t", "-v", "test_data/large_test_1.txt"];
    compare_rustcat_and_cat_for_linux(&rustcat_args, &cat_args);
}

#[test]
fn test_v_t() {
    let test_args = vec!["-v", "-t"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_n() {
    let test_args = vec!["-v", "-n"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_b() {
    let test_args = vec!["-v", "-b"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_e() {
    let test_args = vec!["-v", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_s() {
    let test_args = vec!["-v", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_t_n() {
    let test_args = vec!["-v", "-t", "-n"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_n_b() {
    let test_args = vec!["-v", "-n", "-b"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_b_e() {
    let test_args = vec!["-v", "-b", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_e_s() {
    let test_args = vec!["-v", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_t_n_b() {
    let test_args = vec!["-v", "-t", "-n", "-b"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_n_b_e() {
    let test_args = vec!["-v", "-n", "-b", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_b_e_s() {
    let test_args = vec!["-v", "-b", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_t_n_b_e() {
    let test_args = vec!["-v", "-t", "-n", "-b", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_v_n_b_e_s() {
    let test_args = vec!["-v", "-n", "-b", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_n() {
    let test_args = vec!["-t", "-n"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_b() {
    let test_args = vec!["-t", "-b"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_e() {
    let test_args = vec!["-t", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_s() {
    let test_args = vec!["-t", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_n_b() {
    let test_args = vec!["-t", "-n", "-b"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_b_e() {
    let test_args = vec!["-t", "-b", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_e_s() {
    let test_args = vec!["-t", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_n_b_e() {
    let test_args = vec!["-t", "-n", "-b", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_b_e_s() {
    let test_args = vec!["-t", "-b", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_t_n_b_e_s() {
    let test_args = vec!["-t", "-n", "-b", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_n_b() {
    let test_args = vec!["-n", "-b"];

    compare_rustcat_and_cat(&test_args);
}
#[test]
fn test_n_e() {
    let test_args = vec!["-n", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_n_s() {
    let test_args = vec!["-n", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_n_b_e() {
    let test_args = vec!["-n", "-b", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_n_e_s() {
    let test_args = vec!["-n", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_b_e() {
    let test_args = vec!["-b", "-e"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_b_s() {
    let test_args = vec!["-b", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_b_e_s() {
    let test_args = vec!["-b", "-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}

#[test]
fn test_e_s() {
    let test_args = vec!["-e", "-s"];

    compare_rustcat_and_cat(&test_args);
}
