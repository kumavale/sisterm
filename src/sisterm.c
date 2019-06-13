
#define COMMAND_NAME   "sist"
#define PROGRAM_NAME   "sisterm"
#define VERSION        "1.4.4-rc"
#define UPDATE_DATE    "20190613"

#define CONFIG_FILE    "sist.conf"
#define MAX_PARAM_LEN  2048

#include "sisterm.h"
#include "palette.h"


#ifdef __linux__
// CLOCK_REALTIME_COARSE
#define              CLOCK 5
#else
#define              CLOCK CLOCK_REALTIME
#endif

#define MAX_LENGTH   256
#define REG_FLAGS    REG_EXTENDED | REG_NOSUB | REG_ICASE

char s[MAX_LENGTH];
char *io = s;
bool bsflag = false;

// param
typedef struct {
    char *name;
    char *color;
    regex_t regex;
    bool cmped;
} Param;

Param *params;  // Dynamic array
int params_len;

//For debug
void DebugLog(const char *_format, ... ) {
    int len;
    va_list argList;
    va_start(argList, _format);
    char str[MAX_LENGTH];
    len = vsprintf(str, _format, argList);
    va_end(argList);
    transmission(STDOUT_FILENO, str, len);
}

void error(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    //fprintf(stderr, "\n");
}

int main(int argc, char **argv) {
    char *sPort       = NULL;             // SerialPort
    char *B           = NULL;             // BaudRate
    char *R           = NULL;             // File path to load
    char *W           = NULL;             // File path to save
    char *C           = NULL;             // File path to config
    speed_t baudRate  = B9600;            // Default BaudRate
    bool existsflag   = false;            // Whether to log file
    //bool excflag      = false;            // Exclamation mark flag for comment
    //int  comlen       = 0;                // Comment length
    bool escflag      = false;            // '^'
    bool spflag       = false;            // '['
    bool tilflag      = false;            // Del key -> BS key
    bool arrflag      = false;            // Arrow keys flag
    bool logflag      = false;            // Whether to take a log
    bool tcpflag      = false;            // TCP
    bool wflag        = false;            // Write file Flag
    bool rflag        = false;            // Read file Flag
    bool another_conf = false;            // another config file
    bool cflag        = true;             // Color Flag
    bool ts           = false;            // Whether to timestamp
    char* logbuf      = (char*)malloc(MAX_LENGTH);
    char* lb          = logbuf;           // Log buffer pointer for operation
    int  lblen        = MAX_LENGTH - 2;
    //char              comm[32];           // For comment
    char              date[81];           // Buffer to set timestamp
    struct timespec   now;
    struct tm         tm;
    FILE *lf          = NULL;             // Log file
    char mode[3]      = "w+";             // Log file open mode
    char dstaddr[21+1];
    const char CR     = 0x0d;           // Carriage Return


    {
	    static struct option longopts[] = {
	    	{"port",      required_argument, 0, 'l'},
	    	{"speed",     required_argument, 0, 's'},
	    	{"read",      required_argument, 0, 'r'},
	    	{"write",     required_argument, 0, 'w'},
	    	{"config",    required_argument, 0, 'c'},
	    	{"help",      no_argument,       0, 'h'},
	    	{"version",   no_argument,       0, 'v'},
	    	{"timestamp", no_argument,       0, 't'},
	    	{"append",    no_argument,       0, 'a'},
	    	{"no-color",  no_argument,       0, 'n'},
	    	{0,0,0,0}
	    };
        int opt, idx;
        while((opt = getopt_long(
                argc,
                argv,
                "l:s:r:w:c:tanp:hv",
                longopts,
                &idx)
        ) != -1) {
            switch(opt) {
                case 'l':
                  // /path/to/SerialPort
                    sPort = (char*)malloc(strlen(optarg)+1);
                    strcpy(sPort, optarg);
                    break;

                case 's':
                  // BaudRate speed
                    B = (char*)malloc(strlen(optarg)+1);
                    strcpy(B, optarg);
                    break;

                case 'r':
                  // /path/to/config.txt
                    R = (char*)malloc(strlen(optarg)+1);
                    strcpy(R, optarg);
                    rflag = true;
                    break;

                case 'w':
                  // /path/to/log.txt
                    W = (char*)malloc(strlen(optarg)+1);
                    strcpy(W, optarg);
                    wflag = true;
                    break;

                case 'c':
                  // /path/to/config
                    C = (char*)malloc(strlen(optarg)+1);
                    strcpy(C, optarg);
                    another_conf = true;
                    break;

                case 't':
                  // Add timestamp to log
                    ts = true;
                    break;

                case 'a':
                  // Append log
                    strcpy(mode, "a+");
                    break;

                case 'n':
                  // Without color
                    cflag = false;
                    break;

                case 'p':
                  // Tcp socket
                  // XXX.XXX.XXX.XXX:XXXXX
                  // ToDo => hostname
                    if(!correct_ipaddr_format(optarg)) {
                        error("%serror:%s Bad address or port number: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, optarg, RESET);
                        return EXIT_FAILURE;
                    }
                    tcpflag = true;
                    pack_space_cpy(dstaddr, optarg);
                    break;

                case 'h':
                  // Show help
                    usage(argv);
                    return EXIT_SUCCESS;

                case 'v':
                  // Show version
                    version();
                    return EXIT_SUCCESS;

                default :
                    error("Use %s -h for help\n", argv[0]);
                    return EXIT_FAILURE;
              }
         }
    }



    if( sPort == NULL && !rflag && !tcpflag ) {
        error("%serror:%s must specify Serial Port\n", ERROR_RED, RESET, argv[0]);
        return EXIT_FAILURE;
    }


    if( B != NULL && !rflag && !tcpflag ) {
        if     (!strcmp(B, "0"))      baudRate = B0;      // hang up
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
        else if(!strcmp(B, "9600"))   baudRate = B9600;   // Default
        else if(!strcmp(B, "19200"))  baudRate = B19200;
        else if(!strcmp(B, "38400"))  baudRate = B38400;
        else if(!strcmp(B, "57600"))  baudRate = B57600;
        else if(!strcmp(B, "115200")) baudRate = B115200;
        else if(!strcmp(B, "230400")) baudRate = B230400;
        else {
          error("%serror:%s Invalid BaudRate: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, B, RESET);
          return EXIT_FAILURE;
        }
    }


    if( wflag && !rflag ) {
        if(!access(W, F_OK))
            existsflag = true;

        if( existsflag && (access( W, (F_OK | R_OK) ) < 0) ) {
            error("%serror:%s Access to the log file is denied\n", ERROR_RED, RESET);
            return EXIT_FAILURE;
        }

        if( existsflag && !strcmp(mode, "w+") ) {
            error("\a%s\"%s\"%s is already exists!\n", ERROR_YELLOW, W, RESET);
            error("Do you want to overwrite?[confirm]");
            char con = getchar();
            if( !(con=='\n' || con=='y' || con=='Y') )
                return EXIT_SUCCESS;
        }

        lf = fopen(W, mode);

        if(lf == NULL) {
            if(access(W, F_OK)) {
              error("%serror:%s Failed to create file: Try to check the permissions\n", ERROR_RED, RESET);
              return EXIT_FAILURE;
            }
            else if( access( W, (F_OK | R_OK) ) < 0 ) {
              error("%serror:%s Access to the log file is denied\n", ERROR_RED, RESET);
              return EXIT_FAILURE;
            }

            error("%s: open (%s): Failure\n", argv[0], W);
            error("%serror:%s file open Failure: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, W, RESET);
            return EXIT_FAILURE;
        }

        logflag = 1;
    }


    {
        FILE *cfp;  // Config File Pointer
        char *path;

        if(another_conf) {
            path = (char*)malloc(strlen(C)+1);
            strcpy(path, C);
            path[strlen(C)] = '\0';
        } else {
            int len = strlen(getenv("HOME")) + 1 + strlen(CONFIG_FILE);
            path = (char*)malloc(len+1);
            memset(path, '\0', len+1);
            sprintf(path, "%s/%s", getenv("HOME"), CONFIG_FILE);
        }

        cfp = fopen(path, "r");

        if(cfp == NULL) {
            cflag = false;
            error("%serror:%s File open error: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, path, RESET);
            error("Press ENTER to continue of without color mode");
            (void)getchar();
        }
        else {
            params = (Param*)malloc(sizeof(Param));
            char *str = (char*)malloc(MAX_PARAM_LEN);
            int line = 0;

            while(fgets(str, MAX_PARAM_LEN, cfp) != NULL) {
                ++line;
                char top = '\0';
                sscanf(str, " %c", &top);
                // ignore comment and blank line
                if(strchr(" #\n\0", top))
                    continue;

                char *key   = (char*)malloc(64),
                     *op    = (char*)malloc(2+1),
                     *name  = (char*)malloc(64),
                     *param = (char*)malloc(MAX_PARAM_LEN);

                sscanf(str, " %63[^ .] . %63[^ +=] %2[+=] %2047[^\n]", name, key, op, param);
                //printf("[name:%s, key:%s, op:%s, param:%s]\n", name, key, op, param);

                bool suffer = false;
                for(int i=0; i<params_len; ++i)
                    if(!strcmp(params[i].name, name)) {
                        suffer = true;
                        break;
                    }
                if(!suffer) {
                    if(!strcmp(op, "+=")) {
                        int cnt = chrcnt(line);
                        error("%serror:%s '%s%s.%s%s' is used uninitialized\n", ERROR_RED, RESET, ERROR_YELLOW, name, key,RESET);
                        error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt), RESET, path, line);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    ++params_len;
                    Param *params_tmp = (Param*)realloc(params, params_len * sizeof(Param));
                    if(params_tmp == NULL) {
                        free(params);
                        error("%serror:%s realloc() failed\n", ERROR_RED, RESET);
                        return EXIT_FAILURE;
                    }
                    params = params_tmp;
                    params[params_len-1].name = (char*)malloc(strlen(name)+1);
                    strcpy(params[params_len-1].name, name);
                    params[params_len-1].name[strlen(name)] = '\0';
                }

                // DOS file format
                if(param[strlen(param)-1] == 0x0D)
                    param[strlen(param)-1] = '\0';

                if(!strcmp(key, "color")) {
                    bool color_flug = false;
                    for(int i=0; i<AC_MAX; ++i)
                        if(!strcasecmp(param, ansi_colors[i].key)) {
                            color_flug = true;
                            params[params_len-1].color = (char*)malloc(strlen(ansi_colors[i].val)+1);
                            strcpy(params[params_len-1].color, ansi_colors[i].val);
                            params[params_len-1].color[strlen(ansi_colors[i].val)] = '\0';
                            break;
                        }
                    if(!color_flug) {
                        if(strlen(param) > 6) {
                            char *p;
                            char *param_buf = (char*)malloc(strlen(param)+1);
                            strcpy(param_buf, param);
                            p = param_buf;
                            int i = 0;
                            while(*p) {
                                if(isspace(*p)) {
                                    ++p;
                                    continue;
                                }
                                param_buf[i] = *p;
                                ++i;
                                ++p;
                            }
                            param_buf[i] = '\0';
                            if(param_buf[0] == '"' || param_buf[i-1] == '"') {
                                int cnt = chrcnt(line);
                                error("%serror:%s Invalid color: '%s%s%s': expected not to require '\"'\n", ERROR_RED, RESET, ERROR_YELLOW, param, RESET);
                                error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt), RESET, path, line);
                                error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                                error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                return EXIT_FAILURE;
                            }
                            if(param_buf[i-1] != 'm') {
                                int cnt = chrcnt(line);
                                error("%serror:%s Invalid color: '%s%s%s': expected 'm' in end\n", ERROR_RED, RESET, ERROR_YELLOW, param, RESET);
                                error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt), RESET, path, line);
                                error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                                error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                return EXIT_FAILURE;
                            }

                            replace(param_buf, "\\033", "\033");
                            replace(param_buf, "\\e",   "\x1B");
                            replace(param_buf, "\\x1b", "\x1b");
                            replace(param_buf, "\\x1B", "\x1B");

                            strcpy(param, param_buf);
                            free(param_buf);

                            if(!strcmp(op, "+=")) {
                                char *param_tmp = (char*)realloc(params[params_len-1].color, strlen(params[params_len-1].color)+strlen(param)+1);
                                if(param_tmp == NULL) {
                                    error("%serror:%s realloc() failed\n", ERROR_RED, RESET);
                                    return EXIT_FAILURE;
                                }
                                params[params_len-1].color = param_tmp;
                                strcat(params[params_len-1].color, param);
                            } else {
                                params[params_len-1].color = (char*)malloc(strlen(param)+1);
                                strcpy(params[params_len-1].color, param);
                                params[params_len-1].color[strlen(param)] = '\0';
                            }
                        }
                        else if(strlen(param) == 3) {
                            for(int i=0; i<3; ++i)
                                if(!isdigit(param[i])) {
                                    int cnt = chrcnt(line);
                                    error("%serror:%s Invalid color: '%s%s%s'\n", ERROR_RED, RESET, ERROR_YELLOW, param, RESET);
                                    error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt), RESET, path, line);
                                    error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                    error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                                    error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                    return EXIT_FAILURE;
                                }
                            u_int16_t num = strtol(param, NULL, 10);
                            if(num > 255) {
                                int cnt = chrcnt(line);
                                error("%serror:%s Invalid color: '%s%s%s': less than 256\n", ERROR_RED, RESET, ERROR_YELLOW, param, RESET);
                                error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt), RESET, path, line);
                                error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                                error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                return EXIT_FAILURE;
                            }
                            char format[11+1];  // \033[38;5;XXXm
                            snprintf(format, sizeof(format), "\033[38;5;%3sm", param);
                            params[params_len-1].color = (char*)malloc(strlen(format)+1);
                            strcpy(params[params_len-1].color, format);
                            params[params_len-1].color[strlen(format)] = '\0';
                        }
                        else if(strlen(param) == 6) {
                            for(int i=0; i<6; ++i)
                                if(!ishex(param[i])) {
                                    int cnt = chrcnt(line);
                                    error("%serror:%s Invalid color: '%s%s%s'\n", ERROR_RED, RESET, ERROR_YELLOW, param, RESET);
                                    error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt), RESET, path, line);
                                    error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                    error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                                    error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                                    return EXIT_FAILURE;
                                }
                            char hexs[3][2+1] = {
                                { param[0], param[1], '\0' },
                                { param[2], param[3], '\0' },
                                { param[4], param[5], '\0' }};
                            char format[19+1];  // \033[38;2;XXX;XXX;XXXm
                            snprintf(format, sizeof(format), "\033[38;2;%03ld;%03ld;%03ldm",
                            strtol(hexs[0], NULL, 16),
                            strtol(hexs[1], NULL, 16),
                            strtol(hexs[2], NULL, 16));
                            params[params_len-1].color = (char*)malloc(strlen(format)+1);
                            strcpy(params[params_len-1].color, format);
                            params[params_len-1].color[strlen(format)] = '\0';
                        } else {
                            int cnt = chrcnt(line);
                            error("%serror:%s Invalid color: '%s%s%s'\n", ERROR_RED, RESET, ERROR_YELLOW, param, RESET);
                            error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt), RESET, path, line);
                            error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                            error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                            error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                            return EXIT_FAILURE;
                        }
                    }
                }
                else if(!strcmp(key, "regex")) {
                    if(param[0] == '"' || param[strlen(param)-1] == '"') {
                        int cnt = chrcnt(line);
                        error("%serror:%s Invalid regex: '%s%s%s': expected not to require '\"'\n", ERROR_RED, RESET, ERROR_YELLOW, param, RESET);
                        error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt),  RESET, path, line);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    int rc;
                    if((rc = regcomp(&params[params_len-1].regex, param, REG_FLAGS))) {
                        int cnt = chrcnt(line);
                        char msg[100];
                        regerror(rc, &params[params_len-1].regex, msg, 100);
                        error("%serror:%s regcomp() failred: %s\n", ERROR_RED, RESET, msg);
                        error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt),  RESET, path, line);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    if(!strcmp(op, "+=")) {
                        int cnt = chrcnt(line);
                        error("%serror:%s The \"+=\" operator can\'t be used with regex\n", ERROR_RED, RESET);
                        error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt),  RESET, path, line);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                        error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    params[params_len-1].cmped = true;
                }
                else {
                    int cnt = chrcnt(line);
                    error("%serror:%s Neither color nor regex: '%s%s%s'\n", ERROR_RED, RESET, ERROR_YELLOW, key, RESET);
                    error("  %s%s>%s %s:%d\n", ERROR_BLUE, loopc('-', cnt),  RESET, path, line);
                    error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                    error("%s%d |%s %s", ERROR_BLUE, line, RESET, str);
                    error(" %s%s|%s\n", loopc(' ', cnt), ERROR_BLUE, RESET);
                    return EXIT_FAILURE;
                }
                free(name);
                free(key);
                free(op);
                free(param);
            }
            fclose(cfp);
            free(str);

            // Both color and regex
            for(int i=0, failed=0; i<params_len; ++i) {
                if(params[i].color == NULL) failed = 1;
                if(params[i].cmped == 0)    failed = 2;
                if(failed) {
                    error("%serror:%s %s.%s is not defined\n", ERROR_RED, RESET, params[i].name, failed==1?"color":"regex");
                    return EXIT_FAILURE;
                }
            }
        }
        free(path);
    }

    struct termios tio;
    struct termios stdio;
    struct termios old_stdio;
    int fd;

    char c             = '0';
    const char endcode = '~';
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
    if( fd < 0 && !rflag && !tcpflag ) {
        tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
        if(access( sPort, F_OK ) < 0)
            error("%serror:%s No such file or directory: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, sPort, RESET);
        else if(access( sPort, (R_OK | W_OK) ) < 0)
            error("%serror:%s Permission denied: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, sPort, RESET);
        // unstable
        //else if(fcntl(fd, F_GETFL) == -1)
        //  printf("%s: %s: Line in use\n", argv[0], sPort);
        else
            error("%serror:%s File open failure: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, sPort, RESET);
        close(fd);
        return EXIT_FAILURE;
    }

    if( cfsetispeed(&tio, baudRate) != 0 ) return EXIT_FAILURE;
    if( cfsetospeed(&tio, baudRate) != 0 ) return EXIT_FAILURE;

    if( rflag && !tcpflag ) {
        FILE *fr;
        fr = fopen(R, "r");
        if(fr == NULL) {
            tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
            if(access( R, F_OK ) < 0)
                error("%serror:%s No such file or directory: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, R, RESET);
            else if(access( R, (R_OK) ) < 0)
                error("%serror:%s Permission denied: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, R, RESET);
            else
                error("%serror:%s File open failure: %s\"%s\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, R, RESET);
            return EXIT_FAILURE;
        }

        //if( setvbuf(stdout, NULL, _IOLBF, 2048) != 0 )
        //{
        //  /* If failure without buffering */
        //}

        tcsetattr(fd, TCSANOW, &tio);

        int i;
        while((i=fgetc(fr)) != EOF) {
            c = (char)i;
            if( 0x07==c || 0x08==c || 0x0a==c || 0x0d==c || (0x1f<c && 0x7f>c) )
                transmission(STDOUT_FILENO, &c, 1);

            if( 0x0a==c ) {
                transmission(STDOUT_FILENO, &CR, 1);
                transmission(STDOUT_FILENO, RESET, strlen(RESET));
            }

            if( cflag ) {
                coloring(c);
            }

            if(read(STDIN_FILENO, &c, 1) > 0) {
                if(c == endcode) break;
            }
        }

        tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
        fclose(fr);
        printf("\n%s", RESET);

        return EXIT_SUCCESS;
    }

    /* ----------------------------------------------------------------------- */
    // 分割したい
    if( tcpflag ) {
        tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);

        struct sockaddr_in sa;
        const uint16_t default_port = 23;  // TELNET
        int port;
        if((port = pull_port_num(dstaddr)) < 0)
            port = default_port;
        
        if( (fd = socket(AF_INET, SOCK_STREAM, 0)) < 0 ) {
            error("%serror:%s Failed socket()\n", ERROR_RED, RESET);
            return EXIT_FAILURE;
        }

        char *address = (char*)malloc(15+1);
        store_address(address, dstaddr);

        memset(&sa, 0, sizeof(sa));
        sa.sin_family = AF_INET;
        sa.sin_port = htons(port);
        sa.sin_addr.s_addr = inet_addr(address);

        free(address);

        if( sa.sin_addr.s_addr == 0xffffffff ) {
            error("%serror:%s Bad address\n", ERROR_RED, RESET);
            return EXIT_FAILURE;
        }

//#include <sys/ioctl.hj
//        ioctl(fd, FIONBIO, 1);

        if( connect(fd, (struct sockaddr *)&sa, sizeof(sa)) > 0) {
            error("%serror:%s Not established\n", ERROR_RED, RESET);
            close(fd);
            return EXIT_FAILURE;
        }

        printf("Connected.\n");

        tcsetattr(fd, TCSANOW, &tio);

        //struct { char *addr; } share_logbuf;
        //share_logbuf.addr = logbuf;

        pid_t pid;
        pid_t p_pid = getpid();
        pid = fork();

        if( 0 > pid ) {
            error("%serror:%s Failed fork()\n", ERROR_RED, RESET);
            return EXIT_FAILURE;
        }

        if( 0 == pid ) {
            for(;;) {
                //if( recv(fd, &c, 1, MSG_DONTWAIT) > 0 )
                if( recv(fd, &c, 1, 0) > 0 ) {
                    if( 0x07==c || 0x08==c || 0x0a==c || 0x0d==c || (0x1f<c && 0x7f>c) )
                        transmission(STDOUT_FILENO, &c, 1);

                    if( logflag ) {
                        if( (int)strlen(logbuf) > lblen ) {
                            char *lb_tmp = (char*)realloc(logbuf, sizeof(char) * (lblen += MAX_LENGTH));
                            if(lb_tmp == NULL) {
                                free(logbuf);
                                error("%serror:%s Failed realloc()\n");
                                abort_exit(STDOUT_FILENO, TCSANOW, &old_stdio);
                            }
                            //share_logbuf.addr = lb = logbuf = lb_tmp;
                            lb = logbuf = lb_tmp;
                        }

                        if( 0x08==c ) {
                            //if( lb != logbuf ) // Sent 0x07 from device
                            lb--;
                        }
                        else if( 0x1f<c && 0x7f>c ) {
                            *lb = c;
                            ++lb;
                        }
                        else if( '\n'==c ) {
                            logbuf[strlen(logbuf)] = '\n';

                            if( ts ) {
                                clock_gettime(CLOCK, &now);
                                localtime_r(&now.tv_sec, &tm);
                                sprintf(date, "[%d-%02d-%02d %02d:%02d:%02d.%03d] ",
                                    tm.tm_year+1900, tm.tm_mon+1, tm.tm_mday,
                                    tm.tm_hour, tm.tm_min, tm.tm_sec,
                                    (int) now.tv_nsec / 1000000);
                                fwrite(date, 1, strlen(date), lf);
                            }

                            fwrite(logbuf, 1, strlen(logbuf), lf);
                            fflush(lf);

                            if( lblen > MAX_LENGTH - 2 ) {
                                lblen = MAX_LENGTH - 2;
                                char *lb_tmp = (char*)realloc(logbuf, sizeof(char) * (MAX_LENGTH));
                                if(lb_tmp == NULL) {
                                    free(logbuf);
                                    error("%serror:%s Failed realloc()\n", ERROR_RED, RESET);
                                    abort_exit(STDOUT_FILENO, TCSANOW, &old_stdio);
                                }
                                //share_logbuf.addr = lb = logbuf = lb_tmp;
                                lb = logbuf = lb_tmp;
                            }

                            //memset( share_logbuf.addr = lb = logbuf, '\0', MAX_LENGTH );
                            memset( lb = logbuf, '\0', MAX_LENGTH );
                        }
                    }

                    if( 0x0a==c ) {
                        transmission(STDOUT_FILENO, RESET, strlen(RESET));
                    }

                    //if( 0x21==c && cflag && !excflag ) {
                    //    comlen = 0;
                    //    excflag = true;
                    //    transmission(stdout_fileno, comm, sprintf(comm, "\b%s%c", color_comment, c));
                    //}

                    //if( excflag && 0x07!=c )
                    //    comlen++;

                    //if( 0x08==c ) {
                    //    if( excflag )
                    //        comlen-=2;
                    //    if( excflag && 0>=comlen ) {
                    //        transmission(stdout_fileno, comm, sprintf(comm, "%s", reset));
                    //        io++;
                    //        excflag = false;
                    //    }
                    //}

                    //if( !excflag && cflag )
                    if( cflag ) {
                        coloring(c);
                    }

                }
                else if( recv(fd, &c, 1, 0) == 0) {
                    kill(p_pid, SIGINT);
                    break;  // hang up
                }

                if( kbhit() ) {
                    transmission(STDOUT_FILENO, "\b", 1);
                    //DebugLog("\b");
                }
            }
        }
        

        if( 0 != pid ) {
            for(;;) {
                if( kbhit() ) {
                    //if(read(STDIN_FILENO, &c, 1) > 0) {
                    {
                        c = getchar();
                        if( 0x1b==c )                          escflag = true;  // ^
                        else if( escflag && 0x5b==c )          spflag  = true;  // ^[
                        else if( spflag  && 0x33==c )          tilflag = true;  // ^[3
                        else if( spflag  && 0x40<c && 0x45>c ) arrflag = true;  // ^[[ABCD]
                        else if( tilflag && 0x7e==c ) {                         // ^[3~
                            c = 0x7f;
                            escflag = spflag = tilflag = false;
                        }
                        else {
                            escflag = spflag = false;
                        }

                        if( endcode == c ) {
                            //kill(p_pid, SIGINT);
                            // 子プロセスからlogbufのアドレスを持ってきたい
                            //if(logflag) {
                            //    if( ts ) {
                            //        clock_gettime(CLOCK, &now);
                            //        localtime_r(&now.tv_sec, &tm);
                            //        sprintf(date, "[%d-%02d-%02d %02d:%02d:%02d.%03d] ",
                            //            tm.tm_year+1900, tm.tm_mon+1, tm.tm_mday,
                            //            tm.tm_hour, tm.tm_min, tm.tm_sec,
                            //            (int) now.tv_nsec / 1000000);
                            //        fwrite(date, 1, strlen(date), lf);
                            //    }
                            //    char *loglast = (char*)malloc(strlen(share_logbuf.addr)+2);
                            //    sprintf(loglast, "%s\n", share_logbuf.addr);
                            //    fwrite(loglast, 1, strlen(loglast), lf);
                            //    fflush(lf);
                            //    fclose(lf);
                            //}
                            kill(pid, SIGINT);

                            break;  // hang up
                        }

                        if( 0x00 == c )
                            c = 0x7f;  // BS on Vimterminal

                        //DebugLog("[0x%02x]", c);
                        if( !escflag )
                            send(fd, &c, 1, 0);
                        if( arrflag ) {
                            char* arrow = (char*)malloc(4);
                            sprintf(arrow, "%c%c%c", 0x1b, 0x5b, c);
                            send(fd, arrow, 3, 0);
                            free(arrow);
                            arrflag = false;
                        }
                    }
                }

                // 100 microsecond
                usleep(100);
            }
        }
        

        tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
        printf("%s\nDisconnected.\n", RESET);
        close(fd);
        return EXIT_SUCCESS;
    }
    /* ----------------------------------------------------------------------- */

    printf("Connected.\n");

    tcsetattr(fd, TCSANOW, &tio);

    for(;;) {
        // if new data is available on the serial port, print it out
        // ToDo Parallel processing
        if(read(fd, &c, 1) > 0) {
            if( 0x07==c || 0x08==c || 0x0a==c || 0x0d==c || (0x1f<c && 0x7f>c) )
                transmission(STDOUT_FILENO, &c, 1);
          //DebugLog("[%02x]", c);

            if( logflag ) {
                // Unstable
                if( (int)strlen(logbuf) > lblen ) {
                    char *lb_tmp = (char*)realloc(logbuf, sizeof(char) * (lblen += MAX_LENGTH));
                    if(lb_tmp == NULL) {
                        free(logbuf);
                        error("%serror:%s Failed realloc()\n", ERROR_RED, RESET);
                        abort_exit(STDOUT_FILENO, TCSANOW, &old_stdio);
                    }
                    lb = logbuf = lb_tmp;
                }

                if( 0x08==c ) {
                    //if( lb != logbuf ) // Sent 0x07 from device
                    lb--;
                }
                else if( 0x1f<c && 0x7f>c ) {
                  *lb = c;
                  ++lb;
                }
                //else if( /*0x0d==c ||*/ 0x0a==c ) {
                else if( '\n'==c ) {
                    logbuf[strlen(logbuf)] = '\n';
                    //if( 0x0a==c )
                    {
                        if( ts ) {
                            clock_gettime(CLOCK, &now);
                            localtime_r(&now.tv_sec, &tm);
                            sprintf(date, "[%d-%02d-%02d %02d:%02d:%02d.%03d] ",
                                tm.tm_year+1900, tm.tm_mon+1, tm.tm_mday,
                                tm.tm_hour, tm.tm_min, tm.tm_sec,
                                (int) now.tv_nsec / 1000000);
                            fwrite(date, 1, strlen(date), lf);
                        }

                        fwrite(logbuf, 1, strlen(logbuf), lf);

                        // ToDo
                        if( lblen > MAX_LENGTH - 2 ) {
                            lblen = MAX_LENGTH - 2;
                            free(logbuf);
                            //lb = logbuf = (char *)realloc(
                            //    logbuf, sizeof(char) * (MAX_LENGTH));
                            lb = logbuf = (char*)malloc(MAX_LENGTH);
                        }

                        memset( lb = logbuf, '\0', MAX_LENGTH );
                    }
                }
            }

            if( 0x0a==c ) {
                //excflag = false;
                //transmission(STDOUT_FILENO, comm, sprintf(comm, "%s", RESET));
                transmission(STDOUT_FILENO, RESET, strlen(RESET));
            }

            //if( 0x21==c && cflag && !excflag ) {
            //    comlen = 0;
            //    excflag = true;
            //    transmission(STDOUT_FILENO, comm, sprintf(comm, "\b%s%c", COLOR_COMMENT, c));
            //}

            //if( excflag && 0x07!=c )
            //    comlen++;

            if( 0x07==c )
                bsflag = false;

            //if( 0x08==c ) {
            //    if( excflag )
            //        comlen-=2;
            //    if( excflag && 0>=comlen ) {
            //        transmission(STDOUT_FILENO, comm, sprintf(comm, "%s", RESET));
            //        //memset( io = s, '\0', MAX_LENGTH );
            //        io++;
            //        excflag = false;
            //    }
            //}

            //if( !excflag && cflag )
            if( cflag )
                coloring(c);
        }

        // if new data is available on the console, send it to the serial port
        if(read(STDIN_FILENO, &c, 1) > 0) {
            if( 0x1b==c )                 escflag = true; // ^
            else if( 0x5b==c && escflag ) spflag  = true; // ^[
            else if( 0x33==c && spflag )  tilflag = true; // ^[3
            else if( 0x7e==c && tilflag ) {               // ^[3~
                c = 0x7f;
                escflag = spflag = tilflag = false;
            }
            else {
                escflag = spflag = false;
            }

            if( endcode == c )                   break; // hang up
            if( 0x00 == c )                  c = 0x7f;  // BS on Vimterminal

            if( 0x7f == c )             bsflag = 3;     // BS on Vimterminal

            //if( '$' == c ) DebugLog("[lblen:%d]", lblen);
            //DebugLog("[0x%02x]", c);
            transmission(fd, &c, 1);
        }

        // 100 microsecond
        usleep(100);
    }

    close(fd);

    if(logflag) {
        if( ts ) {
            clock_gettime(CLOCK, &now);
            localtime_r(&now.tv_sec, &tm);
            sprintf(date, "[%d-%02d-%02d %02d:%02d:%02d.%03d] ",
                tm.tm_year+1900, tm.tm_mon+1, tm.tm_mday,
                tm.tm_hour, tm.tm_min, tm.tm_sec,
                (int) now.tv_nsec / 1000000);
            fwrite(date, 1, strlen(date), lf);
        }
        char loglast[strlen(logbuf)+1];
        sprintf(loglast, "%s%c", logbuf, 0x0a);
        fwrite(loglast, 1, strlen(loglast), lf);
        fclose(lf);
    }

    tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
    printf("%s\nDisconnected.\n", RESET);

    return EXIT_SUCCESS;
}

int ishex(char c) {
    return ('a' <= c && c <= 'f') ||
           ('A' <= c && c <= 'F') ||
           ('0' <= c && c <= '9');
}

int chrcnt(int num) {
    int cnt = 1;
    while(num /= 10)
        ++cnt;
    return cnt;
}

char *loopc(const char c, int n) {
    char *str = (char*)malloc(n+1);
    int i;
    for(i=0; i<n; ++i)
        str[i] = c;
    str[i] = '\0';
    return str;
}

void pack_space_cpy(char *dstaddr, const char *addr) {
    int i = 0;
    const char *p = addr;
    while(*p) {
        if(isspace(*p)) {
            ++p;
            continue;
        }
        if(i < 21)
            dstaddr[i++] = *p;
        ++p;
    }
    dstaddr[i] = '\0';
}

bool correct_ipaddr_format(const char *addr) {
    // XXX.XXX.XXX.XXX:XXXXX || XXX.XXX.XXX.XXX
    enum { OUT_OF_RANGE = -2 };
    regex_t preg;
    const char *format = "^ *(2[0-4][0-9]|1[0-9]{2}|[1-9][0-9]|[1-8])[.]((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9]) *(: *[1-9][0-9]{4}|: *[1-9][0-9]{3}|: *[1-9][0-9]{2}|: *[1-9][0-9]{1}|: *[0-9]|)$";
    int rc = regcomp(&preg, format, REG_NOSUB | REG_EXTENDED | REG_NEWLINE);
    if(rc != 0) {
        char msg[100];
        regerror(rc, &preg, msg, 100);
        regfree(&preg);
        error("%serror:%s Failed regcomp(): %s\"%S\"%s\n", ERROR_RED, RESET, ERROR_YELLOW, msg, RESET);
        return false;
    }
    rc = regexec(&preg, addr, 0, 0, 0);
    regfree(&preg);

    if(rc == 0) {
        if(pull_port_num(addr) == OUT_OF_RANGE)
            return false;
        return true;
    }
    return false;
}

void store_address(char *addr, const char *dstaddr) {
    sscanf(dstaddr, "%15[^:\n]", addr);
    addr[strlen(addr)] = '\0';
}

int pull_port_num(const char *addr) {
    enum {
        NONE_PORT    = -1,
        OUT_OF_RANGE = -2,
        MAX_PORT_NUM = 65535,
    };
    int port;

    if(sscanf(addr, "%*[^:]:%d", &port) != 1)
        return NONE_PORT;
    if(port > MAX_PORT_NUM)
        return OUT_OF_RANGE;
    return port;
}

void transmission(int _fd, const void* _buf, size_t _len) {
    if( -1 == write(_fd, _buf, _len) ) {
        error("%serror:%s Failed write()\n", ERROR_RED, RESET);
        //exit(EXIT_FAILURE);
    }
}


int kbhit() {
    struct termios oldt, newt;
    int ch, oldf;
    tcgetattr(STDIN_FILENO, &oldt);
    newt = oldt;
    newt.c_lflag &= ~(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &newt);
    oldf = fcntl(STDIN_FILENO, F_GETFL, 0);
    fcntl(STDIN_FILENO, F_SETFL, oldf | O_NONBLOCK);

    ch = getchar();

    tcsetattr(STDIN_FILENO, TCSANOW, &oldt);
    fcntl(STDIN_FILENO, F_SETFL, oldf);

    if (ch != EOF) {
        ungetc(ch, stdin);
        return 1;
    }
    return 0;
}

void replace(char *str, const char *before, const char *after) {
    char *p;
    while((p = strstr(str, before)) != NULL) {
        *p = '\0';
        p += (int)strlen(before);
        strcat(str, after);
        strcat(str, p);
    }
}

int syntaxCheck(char *str) {
    int hi_num;
    for(hi_num = 0; hi_num < params_len; ++hi_num)
        if( regexec(&params[hi_num].regex, str, 0, 0, 0) == 0 )
            return hi_num;
    return -1;
}

void repaint(const char *color) {
    io = s;
    int  i  = 0;
    char bs = 0x08;
    char tmp[MAX_LENGTH];
    char str[MAX_LENGTH + 32];
    while(*io) {
        tmp[i++] = *io++;
        transmission(STDOUT_FILENO, &bs, 1);
    }
    if(tmp[i]!='\0') {
        tmp[i]='\0';
    }
    transmission(STDOUT_FILENO, str, sprintf(str, "%s%s%s", color, tmp, RESET));
}

void coloring(char c) {
    if( (/*0x08!=c &&*/ 0x21>c && !bsflag) ) {
        memset( io = s, '\0', MAX_LENGTH );
        return;
    }

    //en route
    if( bsflag ) {
        if( 2==bsflag-- /*&& '\0'!=s[0]*/)
            *io++ = '\0';
            //*io++ = 0x20;
        else
            io--;

        if( bsflag )
            return;
    }
    /*else if( 0x08==c ) {
        io--;
        if( '\0'==s )
            memset( io = s, '\0', MAX_LENGTH );
        return;
    } //*/
    else if( strlen(s) < MAX_LENGTH - 1 ) {
        *io++ = c;
    }
    else {
        memset( io = s, '\0', MAX_LENGTH );
        return;
    }

    int checked = syntaxCheck(s);
    if(checked >= 0)
        repaint(params[checked].color);

    //        case HL_SLASH:
    //            sprintf(s, "/");
    //            repaint(COLOR_SLASH);
    //            memset( io = s, '\0', sizeof(s) );
    //            break;

}

void setSignal(int p_signame) {
    if( signal(p_signame, sigcatch) == SIG_ERR ) {
        error("%serror:%s SIG_ERR\n", ERROR_RED, RESET);
        exit(EXIT_FAILURE);
    }
}

void sigcatch() {
    exit(EXIT_SUCCESS);
}

void abort_exit(int fd, int when, const struct termios *termptr) {
    tcsetattr(fd, when, termptr);
    exit(EXIT_FAILURE);
}

void version() {
    printf("%s (%s) %s %s\n", COMMAND_NAME, PROGRAM_NAME, VERSION, UPDATE_DATE);
}

void usage(char *argv[]) {
    printf("Usage: %s [-l SERIAL_PORT] [-s BAUDRATE] [-r /path/to/file]\n"
           "            [-w /path/to/LOG] [-c /path/to/config] [-t] [-a] [-n] [-h] [-v]\n"
           "            [-p IPAddress[:port]]\n\n", argv[0]);

    printf("Command line interface for Serial Console by Network device.\n");
    printf("------------------------------------------------------------\n");
    printf("https://github.com/yorimoi/sisterm\n\n");

    printf("Options:\n");
    printf("  -h,--help          Show this help message and exit\n");
    printf("  -v,--version       Show %s version and exit\n", PROGRAM_NAME);
    printf("  -l port            Use named device   (e.g. /dev/ttyS0)\n");
    printf("  -s speed           Use given speed    (default 9600)\n");
    printf("  -r path            Output log file    (e.g. /tmp/config.txt)\n");
    printf("  -w path            Saved log          (e.g. /tmp/sist.log)\n");
    printf("  -t                 Add timestamp to log\n");
    printf("  -a                 Append to log      (default overwrite)\n");
    printf("  -n                 Without color\n");
    printf("  -c path            Specification of config file (e.g. /tmp/for_cisco.conf)\n");
    printf("  -p address[:port]  Telnet !!!Beta version!!! Many bugs!\n\n");

    printf("Commands:\n");
    printf("  ~           Terminate the conversation\n");
}

