#![allow(dead_code)]
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u8 is: {}", 1000 as u16);

    // println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    let _c = 1000 % 256;
    println!("1000 mod 256 is : {}", 1000 % 256);

    println!(" 128 as a i16 is: {}", 128 as i16);

    println!(" 1000 as a u8 is : {}", 1000 as u8);

    println!(" 232 as a i8 is : {}", 232 as i8);

    println!("    nan as u8 is : {}", f32::NAN as u8);

    unsafe {
        println!(" 300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());

        println!(
            " -100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
    }
}
