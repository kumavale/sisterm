use std::fmt;
use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, BufWriter, Write};

use lazy_static::lazy_static;

static CONTENTS: &[u8] =
br#"##
## This is the configuration file for the sisterm
##
## Its syntax is similar to Perl-style regular expressions, but lacks a few
## features like look around and backreferences.
## For more specific details on the API for regular expressions, please see
## the documentation for the Regex(https://docs.rs/regex) type.
##
## [[colorings]]
## color = "String"       # required
## regex = "String"       # required
## underlined = Boolean   # option
##
## Color example
##  * RED           [Uppercase]
##  * 001           [Decimal number]
##  * FF0000        [Hexadecimal]
##  * #FF0000       [Hexadecimal]
##  * (255, 0, 0)   [Decimal number]
##
## Predefined colors
##  * BLACK
##  * RED
##  * GREEN
##  * YELLOW
##  * BLUE
##  * MAGENTA
##  * CYAN
##  * WHITE
##


## Specify default serial port
#port  = "/dev/ttyS0"

## Specify default baud rate
#speed = "9600"

## Specify default Send '\r' instead of '\n'
#instead_cr = true

## Specify read buffer size
#read_buf_size = 16

## Specify TCP connect timeout
#tcp_connect_timeout = 5

## Specify timestamp format
## See below for detailed documentation
## https://docs.rs/chrono/0.4.11/chrono/format/strftime/index.html
#timestamp_format = "[%Y-%m-%d %H:%M:%S %Z] "


##############################
#_/_/_/_/_/_/_/_/_/_/_/_/_/_/#
#_/_/  SAMPLE COLORINGS  _/_/#
#_/_/_/_/_/_/_/_/_/_/_/_/_/_/#
##############################

# comments
[[colorings]]
color = "(128, 150, 200)"
regex = ["(//.*)|(/\\*.*\\*/)|(/\\*.*)"]  # C style

# positive
[[colorings]]
color = "GREEN"
regex = ["( |^)((?i)yes|up|enable|enabled|active(?-i))( |$)"]

# string
[[colorings]]
color = "184"
regex = ["(\".*\")|('.*')"]

# emphansis
[[colorings]]
color = "MAGENTA"
regex = ["( |^)(not?|confirm|warning|warnings|failed|failures|errors?|crash)( |$)"]

# interface
[[colorings]]
color = "CYAN"
regex = ["(([Tt]engigabit|[Gg]igabit|[Ff]ast)?[Ee]thernet|[Ff]a|[Gg]i)\\d+/\\d+"]

# negative
[[colorings]]
underlined = true
color = "RED"
regex = ["unassigned|disable|disabled|deny|shutdown|down|administratively|none"]

# ipv4_net
[[colorings]]
color = "YELLOW"
regex = ["([^0-9]|^)(2[0-4][0-9]|1[0-9]{2}|[1-9][0-9]|[1-8])\\.((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])\\.){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])"]

# ipv4_sub
[[colorings]]
color = "BLUE"
regex = ["((25[0-5]|24[89])\\.)((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])\\.){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])"]

# ipv4_wild
[[colorings]]
color = "MAGENTA"
regex = ["(0\\.)((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])\\.){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])"]

"#;

lazy_static! {
    static ref PARENT_PATH: String = {
        if cfg!(windows) {
            format!("{}/sisterm/", env::var("LOCALAPPDATA").unwrap())
        } else {
            format!("{}/.config/sisterm/", env::var("HOME").unwrap())
        }
    };
}

impl fmt::Display for PARENT_PATH {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// Generate configration file
// If windows => %USERPROFILE%/AppData/Local/sisterm/config.toml
//    other   => $HOME/.config/sisterm/config.toml
// Returns the path to that configuration file
pub fn generate() -> Result<String, String> {
    // Path to the config.toml
    let config_file_path = format!("{}config.toml", *PARENT_PATH);

    // Check for file existence
    if Path::new(&config_file_path).exists() {
        let mut input = String::new();
        println!("configration file is already exists!");
        print!("Overwrite? [y/N]");
        io::stdout().flush().map_err(|e| e.to_string())?;
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        match input.to_lowercase().trim_end() {
            "y" | "yes" => (),  // continue
            _ => std::process::exit(0),
        }
    }

    // Create directory
    fs::create_dir_all(&*PARENT_PATH).map_err(|e| e.to_string())?;

    // Write contents to file
    let mut f = BufWriter::new(fs::File::create(&config_file_path).map_err(|e| e.to_string())?);
    f.write_all(CONTENTS).map_err(|e| e.to_string())?;
    f.flush().map_err(|e| e.to_string())?;

    Ok(config_file_path)
}
