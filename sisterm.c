#define PROGRAM      "sisterm"
#define VERSION      "1.0"
#define RELEASE_DATE "2019-02-04"

#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>

#include <stdio.h>
#include <time.h>
#include <regex.h>

#include "syntax.h"


#ifdef __linux__
#define CLOCK CLOCK_REALTIME_COARSE
#else
#define CLOCK CLOCK_REALTIME
#endif


unsigned char s[128];
unsigned char *io = s;

regex_t reg_prompt;
regex_t reg_vendors;
regex_t reg_ipv4_net;
regex_t reg_ipv4_sub;
regex_t reg_ipv4_wild;
regex_t reg_var;
regex_t reg_string;
regex_t reg_action;
regex_t reg_protocol;
regex_t reg_keyword;
regex_t reg_cond;
regex_t reg_interface;


int syntaxCheck(unsigned char *str)
{
  if( regexec(&reg_vendors  , str, 0, 0, 0) == 0 ) return HL_VENDORS;
  if( regexec(&reg_ipv4_net , str, 0, 0, 0) == 0 ) return HL_IPV4_NET;
  if( regexec(&reg_ipv4_sub , str, 0, 0, 0) == 0 ) return HL_IPV4_SUB;
  if( regexec(&reg_ipv4_wild, str, 0, 0, 0) == 0 ) return HL_IPV4_WILD;
  if( regexec(&reg_string   , str, 0, 0, 0) == 0 ) return HL_STRING;
  if( regexec(&reg_var      , str, 0, 0, 0) == 0 ) return HL_VAR;
  if( regexec(&reg_action   , str, 0, 0, 0) == 0 ) return HL_ACTION;
  if( regexec(&reg_protocol , str, 0, 0, 0) == 0 ) return HL_PROTOCOL;
  if( regexec(&reg_keyword  , str, 0, 0, 0) == 0 ) return HL_KEYWORD;
  if( regexec(&reg_cond     , str, 0, 0, 0) == 0 ) return HL_COND;
  if( regexec(&reg_interface, str, 0, 0, 0) == 0 ) return HL_INTERFACE;
  return -1;
}


void repaint(unsigned char *color)
{
  io = s;
  size_t i = 0;
  unsigned char str[128];
  unsigned char tmp[128];
  while(*io) tmp[i++] = *io++, write(STDOUT_FILENO, str, sprintf(str, "\b \b"));
  if(tmp[i]!='\0') tmp[i]='\0';
  write(STDOUT_FILENO, str, sprintf(str, "%s%s%s", color, tmp, RESET));
}


void coloring(unsigned char c)
{
  if( (c!=0x08 && c<0x21) )
    { memset( io = s, '\0', sizeof(s) ); return; }

  if( c=='\b' ) *--io = '\0';
  else          *io++ = c;

  int checked = syntaxCheck(s);
  if(checked > 0)
  {
    switch(checked)
    {
      case HL_VENDORS:
        repaint(AQUA);    break;
      case HL_ACTION:
        repaint(FUCHSIA); break;
      case HL_KEYWORD:
        repaint(MAROON);  break;
      case HL_COND:
        repaint(SILVER);  break;
      case HL_PROTOCOL:
        repaint(OLIVE);   break;
      case HL_VAR:
        repaint(TEAL);    break;
      case HL_STRING:
        repaint(YELLOW);  break;
      case HL_INTERFACE:
        repaint(BLUE);    break;
      case HL_IPV4_NET:
        repaint(RED);
        if(*(io-1)>0x29 || *(io-1)<0x3a) return;
        break;
      case HL_IPV4_SUB:
        repaint(PURPLE);
        if(*(io-1)>0x29 || *(io-1)<0x3a) return;
        break;
      case HL_IPV4_WILD:
        repaint(LIME);
        if(*(io-1)>0x29 || *(io-1)<0x3a) return;
        break;
      default: break;
    }
    memset( io = s, '\0', sizeof(s) );
  }

}

void version()
{
  printf("%s %s\n", PROGRAM, VERSION);
}

void usage(char *v)
{
  printf("Usage: %s [-l SERIAL_PORT] [-s BAUDRATE]\n"
         "            [-w /path/to/LOG] [-t] [-h] [-v]\n\n", v);

  printf("Command line interface for Serial Console by Network device.\n");
  printf("------------------------------------------------------------\n");
  printf("https://github.com/yorimoi/sisterm\n\n");

  printf("optional arguments:\n");
  printf("  -h          Show this help message and exit\n");
  printf("  -v          Show %s version and exit\n", PROGRAM);
  printf("  -l port     Use named device (e.g.    /dev/ttyS0)\n");
  printf("  -s speed    Use given speed  (default 9600)\n");
  printf("  -w path     Saved log        (e.g.    /tmp/sist.log)\n");
  printf("  -t          Add timestamp to log\n");
}


int main(int argc, char **argv)
{
  const char *sPort = NULL;
  const char *B     = NULL;
  const char *W     = NULL;
  speed_t baudRate  = B9600;
  int  logflag      = 0;
  int  ts           = 0;
  unsigned char     date[32];
  struct timespec   now;
  struct tm         tm;
  FILE              *log;

  for (int i=1; i<argc; i++)
  {
    if(*argv[i] == '-')
    {
      switch(*++argv[i])
      {
        //ToDo Add or Overwrite
        case 'l': sPort = argv[++i];    break;
        case 's': B = argv[++i];        break;
        case 'w': W = argv[++i];        break;
        case 't': ts = 1;               break;
        case 'h': usage(argv[0]);       return EXIT_SUCCESS;
        case 'v': version();            return EXIT_SUCCESS;
        default :
          printf("%s: unrecognized option `-%s`\n", argv[0], argv[i]);
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

  if( sPort == NULL )
  {
    printf("%s: must specify Serial Port\n", argv[0]);
    return EXIT_FAILURE;
  }

  if( B != NULL )
  {
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
      printf("(%s) Invalid BaudRate...\n", B);
      return EXIT_FAILURE;
    }
  }

  if( W != NULL )
  {
    log = fopen(W, "a+");
    if(access( W, (F_OK | R_OK) ) < 0)
    {
      printf("Logfile Access Denied\n");
      return EXIT_FAILURE;
    }
    if(log < 0) return EXIT_FAILURE;
    logflag = 1;
  }
  else
    ts = 0;

  struct termios tio;
  struct termios stdio;
  struct termios old_stdio;
  int fd;

  int regmiss      = 0 ;
  int bsflag       = 0 ;
  int prflag       = 0 ;
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

  fd = open(sPort, O_RDWR | O_NONBLOCK);
  if(fd < 0)
  {
    tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
    if(access( sPort, F_OK ) < 0)
      printf("%s: open (%s): No such file or directory\n", argv[0], sPort);
    else if(access( sPort, (R_OK | W_OK) ) < 0)
      printf("%s: open (%s): Permission denied\n", argv[0], sPort);
    // en route
    //else if(fcntl(fd, F_GETFL) == (F_RDLCK | F_WRLCK) )
    //  printf("%s: Line in use\n", sPort);
    else
      printf("%s: open (%s): Failure\n", argv[0], sPort);
    close(fd);
    return EXIT_FAILURE;
  }

  if( cfsetspeed(&tio, baudRate) < 0 ) return EXIT_FAILURE;

  if(regcomp(&reg_prompt   , "#|>"    , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_vendors  , VENDORS  , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_ipv4_net , IPV4_NET , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_ipv4_sub , IPV4_SUB , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_ipv4_wild, IPV4_WILD, REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_var      , VAR      , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_string   , STRING   , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_action   , ACTION   , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_protocol , PROTOCOL , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_keyword  , KEYWORD  , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_cond     , COND     , REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regcomp(&reg_interface, INTERFACE, REG_EXTENDED | REG_NOSUB | REG_ICASE) != 0) regmiss=1;
  if(regmiss) return EXIT_FAILURE;

  printf("Connected.\n");

  tcsetattr(fd, TCSANOW, &tio);

  while (1)
  {
    // if new data is available on the serial port, print it out
    // ToDo Parallel processing
    if(read(fd, &c, 1) > 0)
    {
      write(STDOUT_FILENO, &c, 1);
      if(logflag) fwrite(&c, 1, 1, log);

      if(0x08==c && 0==bsflag)
      {
        bsflag=3;
      }
      if(0 == bsflag) coloring(c);
      else if(3 == bsflag--) coloring(c);

      if(0x0a==c)
      {
        prflag=1;
        if(ts)
        {
          clock_gettime(CLOCK, &now);
          localtime_r(&now.tv_sec, &tm);
          sprintf(date, "[%d-%02d-%02d %02d:%02d:%02d.%03d] ",
              tm.tm_year+1900, tm.tm_mon+1, tm.tm_mday,
              tm.tm_hour, tm.tm_min, tm.tm_sec,
              now.tv_nsec / 1000000
              );
          fwrite(date, strlen(date), 1, log);
        }
      }
      if(prflag) {
        if( regexec(&reg_prompt, &c, 0, 0, 0) == 0)
        {
          memset( io = s, '\0', sizeof(s) );
          prflag=0;
        }
      }
    }

    // if new data is available on the console, send it to the serial port
    if(read(STDIN_FILENO, &c, 1) > 0)
    {
      if(c == endcode) break;
      write(fd, &c, 1);
    }
  }

  close(fd);
  if(logflag)
  {
    c = '\n';
    fwrite(&c, 1, 1, log);
    fclose(log);
  }
  tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);

  printf("%s\nDisconnected.\n", RESET);

  return EXIT_SUCCESS;
}
