use crate::{AppError, AppResult, IO_ERROR};
use std::fs;
use std::io::{self, Read};

pub fn read_stdin_text() -> io::Result<String> {
    read_all(std::io::stdin())
}

pub fn read_file_text(path: &str) -> AppResult<String> {
    fs::read_to_string(path)
        .map_err(|err| AppError::new(IO_ERROR, format!("{}: {}", path, err)))
}

fn read_all<R: Read>(mut reader: R) -> io::Result<String> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::{read_all, read_file_text};
    use crate::IO_ERROR;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn read_all_handles_empty_input() {
        assert_eq!(read_all("".as_bytes()).expect("read should succeed"), "");
    }

    #[test]
    fn read_all_handles_text_input() {
        assert_eq!(
            read_all("1\n2\n3\n".as_bytes()).expect("read should succeed"),
            "1\n2\n3\n"
        );
    }

    #[test]
    fn read_file_text_reads_existing_file() {
        let path = unique_test_path("bsort-io-ok");
        fs::write(&path, "7\n8\n9\n").expect("write should succeed");

        let text = read_file_text(path.to_str().expect("utf-8 path")).expect("read should succeed");

        assert_eq!(text, "7\n8\n9\n");

        fs::remove_file(path).expect("cleanup should succeed");
    }

    #[test]
    fn read_file_text_maps_missing_file_to_exit_3() {
        let path = unique_test_path("bsort-io-missing");
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
}
