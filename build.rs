use std::fmt;
use std::env;
use std::fs;
use std::path::Path;

use lazy_static::lazy_static;

static CONFIG_FILENAME: &str = "config.toml";

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

fn main() {
    // Generate configuration file
    match generate() {
        Ok(None) => (), // Already exists configuration file
        Ok(Some(path)) => println!("Generated config file --> {}", path),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        },
    }
}

// Generate configration file
// If windows => %USERPROFILE%/AppData/Local/sisterm/config.toml
//    other   => $HOME/.config/sisterm/config.toml
// Returns the path to that configuration file
fn generate() -> Result<Option<String>, std::io::Error> {
    // Path to the config.toml
    let config_file_path = format!("{}{}", *PARENT_PATH, CONFIG_FILENAME);

    // Check for file existence
    if Path::new(&config_file_path).exists() {
        return Ok(None);
    }

    // Create directory
    fs::create_dir_all(&*PARENT_PATH)?;

    // Copies the contents
    fs::copy(CONFIG_FILENAME, &config_file_path)?;

    Ok(Some(config_file_path))
}
