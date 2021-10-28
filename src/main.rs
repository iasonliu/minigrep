use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for {} in filename: {}",
        config.query, config.filename
    );
    if let Err(e) = minigrep::run(config) {
        eprintln!("Appliction error: {}", e);
        process::exit(1);
    }
}
