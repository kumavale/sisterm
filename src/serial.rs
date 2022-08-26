use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use crate::repl;
use crate::flag;
use crate::setting;
use crate::getch::{Getch, Key};

pub async fn run(port_name: String,
                 baud_rate: u32,
                 mut flags: flag::Flags,
                 params:    Option<setting::Params>)
{
    let receiver = serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        });
    let transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    let (tx, rx) = mpsc::channel();

    // If write_file is already exists
    if let Some(write_file) = flags.write_file() {
        if Path::new(&write_file).exists() {
            if !*flags.append() {
                let g = Getch::new();
                println!("\"{}\" is already exists!", &write_file);
                println!("Press ENTER to continue overwrite");
                match g.getch() {
                    Ok(Key::Char('\r')) => (),   // continue
                    _ => std::process::exit(0),  // exit
                }
            }
        } else if *flags.append() {
            let g = Getch::new();
            println!("\"{}\" is not exists!", &write_file);
            println!("Press ENTER to create the file and continue");
            match g.getch() {
                Ok(Key::Char('\r')) => (),   // continue
                _ => std::process::exit(0),  // exit
            }
            *flags.append_mut() = false;
        }
    }

    // Check if params exists
    if params.is_none() {
        *flags.nocolor_mut() = true;
    }

    println!("Type \"~.\" to exit.");
    println!("Connecting... {}", port_name);

    let flags       = Arc::new(Mutex::new(flags));
    let flags_clone = flags.clone();

    tokio::select! {
        _ = tokio::spawn(repl::receiver(receiver, rx, flags_clone, params)) => {
            println!("\n\x1b[0mDisconnected.");
            std::process::exit(0);
        }
        _ = tokio::spawn(repl::transmitter(transmitter, tx, flags)) => {}
    }
}
