#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>

int main(int argc, char** argv)
{
  if(argc != 2 ) {
    printf("Usage:  `%s /dev/ttyS0` (for example)\n", argv[0]);
    return 1;
  }

  struct termios tio;
  struct termios stdio;
  struct termios old_stdio;
  int fd;
  int baudRate = B9600;
  const char *serialPort = argv[1];
  const char *buf;

  unsigned char c = '0';
  tcgetattr(STDOUT_FILENO, &old_stdio);

  memset(&stdio, 0, sizeof(stdio));
  stdio.c_iflag =     0;
  stdio.c_oflag =     0;
  stdio.c_cflag =     0;
  stdio.c_lflag =     0;
  stdio.c_cc[VMIN] =  1;
  stdio.c_cc[VTIME] = 0;
  tcsetattr(STDOUT_FILENO, TCSANOW,&stdio);
  tcsetattr(STDOUT_FILENO, TCSAFLUSH,&stdio);
  fcntl(STDIN_FILENO, F_SETFL, O_NONBLOCK);     // make the reads non-blocking

  memset(&tio, 0, sizeof(tio));
  tio.c_iflag =     0;
  tio.c_oflag =     0;
  tio.c_cflag =     CS8 | CREAD | CLOCAL;       // 8n1, see termios.h for more information
  tio.c_lflag =     0;
  tio.c_cc[VMIN] =  1;
  tio.c_cc[VTIME] = 5;

  fd = open(serialPort, O_RDWR | O_NONBLOCK);
  cfsetospeed(&tio, baudRate);
  cfsetispeed(&tio, baudRate);

  tcsetattr(fd, TCSANOW, &tio);
  while (1)
  {
    // if new data is available on the serial port, print it out
    if (read(fd, &c, 1) > 0) {
      if (c == '~') break;
      //buf += &c;
      write(STDOUT_FILENO, &c, 1);
    }

    // if new data is available on the console, send it to the serial port
    if (read(STDIN_FILENO, &c, 1) > 0) {
      if (c == '~') break;
      write(fd, &c, 1);
    }
  }

  close(fd);
  tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
  printf("\n\0");

  return EXIT_SUCCESS;
}
