#[macro_use]
extern crate clap;
extern crate sist;

use sist::flag;

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
                .help("The baud rate to connect at")
                .short("s")
                .long("speed")
                .takes_value(true)
                .default_value("9600")
        )
        .arg(
            Arg::with_name("read file")
                .help("Output text from file")
                .short("r")
                .long("read")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("write file")
                .help("Saved log")
                .short("w")
                .long("write")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("config file")
                .help("Specify configuration file")
                .short("c")
                .long("config")
                .takes_value(true)
                .default_value("TODO")
        )
        .arg(
            Arg::with_name("nocolor")
                .help("Without color")
                .short("n")
                .long("no-color")
        )
        .arg(
            Arg::with_name("timestamp")
                .help("Add timestamp to log")
                .short("t")
                .long("time-stamp")
        );

    let matches = app.get_matches();


    // Color display flag
    let nocolor = matches.is_present("nocolor");

    // Timestamp flag
    let timestamp = matches.is_present("timestamp");

    // If "write file (-w)" is specified
    let write_file = matches.value_of("write file");

    // If "config file (-c)" is specified
    let config_file = matches.value_of("config file").expect("Invalid file name");

    // Setting flags
    let flags = flag::Flags::new(nocolor, timestamp, write_file, config_file);


    // If "read file (-r)" is specified
    // Output text from the file
    if let Some(path) = matches.value_of("read file") {
        use sist::read;

        read::run(&path, flags);


    // Else REPL start
    } else {
        use sist::repl;

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
            std::process::exit(1);
        }


        repl::run(port_name, settings, flags);

        println!("\nDisconnected.");
    }
}

