use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_bsort"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn help_prints_to_stdout_and_exits_zero() {
    let output = run(&["--help"]);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: bsort [OPTIONS] [FILE]"));
    assert!(stdout.contains("--help"));
    assert!(stdout.contains("--version"));
}

#[test]
fn version_prints_to_stdout_and_exits_zero() {
    let output = run(&["--version"]);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("bsort {}\n", env!("CARGO_PKG_VERSION"))
    );
}
