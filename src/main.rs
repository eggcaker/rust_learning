#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("invalid input: {}", s);
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("invalid count: {}", count_str);
    };

    (count, item)
}

fn main() {
    assert_eq!(get_count_item("10 apples"), (10, "apples"));
}
