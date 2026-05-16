use bsort::{read_file_text, IO_ERROR};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn reads_file_text() {
    let path = unique_test_path("bsort-file-io-ok");
    fs::write(&path, "10\n20\n").expect("write should succeed");

    let text = read_file_text(path.to_str().expect("utf-8 path")).expect("read should succeed");

    assert_eq!(text, "10\n20\n");

    fs::remove_file(path).expect("cleanup should succeed");
}

#[test]
fn missing_file_returns_exit_3_with_path() {
    let path = unique_test_path("bsort-file-io-missing");
    let path_str = path.to_str().expect("utf-8 path");

    let err = read_file_text(path_str).expect_err("read should fail");

    assert_eq!(err.exit_code, IO_ERROR);
    assert!(err.message.contains(path_str));
}

fn unique_test_path(prefix: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be valid")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nanos}.txt"))
}
