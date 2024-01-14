#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    // let x: ! = panic!("This call never returns.");
}
