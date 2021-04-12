#![allow(dead_code)]
#![allow(unused_variables)]

enum coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}
fn sorter() {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
fn main() {
  sorter(Penny)
}
