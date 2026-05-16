use std::process::{Command, Stdio};

#[test]
fn binary_accepts_empty_stdin() {
    let output = Command::new(env!("CARGO_BIN_EXE_bsort"))
        .stdin(Stdio::piped())
        .output()
        .expect("binary should run");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn binary_accepts_normal_stdin_content() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_bsort"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("binary should run");

    use std::io::Write;

    child
        .stdin
        .take()
        .expect("stdin should be available")
        .write_all(b"4\n2\n9\n")
        .expect("write should succeed");

    let output = child.wait_with_output().expect("wait should succeed");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n4\n9\n");
    assert!(output.stderr.is_empty());
}
