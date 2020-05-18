extern crate sist;

use sist::posix::repl::run;

use std::time::Duration;

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


    run(port_name, settings);

    println!("Disconnected.");
}

