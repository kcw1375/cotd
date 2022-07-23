use std::fs;
use std::process::Command;

pub struct Config {
    // stores the arguments passed in
    pub show_info: bool
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        // handles argument parsing

        args.next(); // skip first element since that's the program name

        let mut show_info = false;
        for arg in args {
            match arg.as_str() {
                "-i" => show_info = true,
                _ => return Err("Not a valid flag.".to_string()),
            }
        }
        Ok(Config {show_info})
    }
}

pub fn run(config: &Config) {
    // the directory where executables are
    // ie where $PATH points to
    let dir = "/usr/bin";

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

pub fn command_description(command_name: &str) -> Result<String, String> {
    // gets a one-line description from the manpages about selected command

    // capture the output from whatis
    let output = Command::new("whatis")
        .arg(format!("{command_name}"))
        .output()
        .expect("whatis command failed to start");

    if output.status.success() {
        // return the stdout if success
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        // return the stderr if failure
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn get_random_command(dir: &str) -> String {
    // an iterator that contains all files in /usr/bin
    let iter = fs::read_dir(dir).unwrap();
    let count = iter.count();

    // now choose a random executable
    let mut iter = fs::read_dir(dir).unwrap();
    let n : usize = (rand::random::<f32>() * count as f32) as usize;
    let command = iter.nth(n).unwrap().unwrap();

    let command_name = format!("{}", command.file_name().to_str().unwrap());
    command_name
}