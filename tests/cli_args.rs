use bsort::{parse_args, Mode, Order, USAGE_OR_PARSE_ERROR};

fn parse(input: &[&str]) -> bsort::AppResult<bsort::CliConfig> {
    parse_args(input.iter().copied())
}

#[test]
fn accepts_zero_args() {
    let config = parse(&["bsort"]).unwrap();

    assert_eq!(config.input_path, None);
    assert_eq!(config.order, Order::Ascending);
    assert_eq!(config.mode, Mode::Sort);
    assert!(!config.unique);
    assert!(!config.ignore_blank);
    assert!(!config.help);
    assert!(!config.version);
}

#[test]
fn accepts_one_file_arg() {
    let config = parse(&["bsort", "nums.txt"]).unwrap();

    assert_eq!(config.input_path, Some("nums.txt".to_string()));
}

#[test]
fn rejects_too_many_positional_args() {
    let err = parse(&["bsort", "a.txt", "b.txt"]).unwrap_err();

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "too many positional arguments");
}

#[test]
fn parses_each_flag() {
    let desc = parse(&["bsort", "--desc"]).unwrap();
    let unique = parse(&["bsort", "--unique"]).unwrap();
    let count = parse(&["bsort", "--count"]).unwrap();
    let check = parse(&["bsort", "--check"]).unwrap();
    let ignore_blank = parse(&["bsort", "--ignore-blank"]).unwrap();
    let help = parse(&["bsort", "--help"]).unwrap();
    let version = parse(&["bsort", "--version"]).unwrap();

    assert_eq!(desc.order, Order::Descending);
    assert!(unique.unique);
    assert_eq!(count.mode, Mode::Count);
    assert_eq!(check.mode, Mode::Check);
    assert!(ignore_blank.ignore_blank);
    assert!(help.help);
    assert!(version.version);
}

#[test]
fn rejects_desc_with_count() {
    let err = parse(&["bsort", "--desc", "--count"]).unwrap_err();

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "cannot combine --desc with --count");
}

#[test]
fn rejects_desc_with_check() {
    let err = parse(&["bsort", "--desc", "--check"]).unwrap_err();

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "cannot combine --desc with --check");
}
