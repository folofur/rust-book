use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("You're searching for: {}", config.query);
    println!("in this file: {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Unable to read that specified file.");

    println!("Which does contain this text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("You've got to enter a query string and file location")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
