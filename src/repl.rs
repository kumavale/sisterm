use std::io::{self, BufWriter, Read, Write};
use std::thread;
use std::fs::File;

use crate::queue::Queue;
use crate::flag;

use serialport::prelude::*;
use getch::Getch;


pub fn run(port_name: String, settings: SerialPortSettings, flags: flag::Flags) {
    let receiver = match serialport::open_with_settings(&port_name, &settings) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        },
    };
    let transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    println!("Connected. {}:", port_name);
    println!("Type \"~.\" to exit.");

    // Receiver
    thread::spawn(move || {
        receiver_run(receiver, flags);
    });

    // Transmitter
    transmitter_run(transmitter);
}

fn receiver_run(mut port: std::boxed::Box<dyn serialport::SerialPort>, flags: flag::Flags) {
    let mut serial_buf: Vec<u8> = vec![0; 1000];

    if let Some(write_file) = flags.write_file() {
        // Save log
        let mut log_file = BufWriter::new(File::create(write_file).expect("File open failed"));
        println!("Log record: {}", write_file);

        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    // Display received string
                    io::stdout().write_all(&serial_buf[..t]).unwrap();

                    // Write to log file
                    log_file.write_all(&serial_buf[..t]).unwrap();
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{}", e),
            }

            // Flush
            let _ = io::stdout().flush();
            let _ = log_file.flush();
        }

    } else {
        // Non save log
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    // Display received string
                    io::stdout().write_all(&serial_buf[..t]).unwrap();
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{}", e),
            }

            // Flush
            let _ = io::stdout().flush();
        }
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
                    eprint!(".");
                    let _ = io::stdout().flush();
                    break;
                }
                // If the previous character is not a tilde and the current character is a tilde
                if !last_is_tilde && key == exit_char1 {
                    last_is_tilde = true;
                    eprint!("~");
                    let _ = io::stdout().flush();
                    continue;
                }
                // If not input "~~" to dispaly error message
                if last_is_tilde {
                    if key == exit_char1 {
                        eprint!("\x08");
                        queue.enqueue(0);
                    } else {
                        eprint!("\x08");
                        eprintln!("[Unrecognized.  Use ~~ to send ~]");
                        let _ = io::stdout().flush();
                        last_is_tilde = false;
                        continue;
                    }
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
