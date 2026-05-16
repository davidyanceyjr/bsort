use std::cmp::Ordering;

use bsort::{compare_pair, needs_swap, Order};

#[test]
fn compare_pair_flips_between_ascending_and_descending() {
    assert_eq!(compare_pair(Order::Ascending, 2, 5), Ordering::Less);
    assert_eq!(compare_pair(Order::Descending, 2, 5), Ordering::Greater);
}

#[test]
fn compare_pair_returns_equal_for_equal_values() {
    assert_eq!(compare_pair(Order::Ascending, 4, 4), Ordering::Equal);
    assert_eq!(compare_pair(Order::Descending, 4, 4), Ordering::Equal);
}

#[test]
fn compare_pair_handles_negative_values() {
    assert_eq!(compare_pair(Order::Ascending, -2, -5), Ordering::Greater);
    assert_eq!(compare_pair(Order::Descending, -2, -5), Ordering::Less);
}

#[test]
fn needs_swap_matches_ascending_behavior() {
    assert!(needs_swap(Order::Ascending, 9, 1));
    assert!(!needs_swap(Order::Ascending, 1, 9));
}

#[test]
fn needs_swap_matches_descending_behavior() {
    assert!(needs_swap(Order::Descending, 1, 9));
    assert!(!needs_swap(Order::Descending, 9, 1));
}

#[test]
fn needs_swap_is_false_for_equal_values() {
    assert!(!needs_swap(Order::Ascending, 3, 3));
    assert!(!needs_swap(Order::Descending, 3, 3));
}
