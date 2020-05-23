# sisterm

[![Actions Status](https://github.com/kumavale/sisterm/workflows/Build/badge.svg)](https://github.com/kumavale/sisterm/actions)
![version](https://img.shields.io/badge/version-2.0.0-success.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
  
sisterm(`sist`) is a simple terminal with syntax highlighting for network devices  which supports:  
* Serial port connections
* TCP/IP (telnet) connections
* Log replaying

![Screenshot](https://user-images.githubusercontent.com/29778890/82722563-e246af00-9d02-11ea-97d1-fc5581b4bf21.png)  


## Command-line options

```
USAGE:
    sist [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -n, --no-color      Without color
    -t, --time-stamp    Add timestamp to log
    -a, --append        Append to log  (default overwrite)
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -l, --line <PORT>      The device path to a serial port  (auto detection)
    -s, --speed <BAUD>     The baud rate to connect at [default: 9600]
    -r, --read <FILE>      Output text from file
    -w, --write <FILE>     Saved log
    -c, --config <FILE>    Specify configuration file
                           [default $HOME/.config/sisterm/config.toml]

SUBCOMMANDS:
    telnet      Login to remote system host with telnet
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
    `%USERPROFILE%\AppData\Local\sisterm\config.toml`  
* Linux  
    `$HOME/.config/sisterm/config.toml`  

sisterm configuration file is written in [TOML](https://github.com/toml-lang/toml) format.  
Its syntax is similar to Perl-style regular expressions, but lacks a few features like look around and backreferences.  
For more specific details on the API for regular expressions, please see the documentation for the [Regex](https://docs.rs/regex/1.3.7/regex/struct.Regex.html) type.  

```
[[colorings]]
color = "String"       # required
regex = "String"       # required
underlined = Boolean   # option
```

### Color syntax

```.toml
# Color example
#   RED
#   001
#   FF0000
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
regex = "(\".*\")|('.*')"

# positive
[[colorings]]
color = "GREEN"
regex = "^(yes|YES|up|enable|enabled|active)$"

# negative
[[colorings]]
color = "(255, 0, 0)"
regex = "^(unassigned|disable|disabled|deny|shutdown|down|administratively|none)$"
underlined = true
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

* If sisterm failed to open your COM port, it may be because the user who ran sisterm does not have privileges to access it. To give yourself access privileges, run: `sudo chmod 666 /path/to/serialport`
