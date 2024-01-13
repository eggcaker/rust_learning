#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

/*
❯ gpt in rust when i need use ref keyword to create a reference

  In Rust, the  ref  keyword is used to create a reference to a value. This keyword allows you
  to create a reference to a value for use in a pattern. Typically, this is used in the context
  of pattern matching or destructuring a data structure.

  Let's look at a simple example:

    let x = 5;
    match x {
        ref r => println!("Got a reference to {}", r),
    }

  In the above example,  ref r  creates a reference  r  to the value  x  in the pattern of the
  match  expression. You would then be able to use  r  inside the match arm as a reference to
  the value  x .

  Without  ref ,  r  would try to take ownership of the value, which is not always what you
  want. In particular, if the value is not  Copy  (i.e., it doesn't implement the  Copy  trait),
  taking ownership would be a move, making the original unavailable.

  Note:  ref  keyword is not as commonly used in Rust as it used to be, because Rust 2018
  introduced a feature that allows you to take references in a more straightforward way. You can
  simply write  &  in the pattern to take a reference, as follows:

    let x = 5;
    match x {
        r => println!("Got a reference to {}", &r),
    }
*/

fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match value {
        r => println!("Got a refrence value: {:?}", &r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}
