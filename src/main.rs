use std::env;

use rust_cotd::*;

fn main() {
    let config = Config::new(env::args());
    run(&config);
}