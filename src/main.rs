use std::env;
use std::process;

use todo_rs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = todo_rs::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
