use crate::{needs_swap, Order};

pub fn is_sorted(values: &[i64], order: Order) -> bool {
    values
        .windows(2)
        .all(|pair| !needs_swap(order, pair[0], pair[1]))
}
