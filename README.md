# sisterm
sisterm(`sist`) is Simplistic serial console for Router, Switch and Firewall.  
![demo](https://user-images.githubusercontent.com/29778890/52164454-e711d900-2734-11e9-9700-adfe7ae03d72.gif)


## Usage
//chmod 666 /dev/ttyS5  
`sist [-l SERIAL_PORT] [-s BAUDRATE] [-h]`  

(e.g. `sist -l /dev/ttyS0 -s 9600`)  


## Installation
Once you have it set up, a simple `make && make install` will compile sisterm and install it into `/usr/local/bin`.  
