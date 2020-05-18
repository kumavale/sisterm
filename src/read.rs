use std::fs::File;
use std::io::prelude::*;

pub fn run(path: &str) {
    let mut f = File::open(path).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Somothing went wrong reading the file");

    println!("{}", contents);
}
