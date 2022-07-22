use std::fs;

fn main() {
    // the directory where executables are
    // ie where $PATH points to
    let dir = "/usr/bin";
    
    // an iterator that contains all files in /usr/bin
    let mut iter = fs::read_dir(dir).unwrap();
    println!("{:?}",iter.next());
}