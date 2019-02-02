# sisterm
sisterm is Serial console for CISCO.  
Not typo :)  
![demo](https://user-images.githubusercontent.com/29778890/52157763-0bd66400-26d5-11e9-829f-27da5c85a3aa.gif)


## Usage
//chmod 666 /dev/ttyS5  
`sisterm [-l SERIAL_PORT] [-s BAUDRATE] [-h]`  

(e.g. `sisterm -l /dev/ttyS0 -s 9600`)  


## Installation
Once you have it set up, a simple `make && make install` will compile sisterm and install it into `/usr/local/bin`.  
