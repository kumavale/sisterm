# sisterm
<!-- ![stable](https://img.shields.io/badge/build-failing-critical.svg) -->
![stable](https://img.shields.io/badge/build-passing-success.svg)
![version](https://img.shields.io/badge/version-1.4.1--rc-success.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
  
sisterm(`sist`) is Simplistic serial console for Router, Switch and Firewall.  
![demo](https://user-images.githubusercontent.com/29778890/53171080-183f4400-3625-11e9-8204-83c20dcc6a3f.gif)


## Usage
If using WSL, execute `specificCOM.bat` with cmd or check with the device manager.  
If using Linux, execute `dmesg | grep USB`. by way of example only.  
After that `chmod 666 /path/to/port`.  
```
Usage: sist [-l SERIAL_PORT] [-s BAUDRATE] [-r /path/to/file]
            [-w /path/to/LOG] [-c /path/to/config] [-t] [-a] [-n] [-h] [-v]
Options:
  -h,--help     Show this help message and exit
  -v,--version  Show sisterm version and exit
  -l port       Use named device    (e.g. /dev/ttyS0)
  -s speed      Use given speed     (default 9600)
  -r path       Output log file     (e.g. /tmp/config.txt)
  -w path       Saved log           (e.g. /tmp/sist.log)
  -t            Add timestamp to log
  -a            Append to log       (default overwrite)
  -n            Without color
  -c path       Specification of config file  (e.g. /tmp/for_cisco.conf)
* -p IPAddress  Telnet beta   Many bugs!!

Commands:
  ~           Terminate the conversation
```
(e.g. `sist -l /dev/ttyS0 -s 9600 -t -a -w /tmp/sist.log`)  


## Installation
Once you have it set up, a simple `make && sudo make install` will compile sisterm and install it into `/usr/local/bin`.  
After that `cp sist.conf.example $HOME/sist.conf`.  


## Uninstall
`sudo make uninstall` after remove this directory  


## Customizing
> [NAME].color = [COLOR]  
> [NAME].regex = [REGEX]  

POSIX Extended Regular Expression Syntax  
Only lines beginning with '#' are comments.  
The maximum length of one line is 2048 characters.  

If the color length is 6 => 24bit color (000000~FFFFFF)  
If the color length is 3 =>  8bit color (000~255)  

```(e.g.)  
HOGE.color = RED
HOGE.regex = fuga
abc_1.color = 00FF00
abc_1.regex = ^(aaa|bbb|ccc)$
FGrgb_BGrgb_UB.color = \033[38;2;255;0;0;48;2;255;255;255;4m
FGrgb_BGrgb_UB.regex = ^sisterm$
```
```
# Color example
#  * RED
#  * 001
#  * FF0000
#  * \e[31m
#  * \033[31m
#  * \x1b[31m
#  * \033[38;5;1m
#  * \033[38;2;255;0;0m
#  * \033[38;2;255;0;0;48;2;0;128;128;4m
# 
# Predefined colors
#  * BLACK
#  * RED
#  * GREEN
#  * YELLOW
#  * BLUE
#  * MAGENTA
#  * CYAN
#  * WHITE
```


## Environment
* Windows10 ( WSL, linux on (VirtualBox|VMware) )  
* Linux


## License
MIT


## Note
1. Even if it is colored, IOS may not support that command.  
2. Hihlight colors are sloppy at the moment.  
