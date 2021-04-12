#![allow(dead_code)]
#![allow(unused_variables)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn sorter(coin: Coin) -> u8 {
    match Coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    },
}
fn main() {
    println!("Hello, world!");

    fn main() {
        sorter(Penny)
    }
}
