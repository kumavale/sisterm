use std::io::{self, BufWriter, Read, Write};
use std::thread;
use std::fs::File;
use std::path::Path;

use crate::queue::Queue;
use crate::flag;

use serialport::prelude::*;
use getch::Getch;
use chrono::Local;


pub fn run(port_name: String, settings: SerialPortSettings, flags: flag::Flags) {
    let receiver = match serialport::open_with_settings(&port_name, &settings) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        },
    };
    let transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    // If write_file is already exists
    if let Some(write_file) = flags.write_file() {
        if Path::new(write_file).exists() {
            let g = Getch::new();
            println!("\"{}\" is already exists!", write_file);
            println!("Press ENTER to continue overwrite");
            match g.getch() {
                Ok(b'\n') | Ok(b'\r') => (),  // continue
                _ => std::process::exit(0),   // exit
            }
        }
    }

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

    // Save log
    if let Some(write_file) = flags.write_file() {

        let mut log_file = BufWriter::new(File::create(write_file).expect("File open failed"));
        let mut log_buf = String::new();
        let mut write_flag = false;
        println!("Log record: \"{}\"", write_file);

        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    // Display received string
                    io::stdout().write_all(&serial_buf[..t]).unwrap();

                    // Check exist '\n'
                    for ch in &serial_buf[..t] {
                        // If '\n' exists, set write_flag to true
                        if ch == &b'\n' {
                            write_flag = true;
                            break;
                        }
                    }

                    // Write timestamp to log file
                    // If '\n' exists, replace to timestamp from '\n'
                    if flags.is_timestamp() && write_flag {
                        // Write to log file. Also the timestamp
                        string_from_utf8_appearance(&mut log_buf, &serial_buf[..t]);
                        log_buf = log_buf.replace("\n", &format_timestamp());

                    } else {
                        // Write to log file
                        string_from_utf8_appearance(&mut log_buf, &serial_buf[..t]);
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{}", e),
            }

            // Flush stdout
            let _ = io::stdout().flush();

            // If end of '\n' then write to log file
            if write_flag {
                log_file.write_all(log_buf.as_bytes()).unwrap();
                log_file.flush().unwrap();
                log_buf.clear();
                write_flag = false;
            }
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

fn format_timestamp() -> String {
    Local::now().format("\n[%Y-%m-%d %H:%M:%S %Z] ").to_string()
}

fn string_from_utf8_appearance(log_buf: &mut String, serial_buf: &[u8]) {
    for c in serial_buf {
        match *c {
            0x7 => (),  // ignore BELL
            0x8 => { log_buf.pop(); }  // BS
            _ => *log_buf += &(*c as char).to_string(),
        }
    }
}
