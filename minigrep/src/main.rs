use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Issue with parsing: {}", err);
        process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You've got to enter a query string and file location");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
