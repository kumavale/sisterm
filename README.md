# sisterm

[![Actions Status](https://github.com/kumavale/sisterm/workflows/Build/badge.svg)](https://github.com/kumavale/sisterm/actions)
[![Crate](https://img.shields.io/crates/v/sisterm.svg)](https://crates.io/crates/sisterm)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
  
sisterm(`sist`) is a simple terminal with syntax highlighting which supports:  
* Serial port connections
* TCP/IP (telnet) connections
* Log replaying

![Screenshot](https://user-images.githubusercontent.com/29778890/82722563-e246af00-9d02-11ea-97d1-fc5581b4bf21.png)  


## Command-line options

```
USAGE:
    sist [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -n, --no-color        Without color
    -t, --time-stamp      Add timestamp to log
    -a, --append          Append to log  (default overwrite)
    -i, --instead-crlf    Send '\r\n' instead of '\r'
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -l, --line <PORT>      The device path to a serial port  (auto detection)
    -s, --speed <BAUD>     The baud rate to connect at [default: 9600]
    -r, --read <FILE>      Output text from file
    -w, --write <FILE>     Saved log
    -c, --config <FILE>    Specify configuration file
                           [default $HOME/.config/sisterm/config.toml]

SUBCOMMANDS:
    telnet      Login to remote system host with telnet
    tcp         TCP connection without telnet
    generate    Generate configuration file
    help        Prints this message or the help of the given subcommand(s)
```


## Installation

The binary name for sisterm is `sist`.  
sisterm is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/) in order to compile it. sisterm compiles with Rust 1.43.0 (stable) or newer. Although sisterm may work with older versions.  

```.sh
$ cargo install sisterm
```

After that, execute the following to generate a configuration file.  

```.sh
$ sist generate
```


## Building

```.sh
$ git clone https://github.com/kumavale/sisterm
$ cd sisterm
$ cargo build --release
```

## Customizing

* Windows  
    `%LOCALAPPDATA%\sisterm\config.toml`  
* Linux  
    `$HOME/.config/sisterm/config.toml`  

sisterm configuration file is written in [TOML](https://github.com/toml-lang/toml) format.  
Its syntax is similar to Perl-style regular expressions, but lacks a few features like look around and backreferences.  
For more specific details on the API for regular expressions, please see the documentation for the [Regex](https://docs.rs/regex) type.  

```
[[colorings]]
color = "String"             # required
regex = ["String", ...]      # required
underlined = Boolean         # option
ignore_whitespace = Boolean  # option
```

### Color syntax

```.toml
# Color example
#   RED
#   001
#   FF0000
#   #FF0000
#   (255, 0, 0)
#
# Predefined colors
#   BLACK
#   RED
#   GREEN
#   YELLOW
#   BLUE
#   MAGENTA
#   CYAN
#   WHITE
```

### Example

```.toml
# string
[[colorings]]
color = "184"
regex = ["(\".*\")|('.*')|(\".*)|('.*)"]
ignore_whitespace = true

# positive
[[colorings]]
color = "GREEN"
regex = ["(?i)yes|up|enable|enabled|active(?-i)"]

# negative
[[colorings]]
underlined = true
color = "(255, 0, 0)"
regex = ["unassigned|disable|disabled|deny|shutdown|down|administratively|none"]
```


## Environment

* Linux (Serialport is not available on WSL1)
* Windows


## Dependencies

For GNU Linux `pkg-config` and `libudev` headers are required  
* Ubuntu: `sudo apt install pkg-config libudev-dev`
* Fedora: `sudo dnf install pkgconf-pkg-config systemd-devel`
* Other: Some Linux distros are providing pkgconf.org's `pkgconf` package instead of freedesktop.org's `pkg-config`.


## License

MIT


## Note

* If sisterm failed to open your COM port, it may be because the user who ran sisterm does not have privileges to access it. To give yourself access privileges, run: `sudo chmod 666 /path/to/serialport`, running your program as the super-user (root), or making your program set-userid so that it runs as the owner of the device file.  
* If the characters couldn't be sent, try the `--instead-crlf` option  

