#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

enum Temperture {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperture::Celsius(10);

    match temperature {
        Temperture::Celsius(t) if t > 30 => println!("Hot"),
        Temperture::Celsius(t) => println!("Cold on {:}", t),

        Temperture::Fahrenheit(t) if t > 86 => println!("Hot"),
        Temperture::Fahrenheit(t) => println!("Cold on {:}", t),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Unhandled case"),
    }
}
