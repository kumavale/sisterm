use sisterm::flag;
use sisterm::setting;

use std::env;

use clap::{command, Subcommand, Parser};
use serialport::available_ports;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The device path to a serial port  (auto detection)
    #[arg(short = 'l', long = "line", value_name = "PORT")]
    port: Option<String>,

    /// The baud rate to connect at
    #[arg(short = 's', long = "speed", value_name = "BAUD", default_value = "9600")]
    baud: Option<String>,

    /// Output text from file
    #[arg(short = 'r', long = "read", value_name = "FILE", conflicts_with_all = ["port", "baud", "write_file", "timestamp", "append", "crlf"])]
    read_file: Option<String>,

    /// Saved log
    #[arg(short = 'w', long = "write", value_name = "FILE", global = true)]
    write_file: Option<String>,

    // config
    #[arg(short = 'c', long = "config", value_name = "FILE", global = true, help = config_file_help_message())]
    config_file: Option<String>,

    /// Without color
    #[arg(short = 'n', long = "no-color", global = true)]
    nocolor: bool,

    /// Add timestamp to log
    #[arg(short = 't', long = "time-stamp", global = true)]
    timestamp: bool,

    /// Append to log  (default overwrite)
    #[arg(short = 'a', long = "append", global = true)]
    append: bool,

    /// Send '\r\n' instead of '\r'
    #[arg(short = 'i', long = "instead-crlf", global = true)]
    crlf: bool,

    /// Prints in hex
    #[arg(short = 'x', long = "hexdump", global = true)]
    hexdump: bool,

    #[command(subcommand)]
    command: Option<Command>,
}


#[derive(Debug, Subcommand)]
enum Command {
    /// Login to remote system host with ssh
    Ssh {
        /// Port number can be omitted. Then 22
        #[arg(value_name = "HOST[:PORT]", required = true, num_args = 1..=2)]
        host: Vec<String>,

        /// Specify login user
        #[arg(short = 'l', long = "login-user", value_name = "USERNAME")]
        login_user: Option<String>,
    },

    /// Login to remote system host with telnet
    Telnet {
        /// Port number can be omitted. Then 23
        #[arg(value_name = "HOST[:PORT]", required = true, num_args = 1..=2)]
        host: Vec<String>,

        /// Specify login user
        #[arg(short = 'l', long = "login-user", value_name = "USERNAME")]
        login_user: Option<String>,
    },

    /// TCP connection without telnet
    Tcp {
        /// Host and port number
        #[arg(value_name = "HOST:PORT", required = true, num_args = 1..=2)]
        host: Vec<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    #[cfg(windows)]
    enable_ansi_support();

    // SSH
    if let Some(Command::Ssh { host, login_user }) = &args.command {
        use sisterm::ssh;

        // Hostname
        let host = host.join(":");

        // Parse arguments
        let (flags, params) = parse_arguments(&args);

        ssh::run(&host, flags, params, login_user.as_deref()).await;

        println!("\n\x1b[0mDisconnected.");

    // Telnet
    } else if let Some(Command::Telnet { host, login_user }) = &args.command {
        use sisterm::telnet;

        // Hostname
        let host = host.join(":");

        // Parse arguments
        let (flags, params) = parse_arguments(&args);

        telnet::run(&host, flags, params, login_user.as_deref()).await;

        println!("\n\x1b[0mDisconnected.");

    // TCP connection witout telnet
    } else if let Some(Command::Tcp { host }) = &args.command {
        use sisterm::tcp;

        // Hostname
        let host = host.join(":");

        // Parse arguments
        let (flags, params) = parse_arguments(&args);

        tcp::run(&host, flags, params).await;

        println!("\n\x1b[0mDisconnected.");

    } else {
        // Parse arguments
        let (flags, params) = parse_arguments(&args);

        // If "read file (-r)" is specified
        // Output text from the file
        if let Some(path) = args.read_file {
            use sisterm::file_read;

            file_read::run(&path, flags, params);


        // Serialport
        } else {
            use sisterm::serial;

            let (port_name, baud_rate) = if let Some(params) = &params {
                // If "port (-l)" is specified
                let port_name = if let Some(port) = args.port {
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
                } else if let Some(baud) = &args.baud {
                    baud
                } else {
                    panic!("No baud rate");
                }.to_string();

                (port_name, baud_rate)
            } else {
                // If "port (-l)" is specified
                let port_name = if let Some(port) = args.port {
                    port.to_string()
                } else {
                    match available_ports() {
                        Ok(port) if !port.is_empty() => port[0].port_name.to_string(),
                        _ => panic!("No serial port"),
                    }
                };
                // If "baudrate (-s)" is specified
                let baud_rate = args.baud.expect("No baud rate");

                (port_name, baud_rate.to_string())
            };

            let baud_rate = match baud_rate.parse::<u32>() {
                Ok(br) => br,
                Err(_) => {
                    eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
                    std::process::exit(1);
                }
            };

            serial::run(port_name, baud_rate, flags, params).await;

            println!("\n\x1b[0mDisconnected.");
        }
    }
}

fn parse_arguments(args: &Args) -> (flag::Flags, Option<setting::Params>) {
    use chrono::Local;

    // If "config file (-c)" is specified
    let config_file = if let Some(file) = &args.config_file {
        file.to_string()
    } else {
        get_config_file_path()
    };

    // Parse configuration file
    let params = setting::Params::new(&config_file);

    // Color display flag
    let nocolor = args.nocolor;

    // Timestamp flag
    let timestamp = args.timestamp;
    let timestamp = if let Some(ref params) = params {
        if timestamp { true } else { params.timestamp }
    } else {
        timestamp
    };

    // Append flag
    let append = args.append;

    // CRLF flag
    let crlf = args.crlf;
    let crlf = if let Some(ref params) = params {
        if crlf { true } else { params.crlf }
    } else {
        crlf
    };

    // Hexdumo flag
    let hexdump = args.hexdump;

    // Debug mode flag
    let debug = if let Some(ref params) = params {
        params.debug
    } else {
        false
    };

    // If "write file (-w)" is specified
    let write_file = &args.write_file;
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

fn get_config_file_path() -> String {
    #[cfg(windows)]
    const VAR_AND_FALLBACK: (&str, &str) = ("LOCALAPPDATA", "%LOCALAPPDATA%");
    #[cfg(not(windows))]
    const VAR_AND_FALLBACK: (&str, &str) = ("HOME", "$HOME");
    let var = env::var(VAR_AND_FALLBACK.0);

    format!("{}/sisterm/config.toml",
        if let Ok(ref home) = var { home } else { VAR_AND_FALLBACK.1 })
}

fn config_file_help_message() -> &'static str {
    #[cfg(windows)]
    return "Specify configuration file\n[default %LOCALAPPDATA%/sisterm/config.toml]";

    #[cfg(not(windows))]
    return "Specify configuration file\n[default $HOME/.config/sisterm/config.toml]";
}

#[cfg(windows)]
#[allow(clippy::collapsible_if)]
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
