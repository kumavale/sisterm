# sisterm
<!-- ![stable](https://img.shields.io/badge/build-passing-success.svg) -->
![disable](https://img.shields.io/badge/build-failing-critical.svg)
![version](https://img.shields.io/badge/version-1.5.0-success.svg)
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
            [-p IPAddress[:port]]
Options:
  -h,--help          Show this help message and exit
  -v,--version       Show sisterm version and exit
  -l port            Use named device    (e.g. /dev/ttyS0)
  -s speed           Use given speed     (default 9600)
  -r path            Output log file     (e.g. /tmp/config.txt)
  -w path            Saved log           (e.g. /tmp/sist.log)
  -t                 Add timestamp to log
  -a                 Append to log       (default overwrite)
  -n                 Without color
  -c path            Specification of config file  (e.g. /tmp/for_cisco.conf)
* -p address[:port]  Telnet beta   Many bugs!!

Commands:
  ~           Terminate the conversation
```
(e.g. `sist -l /dev/ttyS0 -s 9600 -t -a -w /tmp/sist.log`)  


## Installation
Once you have it set up, a simple `make && sudo make install` will compile sisterm and install it into `/usr/local/bin`.  
After that `cp sist.conf $HOME/sist.conf` or copy the configuration file (`sist.conf`) from my [Gist](https://gist.github.com/kumavale/bbadf8e9ac47a478d00f532e15c7c7bf) to your home directory.  


## Uninstall
`sudo make uninstall` after remove this directory  
And remove `sist.conf` in HOME directory  


## Customizing
> [NAME].color =  [COLOR]  
> [NAME].color += [COLOR]  
> [NAME].regex =  [REGEX]  

POSIX Extended Regular Expression Syntax  
Only lines beginning with '#' are comments.  
The maximum length of one line is 2048 characters.  

If the color length is 6 => 24bit color (000000\~FFFFFF)  
If the color length is 3 =>  8bit color (000\~255)  

```.sh
# Examples
HOGE.color = RED
HOGE.regex = fuga
FGrgb_BGrgb_UB.color = \033[38;2;255;0;0;48;2;017;221;255;4m
FGrgb_BGrgb_UB.regex = ^sisterm$
add.color = \033[38;5;1m
add.regex = piyo
add.color += \033[48;5;2m

others.regex = .*
others.color = 99FF99
```
```.sh
# Color example
#   RED
#   001
#   FF0000
#   #FF0000
#   \e[31m
#   \033[31m
#   \x1b[31m
#   \033[38;5;1m
#   \033[38;2;255;0;0m
#   \033[38;2;255;0;0;48;2;0;128;128;4m
 
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
* Linux


## License
MIT


## Note
<a name="note-1"></a>
1. Standard input looks double in appearance.  
2. Other than ASCII code can't be displayed.
