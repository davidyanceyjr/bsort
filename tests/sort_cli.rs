use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_with_stdin(args: &[&str], input: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_bsort"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("binary should run");

    child
        .stdin
        .take()
        .expect("stdin should be available")
        .write_all(input.as_bytes())
        .expect("write should succeed");

    child.wait_with_output().expect("wait should succeed")
}

fn run_with_file(args: &[&str], contents: &str) -> std::process::Output {
    let path = unique_test_path("bsort-sort-cli");
    fs::write(&path, contents).expect("write should succeed");

    let output = Command::new(env!("CARGO_BIN_EXE_bsort"))
        .args(args)
        .arg(path.to_str().expect("utf-8 path"))
        .output()
        .expect("binary should run");

    fs::remove_file(path).expect("cleanup should succeed");
    output
}

fn unique_test_path(prefix: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nanos}.txt"))
}

#[test]
fn sorts_stdin_in_ascending_order() {
    let output = run_with_stdin(&[], "3\n1\n2\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1\n2\n3\n");
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn sorts_file_input_in_ascending_order() {
    let output = run_with_file(&[], "8\n-1\n4\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "-1\n4\n8\n");
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn sorts_stdin_in_descending_order() {
    let output = run_with_stdin(&["--desc"], "3\n1\n2\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "3\n2\n1\n");
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn removes_duplicates_after_sorting() {
    let output = run_with_stdin(&["--unique"], "3\n1\n3\n2\n2\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1\n2\n3\n");
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn count_mode_prints_number_of_parsed_values() {
    let output = run_with_stdin(&["--count"], "3\n1\n3\n2\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "4\n");
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn duplicate_values_are_preserved_without_unique() {
    let output = run_with_stdin(&[], "3\n1\n3\n2\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1\n2\n3\n3\n");
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn empty_input_prints_nothing() {
    let output = run_with_stdin(&[], "");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}
