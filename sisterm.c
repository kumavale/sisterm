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

const char *vendors = "^cisco$|^jun$|^yama$|^mella$";
const char *ipv4 = "(^(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){3}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$";

regex_t reg_vendors;
regex_t reg_ipv4;
//regex_t reg_;

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

  cfsetspeed(&tio, baudRate);

  if( regcomp( &reg_vendors, vendors, REG_EXTENDED | REG_NOSUB | REG_ICASE ) != 0 ) return 1;
  if( regcomp( &reg_ipv4   , ipv4,    REG_EXTENDED | REG_NOSUB | REG_ICASE ) != 0 ) return 1;

  printf("\aConnected.\n");

  tcsetattr(fd, TCSANOW, &tio);

  // for test {
  char s[8];
  int bsflag = 0;
  unsigned char buf[256];
  srand((unsigned)time(NULL));
  // }

  while (1)
  {
    // if new data is available on the serial port, print it out
    if(read(fd, &c, 1) > 0)
    {
      write(STDOUT_FILENO, &c, 1);
      //write(STDOUT_FILENO, s, sprintf(s, "[0x%02x]", c));
      //write(STDOUT_FILENO, buf, sprintf(buf, "\e[38;5;%03dm%c%s", rand()%256, c, RESET));
      if(0x08==c && 0==bsflag) bsflag=3;
      if(0 == bsflag) coloring(c);
      else if(3 == bsflag--) coloring(c);
    }

    // if new data is available on the console, send it to the serial port
    if(read(STDIN_FILENO, &c, 1) > 0)
    {
      if(c == endcode) break;
      write(fd, &c, 1);
      //write(STDOUT_FILENO, s, sprintf(s, "[0x%02x]", c));
    }
  }

  close(fd);
  tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);

  regfree(&reg_vendors);
  regfree(&reg_ipv4);

  printf("%s\n\aDisconnected.\n", RESET);

  return EXIT_SUCCESS;
}

int syntaxCheck(unsigned char *str)
{
  size_t i=0;
  //unsigned char *buf = (unsigned char *)malloc(128);
  //while(*str) buf[i++] = *str++;
  //if(!strcasecmp(buf, "cisco")) { free(buf); return HL_CISCO; }
  //if(!strcasecmp(buf, "test"))  { free(buf); return HL_COND ; }
  if( regexec(&reg_vendors, str, 0, 0, 0) == 0 ) return HL_CISCO;
  if( regexec(&reg_ipv4,    str, 0, 0, 0) == 0 ) return HL_IPV4;
  //if( regexec(&preg, str, 0, 0, 0) == 0 ) { while(*str++) i++; return i; }
  return -1;
}

unsigned char s[128];
unsigned char prev[128];
unsigned char *io = s;
void coloring(unsigned char c)
{
  //if( regcomp())
  if( (c!=0x08 && c<0x21) || c=='#' || c=='>' || c=='"')
    { memset( io = s, '\0', strlen(s) ); return; }

  if( c=='\b' ) *--io = '\0';
  else          *io++ = c;

  int checked = syntaxCheck(s);
  if(checked > 0)
  {
    unsigned char *buf, *tmp, *b;
    buf = (unsigned char*)malloc(128);
    tmp = (unsigned char*)malloc(128);
    b   = (unsigned char*)malloc(128);

    size_t i = 0;
    io = s;
    switch(checked)
    {
      case HL_CISCO:
        //while(*io) write(STDOUT_FILENO, buf, sprintf(buf, "%s[0x%02x]%s",AQUA,*io++,RESET));
        while(*io) b[i] = '\b', tmp[i++] = *io++;
        if(tmp[i]!='\0') tmp[i]='\0';
        write(STDOUT_FILENO, buf, sprintf(buf, "%s%s%s%s", b, AQUA, tmp, RESET));
        //write(STDOUT_FILENO, buf, sprintf(buf, "%s[%d]%s", LIME, strlen(b), RESET));
        break;
      case HL_IPV4:
        while(*io) b[i] = '\b', tmp[i++] = *io++;
        //io = s; i=0;
        //write(STDOUT_FILENO, buf, sprintf(buf, "%s",b));
        //while(*io++) write(STDOUT_FILENO, buf, sprintf(buf, "%s%c%s",RED,tmp[i++],RESET));
        write(STDOUT_FILENO, buf, sprintf(buf, "%s%s%s%s", b, RED, tmp, RESET));
        //write(STDOUT_FILENO, buf, sprintf(buf, "%s[%d]%s", LIME, strlen(tmp), RESET));
        break;
      default:
        //write(STDOUT_FILENO, buf, sprintf(buf, "%s[%d]%s", LIME, checked, RESET));
        break;
    }
    //strcpy(prev, s);
    //if(prev[strlen(prev)-1]
    if(tmp[i-1]<0x29 || tmp[i-1]>0x3a)
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
