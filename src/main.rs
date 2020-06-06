#[macro_use]
extern crate clap;
extern crate sisterm;

use sisterm::flag;
use sisterm::setting;

use std::env;
use std::time::Duration;

use clap::{App, AppSettings, Arg, SubCommand};
use serialport::{available_ports, SerialPortSettings};

fn main() {

    let matches = build_app().get_matches();

    // Generate configuration file
    if matches.subcommand_matches("generate").is_some() {
        use sisterm::config;

        match config::generate() {
            Ok(path) => {
                println!("Complete! --> {}", path);
                std::process::exit(0);
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        }
    }

    // Telnet
    if let Some(ref matches) = matches.subcommand_matches("telnet") {
        use sisterm::telnet;

        // Hostname
        let host = matches.value_of("host[:port]").unwrap();

        // Parse arguments
        let (flags, params) = parse_arguments(matches);

        telnet::run(host, flags, params);

        println!("\n\x1b[0mDisconnected.");

    // TCP connection witout telnet
    } else if let Some(ref matches) = matches.subcommand_matches("tcp") {
        use sisterm::tcp;

        // Hostname
        let host = matches.value_of("host:port").unwrap();

        // Parse arguments
        let (flags, params) = parse_arguments(matches);

        tcp::run(host, flags, params);

        println!("\n\x1b[0mDisconnected.");

    } else {
        // Parse arguments
        let (flags, params) = parse_arguments(&matches);

        // If "read file (-r)" is specified
        // Output text from the file
        if let Some(path) = matches.value_of("read file") {
            use sisterm::file_read;

            file_read::run(&path, flags, params);


        // Serialport
        } else {
            use sisterm::serial;

            let (port_name, baud_rate) = if let Some(params) = &params {
                // If "port (-l)" is specified
                let port_name = if let Some(port) = matches.value_of("port") {
                    port.to_string()
                } else if let Some(port) = &params.port {
                    port.to_string()
                } else {
                    match available_ports() {
                        Ok(port) if !port.is_empty() => port[0].port_name.to_string(),
                        _ => panic!("No serial port"),
                    }
                };
                // If "baudrate (-s)" is specified
                let baud_rate = if let Some(baud) = &params.speed {
                    baud
                } else if let Some(baud) = matches.value_of("baud") {
                    baud
                } else {
                    panic!("No baud rate");
                }.to_string();

                (port_name, baud_rate)
            } else {
                // If "port (-l)" is specified
                let port_name = if let Some(port) = matches.value_of("port") {
                    port.to_string()
                } else {
                    match available_ports() {
                        Ok(port) if !port.is_empty() => port[0].port_name.to_string(),
                        _ => panic!("No serial port"),
                    }
                };
                // If "baudrate (-s)" is specified
                let baud_rate = matches.value_of("baud").expect("No baud rate");

                (port_name, baud_rate.to_string())
            };

            let mut settings: SerialPortSettings = Default::default();
            settings.timeout = Duration::from_millis(10);
            if let Ok(rate) = baud_rate.parse::<u32>() {
                settings.baud_rate = rate;
            } else {
                eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
                std::process::exit(1);
            }

            serial::run(port_name, settings, flags, params);

            println!("\n\x1b[0mDisconnected.");
        }
    }
}

fn parse_arguments(matches: &clap::ArgMatches) -> (flag::Flags, Option<setting::Params>) {
    // If "config file (-c)" is specified
    let config_file = if let Some(file) = matches.value_of("config file") {
        file.to_string()
    } else {
        get_config_file_path()
    };

    // Parse configuration file
    let params = setting::Params::new(&config_file);

    // Color display flag
    let nocolor = matches.is_present("nocolor");

    // Timestamp flag
    let timestamp = matches.is_present("timestamp");

    // Append flag
    let append = matches.is_present("append");

    // Instead_CR flag
    let instead_cr = matches.is_present("instead_cr");
    let instead_cr = if let Some(ref params) = params {
        if params.instead_cr { true } else { instead_cr }
    } else {
        instead_cr
    };

    // Debug mode flag
    let debug = if let Some(ref params) = params {
        params.debug
    } else {
        false
    };

    // If "write file (-w)" is specified
    let write_file = matches.value_of("write file");

    // Setting flags
    let flags = flag::Flags::new(nocolor, timestamp, append, instead_cr, debug, write_file);

    (flags, params)
}

fn build_app() -> App<'static, 'static> {

    App::new("sisterm")
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("port")
            .help("The device path to a serial port  (auto detection)")
            .short("l")
            .long("line")
            .value_name("PORT")
            .takes_value(true)
        )
        .arg(Arg::with_name("baud")
            .help("The baud rate to connect at")
            .short("s")
            .long("speed")
            .value_name("BAUD")
            .takes_value(true)
            .default_value("9600")
        )
        .arg(Arg::with_name("read file")
            .help("Output text from file")
            .short("r")
            .long("read")
            .value_name("FILE")
            .takes_value(true)
        )
        .arg(Arg::with_name("write file")
            .help("Saved log")
            .short("w")
            .long("write")
            .value_name("FILE")
            .takes_value(true)
        )
        .arg(Arg::with_name("config file")
            .help(config_file_help_message())
            .short("c")
            .long("config")
            .value_name("FILE")
            .takes_value(true)
        )
        .arg(Arg::with_name("nocolor")
            .help("Without color")
            .short("n")
            .long("no-color")
        )
        .arg(Arg::with_name("timestamp")
            .help("Add timestamp to log")
            .short("t")
            .long("time-stamp")
        )
        .arg(Arg::with_name("append")
            .help("Append to log  (default overwrite)")
            .short("a")
            .long("append")
        )
        .arg(Arg::with_name("instead_cr")
            .help("Send '\\r' instead of '\\n'")
            .short("i")
            .long("instead-cr")
        )
        .subcommand(SubCommand::with_name("telnet")
            .about("Login to remote system host with telnet")
            .setting(AppSettings::DeriveDisplayOrder)
            .arg(Arg::with_name("host[:port]")
                .help("Port number can be omitted. Then 23")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("write file")
                .help("Saved log")
                .short("w")
                .long("write")
                .value_name("FILE")
                .takes_value(true)
            )
            .arg(Arg::with_name("config file")
                .help(config_file_help_message())
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
            )
            .arg(Arg::with_name("nocolor")
                .help("Without color")
                .short("n")
                .long("no-color")
            )
            .arg(Arg::with_name("timestamp")
                .help("Add timestamp to log")
                .short("t")
                .long("time-stamp")
            )
            .arg(Arg::with_name("append")
                .help("Append to log  (default overwrite)")
                .short("a")
                .long("append")
            )
            .arg(Arg::with_name("instead_cr")
                .help("Send '\\r' instead of '\\n'")
                .short("i")
                .long("instead-cr")
            )
        )
        .subcommand(SubCommand::with_name("tcp")
            .about("TCP connection without telnet")
            .setting(AppSettings::DeriveDisplayOrder)
            .arg(Arg::with_name("host:port")
                .help("Host and port number")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("write file")
                .help("Saved log")
                .short("w")
                .long("write")
                .value_name("FILE")
                .takes_value(true)
            )
            .arg(Arg::with_name("config file")
                .help(config_file_help_message())
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
            )
            .arg(Arg::with_name("nocolor")
                .help("Without color")
                .short("n")
                .long("no-color")
            )
            .arg(Arg::with_name("timestamp")
                .help("Add timestamp to log")
                .short("t")
                .long("time-stamp")
            )
            .arg(Arg::with_name("append")
                .help("Append to log  (default overwrite)")
                .short("a")
                .long("append")
            )
            .arg(Arg::with_name("instead_cr")
                .help("Send '\\r' instead of '\\n'")
                .short("i")
                .long("instead-cr")
            )
        )
        .subcommand(SubCommand::with_name("generate")
            .about("Generate configuration file")
        )
}

#[cfg(windows)]
fn get_config_file_path() -> String {
    format!("{}/sisterm/config.toml",
        if let Ok(ref user) = env::var("LOCALAPPDATA") { user } else { "%LOCALAPPDATA%" } )
}
#[cfg(not(windows))]
fn get_config_file_path() -> String {
    format!("{}/.config/sisterm/config.toml",
        if let Ok(ref home) = env::var("HOME") { home } else { "$HOME" } )
}

#[cfg(windows)]
fn config_file_help_message() -> &'static str {
    "Specify configuration file\n[default %LOCALAPPDATA%/sisterm/config.toml]"
}
#[cfg(not(windows))]
fn config_file_help_message() -> &'static str {
    "Specify configuration file\n[default $HOME/.config/sisterm/config.toml]"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_file_path() {
        assert_ne!(get_config_file_path(), "%LOCALAPPDATA%/sisterm/config.toml");
        assert_ne!(get_config_file_path(), "$HOME/sisterm/config.toml");
    }

    #[test]
    #[cfg(windows)]
    fn test_config_file_help_message() {
        assert_eq!(
            config_file_help_message(),
            "Specify configuration file\n[default %LOCALAPPDATA%/sisterm/config.toml]"
        );
    }

    #[test]
    #[cfg(not(windows))]
    fn test_config_file_help_message() {
        assert_eq!(
            config_file_help_message(),
            "Specify configuration file\n[default $HOME/.config/sisterm/config.toml]"
        );
    }
}
