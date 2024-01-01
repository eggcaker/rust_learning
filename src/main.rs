#![allow(dead_code)]

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! The value of immutable variables can't be changed
    // _immutable_binding += 1;
}
