use std::fs::File;
use std::io::Read;

use crate::color;
use crate::flag;
use crate::setting;
use crate::hexdump::hexdump;

pub fn run(path: &str, flags: flag::Flags, params: Option<setting::Params>) {
    let mut f = File::open(path).expect("File open failed");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Somothing went wrong reading the file");

    if *flags.hexdump() {
        hexdump(contents.as_bytes());
    } else {
        // Without coloring
        if *flags.nocolor() {
            println!("{}", contents);

        // Coloring
        } else {
            color::coloring_from_file(contents, params);
        }
    }
}
