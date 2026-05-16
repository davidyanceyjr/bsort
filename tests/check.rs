use bsort::{is_sorted, Order};

#[test]
fn is_sorted_accepts_empty_input() {
    assert!(is_sorted(&[], Order::Ascending));
    assert!(is_sorted(&[], Order::Descending));
}

#[test]
fn is_sorted_accepts_single_item() {
    assert!(is_sorted(&[7], Order::Ascending));
    assert!(is_sorted(&[7], Order::Descending));
}

#[test]
fn is_sorted_accepts_sorted_ascending_input() {
    assert!(is_sorted(&[1, 2, 3, 4], Order::Ascending));
}

#[test]
fn is_sorted_rejects_unsorted_ascending_input() {
    assert!(!is_sorted(&[1, 3, 2, 4], Order::Ascending));
}

#[test]
fn is_sorted_preserves_duplicate_validity() {
    assert!(is_sorted(&[1, 1, 2, 2, 4], Order::Ascending));
    assert!(is_sorted(&[4, 4, 2, 2, 1], Order::Descending));
}

#[test]
fn is_sorted_handles_descending_order() {
    assert!(is_sorted(&[9, 7, 7, 2], Order::Descending));
    assert!(!is_sorted(&[9, 7, 8, 2], Order::Descending));
}
