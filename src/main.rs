#![allow(dead_code)]

use crate::MyList::*;

enum MyList {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<MyList>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl MyList {
    fn new() -> MyList {
        Nil
    }

    fn prepend(self, elem: u32) -> MyList {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(h, ref tail) => {
                format!("{}, {}", h, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = MyList::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
