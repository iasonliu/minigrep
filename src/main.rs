use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in filename: {}", config.query, config.filename);
    if let Err(e) = minigrep::run(config) {
        println!("Appliction error: {}", e);
        process::exit(1);
    }
}
