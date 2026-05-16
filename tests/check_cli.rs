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
    let path = unique_test_path("bsort-check-cli");
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
fn check_mode_returns_success_for_sorted_stdin() {
    let output = run_with_stdin(&["--check"], "1\n2\n3\n");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn check_mode_returns_exit_one_for_unsorted_stdin() {
    let output = run_with_stdin(&["--check"], "1\n3\n2\n");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn check_mode_returns_success_for_sorted_file() {
    let output = run_with_file(&["--check"], "-3\n0\n8\n");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn check_mode_rejects_desc_check_combination() {
    let output = run_with_stdin(&["--desc", "--check"], "3\n2\n1\n");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stdout).is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).trim(),
        "cannot combine --desc with --check"
    );
}
