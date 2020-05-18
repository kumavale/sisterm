#[macro_use]
extern crate clap;
extern crate sist;

use std::time::Duration;

use clap::{App, AppSettings, Arg};
use serialport::prelude::*;
use serialport::available_ports;

fn main() {
    let app = App::new("sisterm")
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port (auto detection)")
                .short("l")
                .long("line")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("baud")
                .help("The baud rate to connect at (default 9600)")
                .short("s")
                .long("speed")
                .takes_value(true)
                .default_value("9600")
        )
        .arg(
            Arg::with_name("read file")
                .help("Output text from file  (e.g. /tmp/config.txt)")
                .short("r")
                .long("read")
                .takes_value(true)
        );

    let matches = app.get_matches();


    // If "read file (-r)" is specified
    // Output text from the file
    if let Some(path) = matches.value_of("read file") {
        use sist::posix::read;

        read::run(&path);


    // Else REPL start
    } else {
        use sist::posix::repl;

        // If "port (-l)" is specified
        let port_name = if let Some(name) = matches.value_of("port") {
            name.to_string()
        } else {
            available_ports().expect("No serial port")[0].port_name.to_string()
        };

        // If "baudrate (-s)" is specified
        let baud_rate = matches.value_of("baud").expect("No baud rate");


        let mut settings: SerialPortSettings = Default::default();
        settings.timeout = Duration::from_millis(10);
        if let Ok(rate) = baud_rate.parse::<u32>() {
            settings.baud_rate = rate;
        } else {
            eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
            ::std::process::exit(1);
        }


        repl::run(port_name, settings);

        println!("\nDisconnected.");
    }
}

