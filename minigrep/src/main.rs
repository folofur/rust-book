use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("You're searching for: {}", query);
    println!("in this file: {}", filename);
}
