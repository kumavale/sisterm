# sisterm
<!-- ![stable](https://img.shields.io/badge/build-passing-success.svg) -->
![disable](https://img.shields.io/badge/build-failing-red.svg)
![version](https://img.shields.io/badge/version-1.4.0--rc1-success.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
  
sisterm(`sist`) is Simplistic serial console for Router, Switch and Firewall.  
![demo](https://user-images.githubusercontent.com/29778890/53171080-183f4400-3625-11e9-8204-83c20dcc6a3f.gif)


## Usage
If using WSL, execute `specificCOM.bat` with cmd or check with the device manager.  
If using Linux, execute `dmesg | grep USB`. by way of example only.  
After that `chmod 666 /path/to/port`.  
```
Usage: sist [-l SERIAL_PORT] [-s BAUDRATE] [-r /path/to/file]
            [-w /path/to/LOG] [-t] [-a] [-n] [-h] [-v]
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
* -p IPAddress  Telnet beta   Many bugs!!

Commands:
  ~           Terminate the conversation
```
(e.g. `sist -l /dev/ttyS0 -s 9600 -t -a -w /tmp/sist.log`)  


## Installation
Once you have it set up, a simple `make && sudo make install` will compile sisterm and install it into `/usr/local/bin`.  
`sist.conf` in HOME directory(en route)


## Uninstall
`sudo make uninstall` after remove this directory  


## Customizing
Rewrite $HOME/sist.conf  
> NAME.color = COLOR  
> NAME.regex = REGEX  

```(e.g.)  
HOGE.color = RED
HOGE.regex = fuga
abc_1.color = 00FF00
abc_1.regex = ^(aaa|bbb|ccc)$
```


## Environment
* Windows10 ( WSL, linux on (VirtualBox|VMware) )  
* Linux


## License
MIT


## Note
1. Even if it is colored, IOS may not support that command.  
2. Hihlight colors are sloppy at the moment.  
