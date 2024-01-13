#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

fn print_s(s: String) {
    println!("{}", s);
}

fn main() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i: i32| i + outer_var;

    println!("{}", closure_annotated(1));
    println!("{}", closure_inferred(1));

    let one = || 1;
    println!("{}", one());

    let two = |s: &str| s.len();

    let s = String::from("Hello");

    println!("{}", two(&s));
    println!("{}", s);
    println!("{}", &s);
}
