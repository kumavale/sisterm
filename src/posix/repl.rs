use std::io::{self, Read, Write};
use std::thread;

use serialport::prelude::*;


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
        println!("Receiving data on {}:", &port_name);
        loop {
            match receiver.read(serial_buf.as_mut_slice()) {
                Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    });


    // Transmitter
    let mut input_buffer = [0];
    loop {
        if io::stdin().read(&mut input_buffer).is_ok() {
            match transmitter.write(&input_buffer) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        } else {
            eprintln!("error: write() failed");
        }
    }
}
