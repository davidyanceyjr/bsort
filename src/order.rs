use std::cmp::Ordering;

use crate::Order;

pub fn compare_pair(order: Order, left: i64, right: i64) -> Ordering {
    match order {
        Order::Ascending => left.cmp(&right),
        Order::Descending => right.cmp(&left),
    }
}

pub fn needs_swap(order: Order, left: i64, right: i64) -> bool {
    compare_pair(order, left, right) == Ordering::Greater
}
