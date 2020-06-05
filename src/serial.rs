use std::thread;
use std::path::Path;
use std::sync::mpsc;

use crate::repl;
use crate::flag;
use crate::setting;
use crate::getch::Getch;

use serialport::SerialPortSettings;


pub fn run(port_name: String,
           settings:  SerialPortSettings,
           mut flags: flag::Flags,
           params:    Option<setting::Params>)
{
    let receiver = serialport::open_with_settings(&port_name, &settings).unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
        std::process::exit(1);
    });
    let transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    let (tx, rx) = mpsc::channel();

    // If write_file is already exists
    if let Some(write_file) = flags.write_file() {
        if Path::new(write_file).exists() {
            if !flags.is_append() {
                let g = Getch::new();
                println!("\"{}\" is already exists!", write_file);
                println!("Press ENTER to continue overwrite");
                match g.getch() {
                    Ok(b'\n') | Ok(b'\r') => (),  // continue
                    _ => std::process::exit(0),   // exit
                }
            }
        } else if flags.is_append() {
            let g = Getch::new();
            println!("\"{}\" is not exists!", write_file);
            println!("Press ENTER to create the file and continue");
            match g.getch() {
                Ok(b'\n') | Ok(b'\r') => (),  // continue
                _ => std::process::exit(0),   // exit
            }
            flags.set_append(false);
        }
    }

    // Check if params exists
    if params.is_none() {
        flags.set_nocolor(true);
    }

    println!("Connected. {}:", port_name);
    println!("Type \"~.\" to exit.");

    // Receiver
    let flags_clone = flags.clone();
    let handle = thread::spawn(move || {
        repl::receiver(receiver, rx, flags_clone, params);
    });

    // Transmitter
    repl::transmitter(transmitter, tx, flags);

    handle.join().unwrap();
}
