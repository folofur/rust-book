use std::io;

fn main() {
    println!("Which temp conversion would you like?");
    println!("For celcius to farhenheit, press 1");
    println!("For farhenheit to celcius, press 2");

    let mut decide = String::new();
    io::stdin()
        .read_line(&mut decide)
        .expect("failed to read line");

    let decide: &str = decide.trim();
    println!("you chose {}", decide)
    

}
