use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Issue with parsing: {}", err);
        process::exit(1);
    });

    println!("You're searching for: {}", config.query);
    println!("in this file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("App Error: {}", e);
        process::exit(1);
    }
}
