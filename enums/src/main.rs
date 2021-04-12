#![allow(dead_code)]
#![allow(unused_variables)]

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}
fn enums() {
    let c: Color = Color::Red;

    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RgbColor(0, 0, 0)
        | Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("no color"),
        Color::RgbColor(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
        _ => println!("catch all other cases"),
    }
}
fn main() {
    enums()
}
