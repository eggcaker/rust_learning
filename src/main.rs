#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_call();
    my::nested::function();
}
