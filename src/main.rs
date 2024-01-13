#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    RgbColor(u8, u8, u8), // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

fn main() {
    // let color = Color::RGB(127, 17, 40);
    let color = Color::Red;

    match color {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        Color::RgbColor(r, g, b) => println!("RgbColor({}, {}, {})", r, g, b),
        Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: _,
        } => println!("CmykColor"),
    }
}
