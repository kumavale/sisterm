# sisterm
sisterm(`sist`) is Simplistic serial console for Router, Switch and Firewall.  
![demo](https://user-images.githubusercontent.com/29778890/52197792-2c084d80-28a4-11e9-8674-7e6cc652d955.gif)



## Usage
`chmod 666 /dev/ttyS5`  
```
sist [-l SERIAL_PORT] [-s BAUDRATE]
     [-e /path/to/LOG] [-t] [-h] [-v]
```
(e.g. `sist -l /dev/ttyS0 -s 9600 -e /tmp/sist.log`)  


## Installation
Once you have it set up, a simple `make && make install` will compile sisterm and install it into `/usr/local/bin`.  
