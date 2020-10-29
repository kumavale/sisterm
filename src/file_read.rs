use std::fs::File;
use std::io::Read;

use crate::color;
use crate::flag;
use crate::setting;

pub fn run(path: &str, flags: flag::Flags, params: Option<setting::Params>) {
    let mut f = File::open(path).expect("File open failed");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Somothing went wrong reading the file");

    // Without coloring
    if *flags.nocolor() {
        println!("{}", contents);

    // Coloring
    } else {
        color::coloring_from_file(contents, params);
    }
}
