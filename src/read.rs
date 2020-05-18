use std::fs::File;
use std::io::prelude::*;

use crate::color;
use crate::flag;

pub fn run(path: &str, flags: flag::Flags) {
    let mut f = File::open(path).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Somothing went wrong reading the file");

    if flags.is_nocolor() {
        println!("{}", contents);
    } else {
        color::coloring_from_file(contents);
        println!();
    }
}
