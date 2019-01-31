#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <regex.h>

#include "syntax.h"

char *increaseChar(unsigned char *str, char);
void syntaxCheck(unsigned char *str);
void usage(char *v);

int main(int argc, char **argv)
{
  const char *B = NULL;
  const char *serialPort = "/dev/ttyS5";
  int baudRate = B9600;

  for (int j=1; j<argc; j++)
  {
    if(*argv[j] == '-')
    {
      switch(*++argv[j])
      {
        case 'l': serialPort = argv[++j]; break;
        case 's': B = argv[++j]; break;
        case 'h': usage(argv[0]); return EXIT_SUCCESS;
        default :
          printf("%s: unrecognized option `-%s`\n", argv[0], argv[j]);
          printf("Usage: %s [-l SERIAL_PORT] [-s BAUDRATE] [-h]\n", argv[0]);
          printf("Use %s -h for help\n", argv[0]);
          return EXIT_FAILURE;
      }
    }
    else
    {
      printf("%s: %s: System not found\n", argv[0], argv[j]);
      return EXIT_FAILURE;
    }
  }

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
      return EXIT_FAILURE;
    }
  }

  struct termios tio;
  struct termios stdio;
  struct termios old_stdio;
  int fd;

  unsigned int i = 0;
  unsigned char buf[255];

  //char *palette[5] = {
  //  RED,
  //  GREEN,
  //  YELLOW,
  //  MAGENTA,
  //  CYAN,
  //};
  srand((unsigned)time(NULL));
  unsigned char s[32];
  unsigned int n = 34;

  unsigned char c = '0';
  const unsigned char endcode = '~';
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
  tio.c_iflag       = 0;
  tio.c_oflag       = 0;
  tio.c_cflag       = CS8 | CREAD | CLOCAL;       // 8n1, see termios.h for more information
  tio.c_lflag       = 0;
  tio.c_cc[VMIN]    = 1;
  tio.c_cc[VTIME]   = 5;

  fd = open(serialPort, O_RDWR | O_NONBLOCK);
  if(fd < 0)
  {
    tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
    if(access( serialPort, F_OK ) < 0)
      printf("%s: open (%s): No such file or directory\n", argv[0], serialPort);
    else if(access( serialPort, (R_OK | W_OK) ) < 0)
      printf("%s: open (%s): Permission denied\n", argv[0], serialPort);
    else
      printf("%s: open (%s): Failure\n", argv[0], serialPort);
    return EXIT_FAILURE;
  }

  cfsetospeed(&tio, baudRate);
  cfsetispeed(&tio, baudRate);

  tcsetattr(fd, TCSANOW, &tio);
  while (1)
  {
    // if new data is available on the serial port, print it out
    if(read(fd, &c, 1) > 0) {
      /*
      buf[i++] = c;
      //if(buf[i-1]=='\0' || buf[i-1]=='\n')
      {
        syntaxCheck(&buf[0]);
        i=0;
      } //*/
      //sprintf(s, "\e[48;5;%03dm\e[38;5;%03dm%c%s", rand()%255, rand()%255, c, RESET);
      //write(STDOUT_FILENO, &s, n);
      write(STDOUT_FILENO, &c, 1);
      //printf("Debug!");
    }

    // if new data is available on the console, send it to the serial port
    if(read(STDIN_FILENO, &c, 1) > 0) {
      if(c == endcode) break;
      /*
      buf[i++] = c;
      if(buf[i-1]=='\0' || buf[i-1]=='\n')
      {
        syntaxCheck(&buf[0]);
        i=0;
      } //*/
      //printf("%c", c);
      write(fd, &c, 1);
    }
  }

  close(fd);
  tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
  printf(RESET);
  printf("\n");

  return EXIT_SUCCESS;
}

void syntaxCheck(unsigned char *str)
{
  //if(!strcasecmp(str, "cisco "))
  //if( regcomp())
  //unsigned int i = 0;
  //unsigned char *_s;
  unsigned char _s[128];
  size_t len = 0;
  //unsigned char *buf = str;
  //while(*str != '\0')
  while(*str)
  {
    //write(STDOUT_FILENO, str, 1);
    _s[len++] = *str++;
    //len++;
  }
    //sprintf(_s, "%s%s%s%s", increaseChar(str, '\b'), CYAN, str, RESET);
    //sprintf(_s, "%s%s%s%s", increaseChar(str, '\b'), CYAN, str, RESET);
    //write(STDOUT_FILENO, &_s, len);
    write(STDIN_FILENO, &_s, len);
}

char *increaseChar(unsigned char *str, char c)
{
  unsigned char *b;
  for(int i=0; i<strlen(str); i++) *b++=c;
  return b;
}

void usage(char *v)
{
  printf("Usage: %s [-l SERIAL_PORT] [-s BAUDRATE] [-h]\n\n", v);
  printf("-----------------------------------------------------\n");
  printf("https://github.com/yorimoi/sisterm\n\n");
  printf("optional arguments:\n");
  printf("  -h    Show this help message and exit\n");
  printf("  -l    Use named device (e.g. /dev/ttyS0)\n");
  printf("  -s    Use given speed  (e.g. 9600)\n");
}
