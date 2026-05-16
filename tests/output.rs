use bsort::{format_count, format_stderr, format_values, AppError, IO_ERROR, USAGE_OR_PARSE_ERROR};

#[test]
fn formats_empty_value_output() {
    assert_eq!(format_values(&[]), "");
}

#[test]
fn formats_multiple_values_one_per_line() {
    assert_eq!(format_values(&[3, -1, 7]), "3\n-1\n7\n");
}

#[test]
fn formats_count_output() {
    assert_eq!(format_count(42), "42\n");
}

#[test]
fn formats_parse_error_for_stderr() {
    let err = AppError::new(USAGE_OR_PARSE_ERROR, "line 2: invalid integer 'abc'");

    assert_eq!(format_stderr(&err), "line 2: invalid integer 'abc'\n");
}

#[test]
fn formats_file_error_for_stderr() {
    let err = AppError::new(IO_ERROR, "numbers.txt: No such file or directory");

    assert_eq!(
        format_stderr(&err),
        "numbers.txt: No such file or directory\n"
    );
}
