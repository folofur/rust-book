#![allow(dead_code)]
#![allow(unused_variables)]

enum IpAddress {
  v4(u8, u8, u8, u8),
  v6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32,i32,i32)
}
impl Message {
  fn retrySend(&self) {
    // implement a function on an enum just like with structs
  }
  fn removeMessage(&self) {
    // implement a function on an enum just like with structs
  }
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
  enums();
}
