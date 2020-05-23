use std::fmt;
use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, BufWriter, Write};

use lazy_static::lazy_static;

static CONTENTS: &[u8] =
br#"#
#  sisterm setting file
#
#  Regular expressions support RE2 Syntax
#
#  [[colorings]]
#  color = "String"       # required
#  regex = "String"       # required
#  underlined = Boolean   # option
#
#  Color example
#   * RED           [Uppercase]
#   * 001           [Decimal number]
#   * FF0000        [Hexadecimal]
#   * (255, 0, 0)   [Decimal number]
#
#  Predefined colors
#   * BLACK
#   * RED
#   * GREEN
#   * YELLOW
#   * BLUE
#   * MAGENTA
#   * CYAN
#   * WHITE
#


# Specify default serial port
# port  = "/dev/ttyS0"

# Specify default baud rate
# speed = "9600"


# positive
[[colorings]]
color = "GREEN"
regex = "^(yes|YES|up|enable|enabled|active)$"

# string
[[colorings]]
color = "184"
regex = "(\".*\")|('.*')"

# emphansis
[[colorings]]
color = "MAGENTA"
regex = "^(no|NO|not|confirm|warning|warnings|failed|failures|error|errors|crash)$"

# interface
[[colorings]]
color = "CYAN"
regex = "^((Tengigabit|Gigabit|Fast)?Ethernet|(Fa|Gi)).*"

# negative
[[colorings]]
underlined = true
color = "RED"
regex = "^(unassigned|disable|disabled|deny|shutdown|down|administratively|none)$"

# ipv4_net
[[colorings]]
color = "YELLOW"
regex = "^(2[0-4][0-9]|1[0-9]{2}|[1-9][0-9]|[1-8])[.]((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$"

# ipv4_sub
[[colorings]]
color = "BLUE"
regex = "^((25[0-5]|24[89])[.])((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$"

# ipv4_wild
[[colorings]]
color = "MAGENTA"
regex = "^(0[.])((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$"

"#;

lazy_static! {
    static ref PARENT_PATH: String = {
        if cfg!(windows) {
            format!("{}/AppData/Local/sisterm/", env::var("USERPROFILE").unwrap())
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
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.to_lowercase().trim_end() {
            "y" | "yes" => (),  // continue
            _ => return Err("".to_string()),
        }
    }

    // Create directory
    match fs::create_dir_all(&*PARENT_PATH) {
        Ok(_) => (),  // continue
        Err(e) => return Err(e.to_string()),
    }

    // Write contents to file
    let mut f = BufWriter::new(fs::File::create(&config_file_path).unwrap());
    f.write_all(CONTENTS).unwrap();
    f.flush().unwrap();

    Ok(config_file_path)
}
