#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>

#include <stdio.h>
//#include <unistd.h>

char backSpaceCount(unsigned char *str);
void syntaxCheck(unsigned char *str);
void usage(char *v);

int main(int argc, char **argv)
{
  //if(argc != 2 ) {
  //  printf("Usage:  `%s /dev/ttyS0` (for example)\n", argv[0]);
  //  return 1;
  //}

  const char *B;
  const char *serialPort;
  int baudRate = B9600;

  int j=1;
  while (j<argc)
  {
    if(*argv[j] == '-')
    {
      if(*argv[j]+1 == 'l')       serialPort = argv[j+1];
      else if(*argv[j]+1 == 's')  B = argv[j+1];
      else if(*argv[j]+1 == 'h')  usage(argv[0]);
      else return 1;
    }
    else return 1;
    j++;
  }

  struct termios tio;
  struct termios stdio;
  struct termios old_stdio;
  int fd;
  if(B != NULL){
    if     (!strcmp(B, "0"))      baudRate = B0;
    else if(!strcmp(B, "50"))     baudRate = B50;
    else if(!strcmp(B, "75"))     baudRate = B75;
    else if(!strcmp(B, "110"))    baudRate = B110;
    else if(!strcmp(B, "134"))    baudRate = B134;
    else if(!strcmp(B, "150"))    baudRate = B150;
    else if(!strcmp(B, "200"))    baudRate = B200;
    else if(!strcmp(B, "300"))    baudRate = B300;
    else if(!strcmp(B, "600"))    baudRate = B600;
    else if(!strcmp(B, "1200"))   baudRate = B1200;
    else if(!strcmp(B, "1800"))   baudRate = B1800;
    else if(!strcmp(B, "2400"))   baudRate = B2400;
    else if(!strcmp(B, "4800"))   baudRate = B4800;
    else if(!strcmp(B, "9600"))   baudRate = B9600;
    else if(!strcmp(B, "19200"))  baudRate = B19200;
    else if(!strcmp(B, "38400"))  baudRate = B38400;
    else if(!strcmp(B, "57600"))  baudRate = B57600;
    else if(!strcmp(B, "115200")) baudRate = B115200;
    else if(!strcmp(B, "230400")) baudRate = B230400;
    else
    {
      printf("Invalid BaudRate...\n");
      return 1;
    }
  }

  //const char *serialPort = argv[1];
  //int i;
  int i = 0;
  unsigned char buf[255];

  unsigned char c = '0';
  tcgetattr(STDOUT_FILENO, &old_stdio);

  memset(&stdio, 0, sizeof(stdio));
  stdio.c_iflag     = 0;
  stdio.c_oflag     = 0;
  stdio.c_cflag     = 0;
  stdio.c_lflag     = 0;
  stdio.c_cc[VMIN]  = 1;
  stdio.c_cc[VTIME] = 0;
  tcsetattr(STDOUT_FILENO, TCSANOW,&stdio);
  tcsetattr(STDOUT_FILENO, TCSAFLUSH,&stdio);
  fcntl(STDIN_FILENO, F_SETFL, O_NONBLOCK);     // make the reads non-blocking

  memset(&tio, 0, sizeof(tio));
  tio.c_iflag     = 0;
  tio.c_oflag     = 0;
  tio.c_cflag     = CS8 | CREAD | CLOCAL;       // 8n1, see termios.h for more information
  tio.c_lflag     = 0;
  tio.c_cc[VMIN]  = 1;
  tio.c_cc[VTIME] = 5;

  fd = open(serialPort, O_RDWR | O_NONBLOCK);
  cfsetospeed(&tio, baudRate);
  cfsetispeed(&tio, baudRate);

  tcsetattr(fd, TCSANOW, &tio);
  while (1)
  {
    // if new data is available on the serial port, print it out
    if(read(fd, &c, 1) > 0) {
      //if(c == '~') if((read(fd, &c, 1) > 0) && c == '.') break;
      //             else c = '~';
      buf[i++] = c;
      if(buf[i]==' ')
      {
        syntaxCheck(buf);
        i=0;
      }
      write(STDOUT_FILENO, &c, 1);
    }

    // if new data is available on the console, send it to the serial port
    if(read(STDIN_FILENO, &c, 1) > 0) {
      //if(c == '~') break;
      if(c == '~') if((read(fd, &c, 1) > 0) && c == '.') break;
                   else c = '~';
      buf[i++] = c;
      if(buf[i]==' ')
      {
        syntaxCheck(buf);
        i=0;
      }
      write(fd, &c, 1);
    }
  }

  close(fd);
  tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
  printf("\n\0");

  return EXIT_SUCCESS;
}

// murida...
void syntaxCheck(unsigned char *str)
{
  if(!strcasecmp(str, "cisco "))
    printf("%s\033[36m%s\033[0m", backSpaceCount(str), str);
}

char backSpaceCount(unsigned char *str)
{
  unsigned char b;
  for(int i=0; i<strlen(str); i++) b+='\b';
  return b;
}

void usage(char *v)
{
  printf("Usage:  %s [<option>]\n", v);
  printf(" -s [speed]    BaudRate\n");
  printf(" -l [Path]     Serial Port\n");
  printf(" -h            This help\n\n");
  printf("Example:  %s -s 9600 -l /dev/ttyS0\n", v);
}
