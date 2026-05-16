pub fn bubble_sort(values: &[i64]) -> Vec<i64> {
    let mut sorted = values.to_vec();
    let len = sorted.len();

    if len < 2 {
        return sorted;
    }

    for pass_end in (1..len).rev() {
        let mut swapped = false;

        for i in 0..pass_end {
            if sorted[i] > sorted[i + 1] {
                sorted.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    sorted
}
