extern crate greps;

use greps::{Config, run};

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("file : {}", config.file);

    if let Err(e) = run(config) {
        println!("Error with: {}", e);
        process::exit(1);
    }
}
