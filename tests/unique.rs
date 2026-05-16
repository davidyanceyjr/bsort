use bsort::dedupe_sorted;

#[test]
fn dedupe_sorted_handles_empty_input() {
    assert_eq!(dedupe_sorted(&[]), Vec::<i64>::new());
}

#[test]
fn dedupe_sorted_handles_single_item() {
    assert_eq!(dedupe_sorted(&[7]), vec![7]);
}

#[test]
fn dedupe_sorted_keeps_distinct_values() {
    assert_eq!(dedupe_sorted(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn dedupe_sorted_collapses_all_duplicate_values() {
    assert_eq!(dedupe_sorted(&[5, 5, 5, 5]), vec![5]);
}

#[test]
fn dedupe_sorted_collapses_mixed_duplicate_runs() {
    assert_eq!(dedupe_sorted(&[1, 1, 2, 3, 3, 3, 8, 8]), vec![1, 2, 3, 8]);
}

#[test]
fn dedupe_sorted_handles_descending_sorted_input() {
    assert_eq!(dedupe_sorted(&[9, 9, 7, 4, 4, 1]), vec![9, 7, 4, 1]);
}
