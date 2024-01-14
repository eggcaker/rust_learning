#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greentig = "green";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greentig);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
