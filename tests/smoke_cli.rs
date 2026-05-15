use std::process::Command;

#[test]
fn binary_runs_with_empty_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_bsort"))
        .output()
        .expect("binary should run");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}
