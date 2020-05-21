# sisterm
[![Actions Status](https://github.com/kumavale/sisterm/workflows/Build/badge.svg)](https://github.com/kumavale/sisterm/actions)
![version](https://img.shields.io/badge/version-2.0.0-success.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
  
sisterm(`sist`) is simplistic terminal with syntax hylight for network device.  
![Screenshot](https://user-images.githubusercontent.com/29778890/82607032-087d2980-9bf3-11ea-8b47-bbf25bcc16e2.png)  



## Command-line options
```
USAGE:
    sist [FLAGS] [OPTIONS]

FLAGS:
    -n, --no-color      Without color
    -t, --time-stamp    Add timestamp to log
    -a, --append        Append to log  (default overwrite)
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -l, --line <port>             The device path to a serial port  (auto detection)
    -s, --speed <baud>            The baud rate to connect at [default: 9600]
    -r, --read <read file>        Output text from file
    -w, --write <write file>      Saved log
    -c, --config <config file>    Specify configuration file [default: sisterm.toml]
```


## Installation


## Building
sisterm is written in Rust, so you'll need to grab a Rust installation in order to compile it. sisterm compiles with Rust 1.43.0 (stable) or newer.  

```.sh
$ git clone https://github.com/kumavale/sisterm
$ cd sisterm
$ cargo build --release
```

## Customizing

sisterm configuration file is written in [TOML](https://github.com/toml-lang/toml) format.  
Its syntax is similar to Perl-style regular expressions, but lacks a few features like look around and backreferences.  
For more specific details on the API for regular expressions, please see the documentation for the [Regex](https://docs.rs/regex/1.3.7/regex/struct.Regex.html) type.  

```.toml
[[colorings]]
color = "String"       # required
regex = "String"       # required
underlined = Boolean   # option
```

```.sh
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


## Environment
* Linux (WSL1 is not support)
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
