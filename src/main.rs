use std::fs;
use std::process::Command;
use rand;

fn main() {
    // the directory where executables are
    // ie where $PATH points to
    let dir = "/usr/bin";
    
    // an iterator that contains all files in /usr/bin
    let iter = fs::read_dir(dir).unwrap();
    let count = iter.count();

    // now choose a random executable
    let mut iter = fs::read_dir(dir).unwrap();
    let n : usize = (rand::random::<f32>() * count as f32) as usize;
    let command = iter.nth(n).unwrap().unwrap();

    // command name as string
    let command_name = format!("{}", command.file_name().to_str().unwrap());
    println!("{command_name}");

    // gets a one-line description from the manpages about selected command
    Command::new("whatis")
        .arg(format!("{command_name}"))
        .spawn()
        .expect("whatis command failed to start");
}