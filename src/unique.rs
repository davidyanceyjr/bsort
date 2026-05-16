pub fn dedupe_sorted(values: &[i64]) -> Vec<i64> {
    let mut deduped = Vec::with_capacity(values.len());

    for value in values {
        if deduped.last() != Some(value) {
            deduped.push(*value);
        }
    }

    deduped
}
