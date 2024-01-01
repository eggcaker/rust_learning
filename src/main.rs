#![allow(dead_code)]

fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);

        let shadowed_bindind = "abc";

        println!("inner shadowed: {}", shadowed_bindind);
    }

    // println!("outer short: {}", short_lived_binding);

    let shadowed_bindind = 1;

    println!("outer shadowed: {}", shadowed_bindind);

    println!("outer long: {}", long_lived_binding);
}
