extern crate clap;
extern crate serialport;

use std::io::{self, Read, Write};
use std::time::Duration;
use std::thread;

use clap::{App, Arg};
use serialport::prelude::*;
use serialport::available_ports;

fn main() {
    let matches = App::new("sisterm")
        .version("2.0.0")
        .about("Reads data from a serial port and echoes it to stdout")
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .short("l")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("baud")
                .help("The baud rate to connect at (default 9600)")
                .short("s")
                .takes_value(true)
        )
        .get_matches();

    // If "port (-l)" is specified
    let port_name = if let Some(name) = matches.value_of("port") {
        name.to_string()
    } else {
        available_ports().expect("No serial port")[0].port_name.to_string()
    };

    // If "baudrate (-s)" is specified
    let default_baud_rate = "9600";
    let baud_rate = if let Some(baud) = matches.value_of("baud") {
        baud.to_string()
    } else {
        default_baud_rate.to_string()
    };


    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate;
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    let mut receiver = match serialport::open_with_settings(&port_name, &settings) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        },
    };
    let mut transmitter = receiver.try_clone().expect("Failed to clone from receiver");

    thread::spawn(move || {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
        loop {
            match receiver.read(serial_buf.as_mut_slice()) {
                Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    });

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

