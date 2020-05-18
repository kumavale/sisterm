use std::io::{self, Read, Write};
use std::thread;

use serialport::prelude::*;
use getch::Getch;


pub fn run(port_name: String, settings: SerialPortSettings) {
    let mut receiver = match serialport::open_with_settings(&port_name, &settings) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        },
    };
    let mut transmitter = receiver.try_clone().expect("Failed to clone from receiver");


    // Receiver
    thread::spawn(move || {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        println!("Connected. {}:", &port_name);

        loop {
            match receiver.read(serial_buf.as_mut_slice()) {
                Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    });


    // Transmitter
    let exit_char = b'~';
    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(key) => {
                if key == exit_char {
                    break;
                }
                if let Err(e) = transmitter.write(&[key]) {
                    eprintln!("{}", e);
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
