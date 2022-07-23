use std::fs;
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

    println!("{command:?}");
}