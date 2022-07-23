use std::env;
use std::process;

use rust_cotd::*;

fn main() {
    let config = match Config::new(env::args()) {
        Err(desc) => {
            eprintln!("Error: {desc}");
            process::exit(1); // temp code, should be different based on error
        },
        Ok(config) => config,
    };

    run(&config);
}