#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    let d: Foo = Foo::Qux(101);
    let e = Foo::Bar;

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = d {
        println!("c is one hundred {:}", value);
    } else {
        println!("c is not one hundred");
    }

    let optional = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = Some(3);

    if let Some(i) = optional {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    match emotion {
        Some(i) => {
            println!("Matched {:?}", i);
        }
        None => {
            println!("Didn't match a number. Let's go with a letter!");
        }
    }

    if let Some(i) = emotion {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    }

    if let Foo::Bar = e {
        println!("f2 is foobar");
    }
}
