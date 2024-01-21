#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

mod my;

fn function() {
    println!("called `function()`");
}

fn main() {

    // rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    // rary::indirect_access();

    println!("main function called")
}
