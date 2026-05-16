use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_bsort"))
        .args(args)
        .output()
        .expect("binary should run")
}

fn run_with_stdin(args: &[&str], input: &str) -> Output {
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
        .expect("stdin write should succeed");

    child.wait_with_output().expect("wait should succeed")
}

fn run_with_fixture(args: &[&str], fixture: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_bsort"))
        .args(args)
        .arg(fixture_path(fixture))
        .output()
        .expect("binary should run")
}

fn run_with_data_file(args: &[&str], data_file: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_bsort"))
        .args(args)
        .arg(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("data")
                .join(data_file),
        )
        .output()
        .expect("binary should run")
}

#[test]
fn stdin_ascending_matches_spec_example() {
    let output = run_with_stdin(&[], "3\n1\n2\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1\n2\n3\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn file_input_sorts_with_whitespace_and_negative_values() {
    let output = run_with_fixture(&[], "file_numbers.txt");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "-2\n0\n4\n9\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn descending_sort_preserves_duplicates() {
    let output = run_with_stdin(&["--desc"], "3\n1\n3\n2\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "3\n3\n2\n1\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn unique_removes_duplicates_after_sorting() {
    let output = run_with_stdin(&["--unique"], "3\n1\n3\n1\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1\n3\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn count_prints_number_of_parsed_values() {
    let output = run_with_stdin(&["--count"], "3\n1\n3\n");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "3\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn check_returns_zero_for_sorted_input() {
    let output = run_with_fixture(&["--check"], "sorted_numbers.txt");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn check_returns_one_for_unsorted_input() {
    let output = run_with_fixture(&["--check"], "unsorted_numbers.txt");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn ignore_blank_controls_blank_line_handling() {
    let rejected = run_with_fixture(&[], "blank_lines.txt");

    assert_eq!(rejected.status.code(), Some(2));
    assert!(rejected.stdout.is_empty());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("line 2"));

    let accepted = run_with_fixture(&["--ignore-blank"], "blank_lines.txt");

    assert!(accepted.status.success());
    assert_eq!(String::from_utf8_lossy(&accepted.stdout), "1\n2\n");
    assert!(accepted.stderr.is_empty());
}

#[test]
fn invalid_integer_error_includes_line_number() {
    let output = run_with_fixture(&[], "invalid_integer.txt");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("line 2"));
    assert!(stderr.contains("abc"));
}

#[test]
fn valid_only_sorts_dirty_data_file() {
    let output = run_with_data_file(&["--valid-only"], "messy_data.data");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "-2147483649\n-42\n-7\n-3\n0\n0\n5\n5\n9\n11\n17\n17\n19\n19\n42\n73\n88\n2147483648\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn valid_only_count_uses_only_valid_rows() {
    let output = run_with_data_file(&["--valid-only", "--count"], "messy_data.data");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "18\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn valid_only_check_evaluates_only_valid_rows() {
    let output = run_with_data_file(&["--valid-only", "--check"], "messy_data.data");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn dirty_data_file_still_fails_without_valid_only() {
    let output = run_with_data_file(&[], "messy_data.data");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("line 2"));
}

#[test]
fn too_many_arguments_exit_two() {
    let output = run(&[
        "tests/fixtures/file_numbers.txt",
        "tests/fixtures/sorted_numbers.txt",
    ]);

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("too many positional arguments"));
}

#[test]
fn missing_file_exit_three_includes_path() {
    let output = run(&["tests/fixtures/does-not-exist.txt"]);

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("tests/fixtures/does-not-exist.txt"));
}

#[test]
fn help_and_version_exit_zero() {
    let help = run(&["--help"]);
    assert!(help.status.success());
    assert!(help.stderr.is_empty());
    assert!(String::from_utf8_lossy(&help.stdout).contains("Usage: bsort [OPTIONS] [FILE]"));

    let version = run(&["--version"]);
    assert!(version.status.success());
    assert!(version.stderr.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&version.stdout),
        format!("bsort {}\n", env!("CARGO_PKG_VERSION"))
    );
}
