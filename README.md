# sisterm
sisterm(`sist`) is Simplistic serial console for Router, Switch and Firewall.  
![demo](https://user-images.githubusercontent.com/29778890/52197792-2c084d80-28a4-11e9-8674-7e6cc652d955.gif)


## Usage
`chmod 666 /path/to/port`  
```
Usage: sist [-l SERIAL_PORT] [-s BAUDRATE] [-r /path/to/file]
            [-w /path/to/LOG] [-t] [-a] [-h] [-v]
Options:
  -h,--help   Show this help message and exit
  -v          Show sisterm version and exit
  -l port     Use named device    (e.g. /dev/ttyS0)
  -s speed    Use given speed     (default 9600)
  -r path     Output config file  (e.g. /tmp/config.txt)
  -w path     Saved log           (e.g. /tmp/sist.log)
  -t          Add timestamp to log
  -a          Append to log       (default overwrite)

Commands:
  ~           Terminate the conversation
```
(e.g. `sist -l /dev/ttyS0 -s 9600 -t -a -w /tmp/sist.log`)  


## Custom color
palette.h  
reinstall


## Installation
Once you have it set up, a simple `make && make install` will compile sisterm and install it into `/usr/local/bin`.  
