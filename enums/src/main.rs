#![allow(dead_code)]
#![allow(unused_variables)]

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8)
}
fn enums() 
{
    let c:Color = Color::Red;

    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RgbColor(0,0,0) => println!("no color"),
        Color::RgbColor(r,g,b) => println!("RGB: {}, {}, {}" , r, g, b)
    }
}
fn main() {}
