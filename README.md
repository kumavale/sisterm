# sisterm
sisterm(`sist`) is Simplistic serial console for Router, Switch and Firewall.  
![demo](https://user-images.githubusercontent.com/29778890/52197792-2c084d80-28a4-11e9-8674-7e6cc652d955.gif)


## Usage
`chmod 666 /path/to/port`  
```
Usage: sist [-l SERIAL_PORT] [-s BAUDRATE] [-r /path/to/file]
            [-w /path/to/LOG] [-t] [-a] [-h] [-v]
Options:
  -h,--help     Show this help message and exit
  -v,--version  Show sisterm version and exit
  -l port       Use named device    (e.g. /dev/ttyS0)
  -s speed      Use given speed     (default 9600)
  -r path       Output config file  (e.g. /tmp/config.txt)
  -w path       Saved log           (e.g. /tmp/sist.log)
  -t            Add timestamp to log
  -a            Append to log       (default overwrite)

Commands:
  ~           Terminate the conversation
```
(e.g. `sist -l /dev/ttyS0 -s 9600 -t -a -w /tmp/sist.log`)  


## Installation
Once you have it set up, a simple `make && make install` will compile sisterm and install it into `/usr/local/bin`.  


## Custom color
Rewrite palette.h  
Reinstall `make && make install`


## Environment[<sup>[1]</sup>](#note-1)
* Windows10 (WSL,VirtualBox,VMware)  
* Linux


## License
MIT


## Note
1. Verification only WSL(Arch) and Linux(Ubuntu18.04).  
2. Even if it is colored, iOS may not support that command.  
3. Hihlight colors are sloppy at the moment.  
