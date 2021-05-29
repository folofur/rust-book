use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("You're searching for: {}", query);
    println!("in this file: {}", filename);

    let contents = fs::read_to_string(filename).expect("Unable to read that specified file.");

    println!("Which does contain this text: \n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
