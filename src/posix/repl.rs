use std::io::{self, Read, Write};
use std::thread;

use super::queue::Queue;

use serialport::prelude::*;
use getch::Getch;


pub fn run(port_name: String, settings: SerialPortSettings) {
    let receiver = match serialport::open_with_settings(&port_name, &settings) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        },
    };
    let transmitter = receiver.try_clone().expect("Failed to clone from receiver");


    // Receiver
    thread::spawn(move || {
        receiver_run(receiver, &port_name);
    });

    // Transmitter
    transmitter_run(transmitter);
}

fn receiver_run(mut port: std::boxed::Box<dyn serialport::SerialPort>, port_name: &str) {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    println!("Connected. {}:", port_name);
    println!("Type \"~.\" to exit.");

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                // Display received string
                io::stdout().write_all(&serial_buf[..t]).unwrap();
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        let _ = io::stdout().flush();
    }
}

fn transmitter_run(mut port: std::boxed::Box<dyn serialport::SerialPort>) {
    let exit_char1 = b'~';
    let exit_char2 = b'.';
    let mut queue = Queue::new(exit_char1, exit_char2);
    let mut last_is_tilde = false;
    let g = Getch::new();

    loop {
        match g.getch() {
            Ok(key) => {
                queue.enqueue(key);
                // If input "~." to exit
                if queue.is_exit_chars() {
                    print!(".");
                    let _ = io::stdout().flush();
                    break;
                }
                // If the previous character is not a tilde and the current character is a tilde
                if !last_is_tilde && key == exit_char1 {
                    last_is_tilde = true;
                    print!("~");
                    let _ = io::stdout().flush();
                    continue;
                }
                // If not input "~~" to dispaly error message
                if last_is_tilde && key != exit_char1 {
                    eprintln!("[Unrecognized.  Use ~~ to send ~]");
                    last_is_tilde = false;
                    continue;
                }
                last_is_tilde = false;

                // Send key
                if let Err(e) = port.write(&[key]) {
                    eprintln!("{}", e);
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
