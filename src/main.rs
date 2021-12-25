#[macro_use]
extern crate clap;
extern crate sisterm;

use sisterm::flag;
use sisterm::setting;

use std::env;
use std::time::Duration;

use clap::{App, AppSettings, Arg, SubCommand};
use serialport::available_ports;

fn main() {

    let matches = build_app().get_matches();

    #[cfg(windows)]
    enable_ansi_support();

    // Telnet
    if let Some(matches) = matches.subcommand_matches("telnet") {
        use sisterm::telnet;

        // Hostname
        let host = matches.values_of("host[:port]").unwrap().collect::<Vec<_>>().join(":");

        // Parse arguments
        let (flags, params) = parse_arguments(matches);

        // Login user
        let login_user = matches.value_of("login_user");

        telnet::run(&host, flags, params, login_user);

        println!("\n\x1b[0mDisconnected.");

    // TCP connection witout telnet
    } else if let Some(matches) = matches.subcommand_matches("tcp") {
        use sisterm::tcp;

        // Hostname
        let host = matches.values_of("host:port").unwrap().collect::<Vec<_>>().join(":");

        // Parse arguments
        let (flags, params) = parse_arguments(matches);

        tcp::run(&host, flags, params);

        println!("\n\x1b[0mDisconnected.");

    } else {
        // Parse arguments
        let (flags, params) = parse_arguments(&matches);

        // If "read file (-r)" is specified
        // Output text from the file
        if let Some(path) = matches.value_of("read file") {
            use sisterm::file_read;

            file_read::run(path, flags, params);


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

            let baud_rate = match baud_rate.parse::<u32>() {
                Ok(br) => br,
                Err(_) => {
                    eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
                    std::process::exit(1);
                }
            };
            let settings = serialport::SerialPortSettings {
                baud_rate,
                timeout: Duration::from_millis(10),
                ..Default::default()
            };

            serial::run(port_name, settings, flags, params);

            println!("\n\x1b[0mDisconnected.");
        }
    }
}

fn parse_arguments(matches: &clap::ArgMatches) -> (flag::Flags, Option<setting::Params>) {
    use chrono::Local;

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
    let timestamp = if let Some(ref params) = params {
        if timestamp { true } else { params.timestamp }
    } else {
        timestamp
    };

    // Append flag
    let append = matches.is_present("append");

    // CRLF flag
    let crlf = matches.is_present("crlf");
    let crlf = if let Some(ref params) = params {
        if crlf { true } else { params.crlf }
    } else {
        crlf
    };

    // Hexdumo flag
    let hexdump = matches.is_present("hexdump");

    // Debug mode flag
    let debug = if let Some(ref params) = params {
        params.debug
    } else {
        false
    };

    // If "write file (-w)" is specified
    let write_file = matches.value_of("write file");
    let write_file = if let Some(write_file) = write_file {
        Some(write_file.to_string())
    } else if let Some(ref params) = params {
        if params.auto_save_log {
            #[cfg(windows)]
            let fmt = format!("{}\\{}", params.log_destination.trim_end_matches('\\'), params.log_format);
            #[cfg(not(windows))]
            let fmt = format!("{}/{}", params.log_destination.trim_end_matches('/'), params.log_format);
            Some(Local::now().format(&fmt).to_string())
        } else {
            None
        }
    } else {
        None
    };

    // Setting flags
    let flags = flag::Flags::new(nocolor, timestamp, append, crlf, hexdump, debug, write_file);

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
            .conflicts_with_all(&[
                "port", "baud", "write file", "timestamp", "append", "crlf"
            ])
        )
        .arg(Arg::with_name("write file")
            .help("Saved log")
            .short("w")
            .long("write")
            .value_name("FILE")
            .takes_value(true)
            .global(true)
        )
        .arg(Arg::with_name("config file")
            .help(config_file_help_message())
            .short("c")
            .long("config")
            .value_name("FILE")
            .takes_value(true)
            .global(true)
        )
        .arg(Arg::with_name("nocolor")
            .help("Without color")
            .short("n")
            .long("no-color")
            .global(true)
        )
        .arg(Arg::with_name("timestamp")
            .help("Add timestamp to log")
            .short("t")
            .long("time-stamp")
            .global(true)
        )
        .arg(Arg::with_name("append")
            .help("Append to log  (default overwrite)")
            .short("a")
            .long("append")
            .global(true)
        )
        .arg(Arg::with_name("crlf")
            .help("Send '\\r\\n' instead of '\\r'")
            .short("i")
            .long("instead-crlf")
            .global(true)
        )
        .arg(Arg::with_name("hexdump")
            .help("Prints in hex")
            .short("x")
            .long("hexdump")
            .global(true)
        )
        .subcommands(vec![SubCommand::with_name("telnet")
            .about("Login to remote system host with telnet")
            .usage("sist telnet [FLAGS] [OPTIONS] <HOST[:PORT]>")
            .setting(AppSettings::DeriveDisplayOrder)
            .arg(Arg::with_name("host[:port]")
                .help("Port number can be omitted. Then 23")
                .value_name("HOST[:PORT]")
                .takes_value(true)
                .required(true)
                .min_values(1)
                .max_values(2)
                //.hidden(true)
            )
            .arg(Arg::with_name("login_user")
                .help("Specify login user")
                .short("l")
                .long("login-user")
                .value_name("USERNAME")
                .takes_value(true)
            ),
        SubCommand::with_name("tcp")
            .about("TCP connection without telnet")
            .setting(AppSettings::DeriveDisplayOrder)
            .arg(Arg::with_name("host:port")
                .help("Host and port number")
                .takes_value(true)
                .required(true)
                .min_values(1)
            ),
        ])
}

fn get_config_file_path() -> String {
    #[cfg(windows)]
    return format!("{}/sisterm/config.toml",
        if let Ok(ref user) = env::var("LOCALAPPDATA") { user } else { "%LOCALAPPDATA%" } );

    #[cfg(not(windows))]
    return format!("{}/.config/sisterm/config.toml",
        if let Ok(ref home) = env::var("HOME") { home } else { "$HOME" } );
}

fn config_file_help_message() -> &'static str {
    #[cfg(windows)]
    return "Specify configuration file\n[default %LOCALAPPDATA%/sisterm/config.toml]";

    #[cfg(not(windows))]
    return "Specify configuration file\n[default $HOME/.config/sisterm/config.toml]";
}

#[cfg(windows)]
fn enable_ansi_support() {
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_INPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    unsafe {
        let input_handle = GetStdHandle(STD_INPUT_HANDLE);
        let mut console_mode: u32 = 0;

        if input_handle == INVALID_HANDLE_VALUE {
            return;
        }

        if GetConsoleMode(input_handle, &mut console_mode) != 0 {
            if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
                SetConsoleMode(input_handle, console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
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
    fn test_config_file_help_message() {
        #[cfg(windows)]
        assert_eq!(
            config_file_help_message(),
            "Specify configuration file\n[default %LOCALAPPDATA%/sisterm/config.toml]"
        );

        #[cfg(not(windows))]
        assert_eq!(
            config_file_help_message(),
            "Specify configuration file\n[default $HOME/.config/sisterm/config.toml]"
        );
    }
}
