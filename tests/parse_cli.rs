use bsort::{parse_lines, USAGE_OR_PARSE_ERROR};

#[test]
fn parse_lines_accepts_empty_input() {
    assert_eq!(parse_lines("", false, false).unwrap(), Vec::<i64>::new());
}

#[test]
fn parse_lines_trims_whitespace() {
    assert_eq!(
        parse_lines(" 1\n\t-2 \n 3 ", false, false).unwrap(),
        vec![1, -2, 3]
    );
}

#[test]
fn parse_lines_rejects_blank_line_when_not_ignored() {
    let err = parse_lines("4\n\n9\n", false, false).unwrap_err();

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "line 2: invalid integer ''");
}

#[test]
fn parse_lines_ignores_blank_line_when_enabled() {
    assert_eq!(parse_lines("4\n\n9\n", true, false).unwrap(), vec![4, 9]);
}

#[test]
fn parse_lines_reports_invalid_first_line() {
    let err = parse_lines("nope\n2\n", false, false).unwrap_err();

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "line 1: invalid integer 'nope'");
}

#[test]
fn parse_lines_reports_invalid_later_line() {
    let err = parse_lines("1\n2x\n3\n", false, false).unwrap_err();

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "line 2: invalid integer '2x'");
}

#[test]
fn parse_lines_rejects_out_of_range_integer() {
    let err = parse_lines("9223372036854775808\n", false, false).unwrap_err();

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "line 1: invalid integer '9223372036854775808'");
}

#[test]
fn parse_lines_skips_dirty_rows_when_valid_only_enabled() {
    assert_eq!(
        parse_lines("4\n\nnope\n9\n9223372036854775808\n", false, true).unwrap(),
        vec![4, 9]
    );
}
