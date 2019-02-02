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

int syntaxCheck(unsigned char *str);
//void printw(int );
void coloring(unsigned char);
void usage(char *v);

int main(int argc, char **argv)
{
  const char *B = NULL;
  const char *serialPort = "/dev/ttyS5";
  int baudRate = B9600;

  for (int i=1; i<argc; i++)
  {
    if(*argv[i] == '-')
    {
      switch(*++argv[i])
      {
        case 'l': serialPort = argv[++i]; break;
        case 's': B = argv[++i];          break;
        case 'h': usage(argv[0]);         return EXIT_SUCCESS;
        default :
          printf("%s: unrecognized option `-%s`\n", argv[0], argv[i]);
          printf("Usage: %s [-l SERIAL_PORT] [-s BAUDRATE] [-h]\n", argv[0]);
          printf("Use %s -h for help\n", argv[0]);
          return EXIT_FAILURE;
      }
    }
    else
    {
      printf("%s: %s: System not found\n", argv[0], argv[i]);
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

  // for test {
  unsigned char buf[256];
  srand((unsigned)time(NULL));
  // }

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
  fcntl(STDIN_FILENO, F_SETFL, O_NONBLOCK);

  memset(&tio, 0, sizeof(tio));
  tio.c_iflag       = 0;
  tio.c_oflag       = 0;
  tio.c_cflag       = CS8 | CREAD | CLOCAL;
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

  printf("\aConnected.\n");

  tcsetattr(fd, TCSANOW, &tio);

  while (1)
  {
    // if new data is available on the serial port, print it out
    if(read(fd, &c, 1) > 0)
    {
      //write(STDOUT_FILENO, &c, 1);
      write(STDOUT_FILENO, buf, sprintf(buf, "\e[38;5;%03dm%c%s", rand()%256, c, RESET));
      coloring(c);
    }

    // if new data is available on the console, send it to the serial port
    if(read(STDIN_FILENO, &c, 1) > 0)
    {
      if(c == endcode) break;
      write(fd, &c, 1);
    }
  }

  close(fd);
  tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
  printf("%s\n\aDisconnected.\n", RESET);

  return EXIT_SUCCESS;
}

int syntaxCheck(unsigned char *str)
{
  size_t i=0;
  unsigned char *buf = (unsigned char *)malloc(128);
  while(*str) buf[i++] = *str++;
  if(!strcasecmp(buf, "cisco")) { free(buf); return HL_CISCO; }
  if(!strcasecmp(buf, "test"))  { free(buf); return HL_COND ; }
  if(!strcmp(buf, "!"))  { free(buf); return HL_ACTION ; }
  return -1;
}

unsigned char s[128];
unsigned char *io = s;
void coloring(unsigned char c)
{
  //if( regcomp())
  if( c=='/' || c=='.' || c=='>' || c=='-' || c=='(' || c==')' || c=='#' || c=='\'' || c=='"' || c==' ' || c=='\n' || c=='\0' || c=='\t' )
  {
    memset( io = s, '\0', strlen(s) );
    return;
  }
  size_t i = 0;
  if( c=='\b' ) *--io = '\0';
  else *io++ = c;
  int checked = syntaxCheck(s);
  if(checked > 0)
  {
    unsigned char *buf, *tmp, *b;
    buf = (unsigned char*)malloc(128);
    tmp = (unsigned char*)malloc(128);
    b   = (unsigned char*)malloc(128);

    io = s;
    switch(checked)
    {
      case HL_CISCO:
        while(*io) b[i] = '\b', tmp[i++] = *io++;
        write(STDOUT_FILENO, buf, sprintf(buf, "%s%s%s%s", b, AQUA, tmp, RESET));
        break;
      case HL_COND:
        while(*io) b[i] = '\b', tmp[i++] = *io++;
        write(STDOUT_FILENO, buf, sprintf(buf, "%s%s%s%s", b, RED, tmp, RESET));
        break;
      case HL_ACTION:
        write(STDOUT_FILENO, buf, sprintf(buf, "\b%s!%s", LIME, RESET));
        break;
      default: break;
    }
    memset( io = s, '\0', strlen(s) );
    free(buf);
    free(tmp);
    free( b );
  }

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
