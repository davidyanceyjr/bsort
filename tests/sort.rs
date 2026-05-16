#[test]
fn bubble_sort_handles_empty_input() {
    assert_eq!(bsort::bubble_sort(&[]), Vec::<i64>::new());
}

#[test]
fn bubble_sort_handles_single_item() {
    assert_eq!(bsort::bubble_sort(&[7]), vec![7]);
}

#[test]
fn bubble_sort_preserves_duplicates() {
    assert_eq!(bsort::bubble_sort(&[4, 1, 4, 2, 1]), vec![1, 1, 2, 4, 4]);
}

#[test]
fn bubble_sort_handles_negative_numbers() {
    assert_eq!(bsort::bubble_sort(&[-3, 0, -1, 2]), vec![-3, -1, 0, 2]);
}

#[test]
fn bubble_sort_leaves_sorted_input_unchanged() {
    assert_eq!(bsort::bubble_sort(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn bubble_sort_handles_reverse_sorted_input() {
    assert_eq!(bsort::bubble_sort(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
}
