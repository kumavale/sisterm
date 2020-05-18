use std::fs::File;
use std::io::prelude::*;

use crate::color;

pub fn run(path: &str) {
    let mut f = File::open(path).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Somothing went wrong reading the file");

    color::coloring_from_file(contents);
}
