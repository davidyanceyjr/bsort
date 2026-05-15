use bsort::{
    AppError, AppResult, Mode, Order, CHECK_FAILED, IO_ERROR, SUCCESS, USAGE_OR_PARSE_ERROR,
};

#[test]
fn exit_codes_match_spec_contract() {
    assert_eq!(SUCCESS, 0);
    assert_eq!(CHECK_FAILED, 1);
    assert_eq!(USAGE_OR_PARSE_ERROR, 2);
    assert_eq!(IO_ERROR, 3);
}

#[test]
fn order_variants_cover_both_directions() {
    assert_eq!(Order::Ascending, Order::Ascending);
    assert_eq!(Order::Descending, Order::Descending);
    assert_ne!(Order::Ascending, Order::Descending);
}

#[test]
fn mode_variants_cover_sort_count_and_check() {
    assert_eq!(Mode::Sort, Mode::Sort);
    assert_eq!(Mode::Count, Mode::Count);
    assert_eq!(Mode::Check, Mode::Check);
}

#[test]
fn app_error_carries_exit_code_and_message() {
    let err = AppError::new(USAGE_OR_PARSE_ERROR, "bad input");

    assert_eq!(err.exit_code, USAGE_OR_PARSE_ERROR);
    assert_eq!(err.message, "bad input");
}

#[test]
fn app_result_alias_uses_app_error() {
    let result: AppResult<()> = Err(AppError::new(CHECK_FAILED, "not sorted"));

    assert_eq!(
        result.expect_err("result should be an error"),
        AppError::new(CHECK_FAILED, "not sorted")
    );
}
