#![allow(overflowing_literals)]

fn main() {
    let decimal = 859.34_f32;
    // let integer: u8 = decimal; Doesn't work
    let integer = decimal as u8;
    let character = integer as char;    
    println!("Castings: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);
    // 1000 - 256 -256 -256 = 232 (1000 as u8)

    println!("300.0 is {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    println!("nan as u8 is {}", f32::NAN as u8);




}
