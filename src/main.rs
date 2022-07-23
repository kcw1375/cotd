use std::env;

use rust_cotd::*;

fn main() {
    // the directory where executables are
    // ie where $PATH points to
    let dir = "/usr/bin";

    let config = Config::new(env::args());
    
    // get random command from /usr/bin
    let command_name = get_random_command(dir);
    println!("{command_name}");

    // get description of the command
    if config.show_info {
        match command_description(&command_name) {
            Ok(desc) => println!("{desc}"),
            Err(err) => println!("{err}"),
        }
    }
}