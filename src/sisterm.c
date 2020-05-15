
#define COMMAND_NAME   "sist"
#define PROGRAM_NAME   "sisterm"
#define VERSION        "1.5.0"
#define UPDATE_DATE    "2020XXXX"

#define CONFIG_FILE    "sist.conf"
#define MAX_PARAM_LEN  2048

#include "sisterm.h"
#include "sist_error.h"
#include "palette.h"


#ifdef __linux__
// CLOCK_REALTIME_COARSE
#define              CLOCK 5
#else
#define              CLOCK CLOCK_REALTIME
#endif

#define MAX_LENGTH   256
#define REG_FLAGS    REG_EXTENDED | REG_NOSUB | REG_ICASE

#define DEBUGLOG sisterr("[%d]", __LINE__)

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

//char getch_(char *c) {
//    return NULL;
//}

int main(int argc, char **argv) {
    char *serial_port     = NULL;             // SerialPort
    char *baud_rate       = NULL;             // BaudRate
    char *read_file_path  = NULL;             // File path to load
    char *write_file_path = NULL;             // File path to save
    char *cfg_file_path   = NULL;             // File path to config
    speed_t baudRate      = B9600;            // Default BaudRate
    bool existsflag       = false;            // Whether to log file
    //bool excflag          = false;            // Exclamation mark flag for comment
    //int  comlen           = 0;                // Comment length
    bool escflag          = false;            // '^'
    bool spflag           = false;            // '['
    bool tilflag          = false;            // Del key -> BS key
    bool arrflag          = false;            // Arrow keys flag
    bool logflag          = false;            // Whether to take a log
    bool tcpflag          = false;            // TCP
    bool wflag            = false;            // Write file Flag
    bool rflag            = false;            // Read file Flag
    bool another_conf     = false;            // another config file
    bool cflag            = true;             // Color Flag
    bool ts               = false;            // Whether to timestamp
    char* logbuf          = (char*)malloc(MAX_LENGTH);
    char* lb              = logbuf;           // Log buffer pointer for operation
    int  lblen            = MAX_LENGTH - 2;
    //char                  comm[32];           // For comment
    FILE *lf              = NULL;             // Log file
    char mode[]           = "w+";             // Log file open mode
    char dstaddr[21+1];
    const char CR         = 0x0d;             // Carriage Return
    const uint16_t default_port = 23;         // TELNET
    int port;


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
        while ((opt = getopt_long(
                argc,
                argv,
                "l:s:r:w:c:tanp:hv",
                longopts,
                &idx)
        ) != -1) {
            switch (opt) {
                case 'l':
                  // /path/to/SerialPort
                    serial_port = (char*)malloc(strlen(optarg)+1);
                    strcpy(serial_port, optarg);
                    break;

                case 's':
                  // BaudRate speed
                    baud_rate = (char*)malloc(strlen(optarg)+1);
                    strcpy(baud_rate, optarg);
                    break;

                case 'r':
                  // /path/to/config.txt
                    read_file_path = (char*)malloc(strlen(optarg)+1);
                    strcpy(read_file_path, optarg);
                    rflag = true;
                    break;

                case 'w':
                  // /path/to/log.txt
                    write_file_path = (char*)malloc(strlen(optarg)+1);
                    strcpy(write_file_path, optarg);
                    wflag = true;
                    break;

                case 'c':
                  // /path/to/config
                    cfg_file_path = (char*)malloc(strlen(optarg)+1);
                    strcpy(cfg_file_path, optarg);
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
                  // XXX.XXX.XXX.XXX:XXXXX || hostname:XXXXX
                    tcpflag = true;
                    if (correct_ipaddr_format(optarg)) {
                        pack_space_cpy(dstaddr, optarg);
                        if ((port = pull_port_num(dstaddr)) < 0) {
                            port = default_port;
                        }
                    }
                    else if (hosttoip(dstaddr, optarg)) {
                        char *packed_arg = malloc(strlen(optarg)+1);
                        pack_space_cpy(packed_arg, optarg);
                        if ((port = pull_port_num(packed_arg)) < 0) {
                            port = default_port;
                        }
                        free(packed_arg);
                    }
                    else {
                        sisterr("%serror:%s Bad address or hostname or port number: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, optarg, RESET);
                        return EXIT_FAILURE;
                    }
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
                    sisterr("Use %s -h for help\n", argv[0]);
                    return EXIT_FAILURE;
              }
         }
    }



    if ( serial_port == NULL && !rflag && !tcpflag ) {
        sisterr("%serror:%s must specify Serial Port\n", E_RED, RESET, argv[0]);
        return EXIT_FAILURE;
    }


    if ( baud_rate != NULL && !rflag && !tcpflag ) {
        if     (!strcmp(baud_rate, "0"))      baudRate = B0;      // hang up
        else if (!strcmp(baud_rate, "50"))     baudRate = B50;
        else if (!strcmp(baud_rate, "75"))     baudRate = B75;
        else if (!strcmp(baud_rate, "110"))    baudRate = B110;
        else if (!strcmp(baud_rate, "134"))    baudRate = B134;
        else if (!strcmp(baud_rate, "150"))    baudRate = B150;
        else if (!strcmp(baud_rate, "200"))    baudRate = B200;
        else if (!strcmp(baud_rate, "300"))    baudRate = B300;
        else if (!strcmp(baud_rate, "600"))    baudRate = B600;
        else if (!strcmp(baud_rate, "1200"))   baudRate = B1200;
        else if (!strcmp(baud_rate, "1800"))   baudRate = B1800;
        else if (!strcmp(baud_rate, "2400"))   baudRate = B2400;
        else if (!strcmp(baud_rate, "4800"))   baudRate = B4800;
        else if (!strcmp(baud_rate, "9600"))   baudRate = B9600;   // Default
        else if (!strcmp(baud_rate, "19200"))  baudRate = B19200;
        else if (!strcmp(baud_rate, "38400"))  baudRate = B38400;
        else if (!strcmp(baud_rate, "57600"))  baudRate = B57600;
        else if (!strcmp(baud_rate, "115200")) baudRate = B115200;
        else if (!strcmp(baud_rate, "230400")) baudRate = B230400;
        else {
          sisterr("%serror:%s Invalid BaudRate: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, baud_rate, RESET);
          return EXIT_FAILURE;
        }
    }


    if ( wflag && !rflag ) {
        if (!access(write_file_path, F_OK)) {
            existsflag = true;
        }

        if ( existsflag && (access( write_file_path, (F_OK | R_OK) ) < 0) ) {
            sisterr("%serror:%s Access to the log file is denied\n", E_RED, RESET);
            return EXIT_FAILURE;
        }

        if ( existsflag && !strcmp(mode, "w+") ) {
            sisterr("\a%s\"%s\"%s is already exists!\n", E_YELLOW, write_file_path, RESET);
            sisterr("Do you want to overwrite?[confirm]");
            char con = getchar();
            if ( !(con=='\n' || con=='y' || con=='Y') ) {
                return EXIT_SUCCESS;
            }
        }

        lf = fopen(write_file_path, mode);

        if (lf == NULL) {
            if (access(write_file_path, F_OK)) {
              sisterr("%serror:%s Failed to create file: Try to check the permissions\n", E_RED, RESET);
              return EXIT_FAILURE;
            }
            else if ( access( write_file_path, (F_OK | R_OK) ) < 0 ) {
              sisterr("%serror:%s Access to the log file is denied\n", E_RED, RESET);
              return EXIT_FAILURE;
            }

            sisterr("%s: open (%s): Failure\n", argv[0], write_file_path);
            sisterr("%serror:%s file open Failure: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, write_file_path, RESET);
            return EXIT_FAILURE;
        }

        logflag = 1;
    }


    {
        FILE *cfp;  // Config File Pointer
        char *path;

        if (another_conf) {
            path = (char*)malloc(strlen(cfg_file_path)+1);
            strcpy(path, cfg_file_path);
            path[strlen(cfg_file_path)] = '\0';
        } else {
            int len = strlen(getenv("HOME")) + 1 + strlen(CONFIG_FILE);
            path = (char*)malloc(len+1);
            memset(path, '\0', len+1);
            sprintf(path, "%s/%s", getenv("HOME"), CONFIG_FILE);
        }

        cfp = fopen(path, "r");

        if (cfp == NULL) {
            cflag = false;
            sisterr("%serror:%s File open error: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, path, RESET);
            sisterr("Press ENTER to continue of without color mode");
            (void)getchar();
        }
        else {
            params = (Param*)malloc(sizeof(Param));
            char *str = (char*)malloc(MAX_PARAM_LEN);
            int line = 0;
            int i, failed;

            while (fgets(str, MAX_PARAM_LEN, cfp) != NULL) {
                ++line;
                char top = '\0';
                sscanf(str, " %c", &top);
                // ignore comment and blank line
                if (strchr(" #\n\0", top)) {
                    continue;
                }

                char *key   = (char*)calloc(64, sizeof(char)),
                     *op    = (char*)calloc(2+1, sizeof(char)),
                     *name  = (char*)calloc(64, sizeof(char)),
                     *param = (char*)calloc(MAX_PARAM_LEN, sizeof(char));

                sscanf(str, " %63[^ .] . %63[^ +=] %2[+=] %2047[^\n]", name, key, op, param);
                //printf("[name:%s, key:%s, op:%s, param:%s]\n", name, key, op, param);

                bool suffer = false;
                for (i=0; i<params_len; ++i) {
                    if (!strcmp(params[i].name, name)) {
                        suffer = true;
                        break;
                    }
                }
                if (!suffer) {
                    if (!strcmp(op, "+=")) {
                        int cnt = numlen(line);
                        sisterr("%serror:%s '%s%s.%s%s' is used uninitialized\n", E_RED, RESET, E_YELLOW, name, key,RESET);
                        sisterr("  %s%*s->%s %s:%d\n", E_BLUE, cnt, "-", RESET, path, line);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    ++params_len;
                    Param *params_tmp = (Param*)realloc(params, params_len * sizeof(Param));
                    if (params_tmp == NULL) {
                        free(params);
                        sisterr("%serror:%s realloc() failed\n", E_RED, RESET);
                        return EXIT_FAILURE;
                    }
                    params = params_tmp;
                    params[params_len-1].name = (char*)malloc(strlen(name)+1);
                    strcpy(params[params_len-1].name, name);
                    params[params_len-1].name[strlen(name)] = '\0';
                }

                // DOS file format
                if (param[strlen(param)-1] == 0x0D) {
                    param[strlen(param)-1] = '\0';
                }

                if (param[0] == '\0') {
                    int cnt = numlen(line);
                    sisterr("%serror:%s Value required after '=':\n", E_RED, RESET);
                    sisterr("  %s%*s->%s %s:%d\n", E_BLUE, cnt, "-", RESET, path, line);
                    sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                    sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                    sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                    return EXIT_FAILURE;
                }

                if (!strcmp(key, "color")) {
                    bool color_flug = false,
                         isColor_24 = false;

                    for (i=0; i<AC_MAX; ++i) {
                        if (!strcasecmp(param, ansi_colors[i].key)) {
                            color_flug = true;
                            params[params_len-1].color = (char*)malloc(strlen(ansi_colors[i].val)+1);
                            strcpy(params[params_len-1].color, ansi_colors[i].val);
                            params[params_len-1].color[strlen(ansi_colors[i].val)] = '\0';
                            break;
                        }
                    }

                    if (!color_flug) {

                        if (strlen(param) == 6 || strlen(param) == 7) {
                            char param_tmp[7+1];
                            int i;
                            strcpy(param_tmp, param);

                            if ((strlen(param) == 7) && param[0] == '#') {
                                for (i=0; i<6; ++i) {
                                    param_tmp[i] = param_tmp[i+1];
                                }
                                param_tmp[i] = '\0';
                            }

                            isColor_24 = true;
                            for (i=0; i<6; ++i) {
                                if (!ishex(param_tmp[i])) {
                                    isColor_24 = false;
                                    break;
                                }
                            }
                            if (isColor_24) {
                                char hexs[3][2+1] = {
                                    { param_tmp[0], param_tmp[1], '\0' },
                                    { param_tmp[2], param_tmp[3], '\0' },
                                    { param_tmp[4], param_tmp[5], '\0' }};
                                char format[19+1];  // \033[38;2;XXX;XXX;XXXm
                                snprintf(format, sizeof(format), "\033[38;2;%03ld;%03ld;%03ldm",
                                strtol(hexs[0], NULL, 16),
                                strtol(hexs[1], NULL, 16),
                                strtol(hexs[2], NULL, 16));
                                params[params_len-1].color = (char*)malloc(strlen(format)+1);
                                strcpy(params[params_len-1].color, format);
                                params[params_len-1].color[strlen(format)] = '\0';
                            }
                        }

                        if (isColor_24) {
                            // Do nothing
                        }
                        else if (strlen(param) >= 6) {
                            char *p;
                            char *param_buf = (char*)malloc(strlen(param)+1);
                            strcpy(param_buf, param);
                            p = param_buf;
                            int i = 0;
                            while (*p) {
                                if (isspace(*p)) {
                                    ++p;
                                    continue;
                                }
                                param_buf[i] = *p;
                                ++i;
                                ++p;
                            }
                            param_buf[i] = '\0';
                            if (param_buf[0] == '"' || param_buf[i-1] == '"') {
                                int cnt = numlen(line);
                                sisterr("%serror:%s Invalid color: '%s%s%s': expected not to require '\"'\n", E_RED, RESET, E_YELLOW, param, RESET);
                                sisterr("  %s%*s->%s %s:%d\n", E_BLUE, cnt, "-", RESET, path, line);
                                sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                                sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                return EXIT_FAILURE;
                            }
                            if (param_buf[i-1] != 'm') {
                                int cnt = numlen(line);
                                sisterr("%serror:%s Invalid color: '%s%s%s': expected 'm' in end\n", E_RED, RESET, E_YELLOW, param, RESET);
                                sisterr("  %s%*s->%s %s:%d\n", E_BLUE, cnt, "-", RESET, path, line);
                                sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                                sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                return EXIT_FAILURE;
                            }

                            replace(param_buf, "\\033", "\033");
                            replace(param_buf, "\\e",   "\x1B");
                            replace(param_buf, "\\x1b", "\x1b");
                            replace(param_buf, "\\x1B", "\x1B");

                            strcpy(param, param_buf);
                            free(param_buf);

                            if (!strcmp(op, "+=")) {
                                char *param_tmp = (char*)realloc(params[params_len-1].color, strlen(params[params_len-1].color)+strlen(param)+1);
                                if (param_tmp == NULL) {
                                    sisterr("%serror:%s realloc() failed\n", E_RED, RESET);
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
                        else if (strlen(param) == 3) {
                            int i;
                            for (i=0; i<3; ++i) {
                                if (!isdigit(param[i])) {
                                    int cnt = numlen(line);
                                    sisterr("%serror:%s Invalid color: '%s%s%s'\n", E_RED, RESET, E_YELLOW, param, RESET);
                                    sisterr(" %s%*s->%s %s:%d\n", E_BLUE, cnt, "-", RESET, path, line);
                                    sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                    sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                                    sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                    return EXIT_FAILURE;
                                }
                            }
                            u_int16_t num = strtol(param, NULL, 10);
                            if (num > 255) {
                                int cnt = numlen(line);
                                sisterr("%serror:%s Invalid color: '%s%s%s': less than 256\n", E_RED, RESET, E_YELLOW, param, RESET);
                                sisterr(" %s%*s->%s %s:%d\n", E_BLUE, cnt, "-", RESET, path, line);
                                sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                                sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                                return EXIT_FAILURE;
                            }
                            char format[11+1];  // \033[38;5;XXXm
                            snprintf(format, sizeof(format), "\033[38;5;%3sm", param);
                            params[params_len-1].color = (char*)malloc(strlen(format)+1);
                            strcpy(params[params_len-1].color, format);
                            params[params_len-1].color[strlen(format)] = '\0';
                        }
                        else {
                            int cnt = numlen(line);
                            sisterr("%serror:%s Invalid color: '%s%s%s'\n", E_RED, RESET, E_YELLOW, param, RESET);
                            sisterr(" %s%*s->%s %s:%d\n", E_BLUE, cnt, "-", RESET, path, line);
                            sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                            sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                            sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                            return EXIT_FAILURE;
                        }
                    }
                }
                else if (!strcmp(key, "regex")) {
                    if (param[0] == '"' || param[strlen(param)-1] == '"') {
                        int cnt = numlen(line);
                        sisterr("%serror:%s Invalid regex: '%s%s%s': expected not to require '\"'\n", E_RED, RESET, E_YELLOW, param, RESET);
                        sisterr(" %s%*s->%s %s:%d\n", E_BLUE, cnt, "-",  RESET, path, line);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    int rc;
                    if ((rc = regcomp(&params[params_len-1].regex, param, REG_FLAGS))) {
                        int cnt = numlen(line);
                        char msg[100];
                        regerror(rc, &params[params_len-1].regex, msg, 100);
                        sisterr("%serror:%s regcomp() failred: %s\n", E_RED, RESET, msg);
                        sisterr(" %s%*s->%s %s:%d\n", E_BLUE, cnt, "-",  RESET, path, line);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    if (!strcmp(op, "+=")) {
                        int cnt = numlen(line);
                        sisterr("%serror:%s The \"+=\" operator can\'t be used with regex\n", E_RED, RESET);
                        sisterr(" %s%*s->%s %s:%d\n", E_BLUE, cnt, "-",  RESET, path, line);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                        sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                        return EXIT_FAILURE;
                    }
                    params[params_len-1].cmped = true;
                }
                else {
                    int cnt = numlen(line);
                    sisterr("%serror:%s Neither color nor regex: '%s%s%s'\n", E_RED, RESET, E_YELLOW, key, RESET);
                    sisterr(" %s%*s->%s %s:%d\n", E_BLUE, cnt, "-",  RESET, path, line);
                    sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
                    sisterr("%s%d |%s %s", E_BLUE, line, RESET, str);
                    sisterr(" %*s%s|%s\n", cnt, "", E_BLUE, RESET);
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
            for (i=0, failed=0; i<params_len; ++i) {
                if (params[i].color == NULL) failed = 1;
                if (params[i].cmped == 0)    failed = 2;
                if (failed) {
                    sisterr("%serror:%s %s.%s is not defined\n", E_RED, RESET, params[i].name, failed==1?"color":"regex");
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

    // logは多重しない 1.4.5で修正予定
    //memset(&stdio, 0, sizeof(stdio));
    memcpy(&stdio, &old_stdio, sizeof(stdio));
    //stdio.c_iflag     = 0;
    //stdio.c_oflag     = 0;
    //stdio.c_cflag     = 0;
    stdio.c_lflag     = 0;
    stdio.c_lflag     &= ~(ECHO | ICANON);
    stdio.c_cc[VMIN]  = 1;
    stdio.c_cc[VTIME] = 10;
    tcsetattr(STDOUT_FILENO, TCSANOW,&stdio);
    tcsetattr(STDOUT_FILENO, TCSAFLUSH,&stdio);
    fcntl(STDIN_FILENO, F_SETFL, O_NONBLOCK);

    memset(&tio, 0, sizeof(tio));
    tio.c_iflag       = 0;
    tio.c_oflag       = 0;
    tio.c_cflag       = CS8 | CREAD | CLOCAL;
    tio.c_lflag       = 0;
    tio.c_cc[VMIN]    = 1;
    tio.c_cc[VTIME]   = 10;

    fd = open(serial_port, O_RDWR | O_NONBLOCK);
    if ( fd < 0 && !rflag && !tcpflag ) {
        tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
        if (access( serial_port, F_OK ) < 0) {
            sisterr("%serror:%s No such file or directory: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, serial_port, RESET);
        } else if (access( serial_port, (R_OK | W_OK) ) < 0) {
            sisterr("%serror:%s Permission denied: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, serial_port, RESET);
        // unstable
        //else if (fcntl(fd, F_GETFL) == -1)
        //  printf("%s: %s: Line in use\n", argv[0], serial_port);
        } else {
            sisterr("%serror:%s File open failure: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, serial_port, RESET);
        }
        close(fd);
        return EXIT_FAILURE;
    }

    if ( cfsetispeed(&tio, baudRate) != 0 ) return EXIT_FAILURE;
    if ( cfsetospeed(&tio, baudRate) != 0 ) return EXIT_FAILURE;

    if ( rflag && !tcpflag ) {
        FILE *fr;
        fr = fopen(read_file_path, "r");
        if (fr == NULL) {
            tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
            if (access( read_file_path, F_OK ) < 0) {
                sisterr("%serror:%s No such file or directory: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, read_file_path, RESET);
            } else if (access( read_file_path, (R_OK) ) < 0) {
                sisterr("%serror:%s Permission denied: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, read_file_path, RESET);
            } else {
                sisterr("%serror:%s File open failure: %s\"%s\"%s\n", E_RED, RESET, E_YELLOW, read_file_path, RESET);
            }
            return EXIT_FAILURE;
        }

        //if ( setvbuf(stdout, NULL, _IOLBF, 2048) != 0 )
        //{
        //  /* If failure without buffering */
        //}

        tcsetattr(fd, TCSANOW, &tio);

        int i;
        while ((i=fgetc(fr)) != EOF) {
            c = (char)i;
            if ( 0x07==c || 0x08==c || 0x0a==c || 0x0d==c || (0x1f<c && 0x7f>c) ) {
                transmission(STDOUT_FILENO, &c, 1);
            }

            if ( 0x0a==c ) {
                transmission(STDOUT_FILENO, &CR, 1);
                transmission(STDOUT_FILENO, RESET, strlen(RESET));
            }

            if ( cflag ) {
                coloring(c);
            }

            if (read(STDIN_FILENO, &c, 1) > 0) {
                if (c == endcode) break;
            }
        }

        tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);
        fclose(fr);
        printf("\n%s", RESET);

        return EXIT_SUCCESS;
    }

    /* ----------------------------------------------------------------------- */
    // 分割したい
    if ( tcpflag ) {
        //tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);

        struct sockaddr_in sa;

//#include <sys/select.h>
//        struct timeval t_val = {0, 1000};
//        fd_set fds, readfds;
//        int select_ret;

        if ( (fd = socket(AF_INET, SOCK_STREAM, 0)) < 0 ) {
            sisterr("%serror:%s Failed socket()\n", E_RED, RESET);
            return EXIT_FAILURE;
        }

        char *address = (char*)malloc(15+1);
        store_address(address, dstaddr);

        memset(&sa, 0, sizeof(sa));
        sa.sin_family = AF_INET;
        sa.sin_port = htons(port);
        sa.sin_addr.s_addr = inet_addr(address);

        free(address);

        if ( sa.sin_addr.s_addr == 0xffffffff ) {
            sisterr("%serror:%s Bad address\n", E_RED, RESET);
            return EXIT_FAILURE;
        }

        if ( connect(fd, (struct sockaddr *)&sa, sizeof(sa)) != 0) {
            sisterr("%serror:%s Connection refused\n", E_RED, RESET);
            close(fd);
            return EXIT_FAILURE;
        }

        printf("Connected.\n");

        //FD_ZERO(&readfds);
        //FD_SET(fd, &readfds);

        tcsetattr(fd, TCSANOW, &tio);

        while (true) {
            //memcpy(&fds, &readfds, sizeof(fd_set));
            //select_ret = select(0, &fds, NULL, NULL, &t_val);

            //if (select_ret != 0) {
            if ( recv(fd, &c, 1, 0) > 0 ) {
                if ( 0x07==c || 0x08==c || 0x0a==c || 0x0d==c || (0x1f<c && 0x7f>c) ) {
                    transmission(STDOUT_FILENO, &c, 1);
                }

                if ( logflag ) {
                    if ( (int)strlen(logbuf) > lblen ) {
                        char *lb_tmp = (char*)realloc(logbuf, sizeof(char) * (lblen += MAX_LENGTH));
                        if (lb_tmp == NULL) {
                            free(logbuf);
                            sisterr("%serror:%s Failed realloc()\n");
                            abort_exit(STDOUT_FILENO, TCSANOW, &old_stdio);
                        }
                        lb = logbuf = lb_tmp;
                    }

                    if ( 0x08==c ) {
                        //if ( lb != logbuf ) // Sent 0x07 from device
                        lb--;
                    }
                    else if ( 0x1f<c && 0x7f>c ) {
                        *lb = c;
                        ++lb;
                    }
                    else if ( '\n'==c ) {
                        logbuf[strlen(logbuf)] = '\n';

                        if ( ts ) {
                            fwritets(lf);
                        }

                        fwrite(logbuf, 1, strlen(logbuf), lf);
                        fflush(lf);

                        if ( lblen > MAX_LENGTH - 2 ) {
                            lblen = MAX_LENGTH - 2;
                            char *lb_tmp = (char*)realloc(logbuf, sizeof(char) * (MAX_LENGTH));
                            if (lb_tmp == NULL) {
                                free(logbuf);
                                sisterr("%serror:%s Failed realloc()\n", E_RED, RESET);
                                abort_exit(STDOUT_FILENO, TCSANOW, &old_stdio);
                            }
                            lb = logbuf = lb_tmp;
                        }

                        memset( lb = logbuf, '\0', MAX_LENGTH );
                    }
                }

                if ( 0x0a==c ) {
                    transmission(STDOUT_FILENO, RESET, strlen(RESET));
                }

                if ( cflag ) {
                    coloring(c);
                }

            }
            //else if ( recv(fd, &c, 1, 0) == 0) {
                //kill(p_pid, SIGINT);
            //    break;  // hang up
            //}

            //else {
            //else if (fgets(&c, 1, stdin) != NULL) {
            //else if (getch_(&c) != NULL) {
            //else if ( kbhit() ) {
                //DEBUGLOG;
            if ( kbhit() ) {
                DEBUGLOG;
                c = getchar();
                if ( 0x1b==c )                          escflag = true;  // ^
                else if ( escflag && 0x5b==c )          spflag  = true;  // ^[
                else if ( spflag  && 0x33==c )          tilflag = true;  // ^[3
                else if ( spflag  && 0x40<c && 0x45>c ) arrflag = true;  // ^[[ABCD]
                else if ( tilflag && 0x7e==c ) {                         // ^[3~
                    c = 0x7f;
                    escflag = spflag = tilflag = false;
                }
                else {
                    escflag = spflag = false;
                }

                if ( endcode == c ) {
                    //kill(pid, SIGINT);
                    break;  // hang up
                }

                if ( 0x00 == c ) {
                    c = 0x7f;  // BS on Vimterminal
                }

                if ( !escflag ) {
                    send(fd, &c, 1, 0);
                }

                if ( arrflag ) {
                    char* arrow = (char*)malloc(4);
                    sprintf(arrow, "%c%c%c", 0x1b, 0x5b, c);
                    send(fd, arrow, 3, 0);
                    free(arrow);
                    arrflag = false;
                }
            }
            //}

            // 100 microsecond
            usleep(100);
        }

        if (logflag) {
            if ( ts ) {
                fwritets(lf);
            }
            char *loglast = (char*)malloc(strlen(logbuf)+2);
            sprintf(loglast, "%s\n", logbuf);
            fwrite(loglast, 1, strlen(loglast), lf);
            fflush(lf);
            fclose(lf);
        }

        tcsetattr(STDOUT_FILENO, TCSANOW, &old_stdio);

        printf("%s\nDisconnected.\n", RESET);
        close(fd);
        return EXIT_SUCCESS;
    }

    /* ----------------------------------------------------------------------- */

    printf("Connected.\n");

    tcsetattr(fd, TCSANOW, &tio);

    while (true) {
        // if new data is available on the serial port, print it out
        // ToDo Parallel processing
        if (read(fd, &c, 1) > 0) {
            if ( 0x07==c || 0x08==c || 0x0a==c || 0x0d==c || (0x1f<c && 0x7f>c) ) {
                transmission(STDOUT_FILENO, &c, 1);
            }

            if ( logflag ) {
                // Unstable
                if ( (int)strlen(logbuf) > lblen ) {
                    char *lb_tmp = (char*)realloc(logbuf, sizeof(char) * (lblen += MAX_LENGTH));
                    if (lb_tmp == NULL) {
                        free(logbuf);
                        sisterr("%serror:%s Failed realloc()\n", E_RED, RESET);
                        abort_exit(STDOUT_FILENO, TCSANOW, &old_stdio);
                    }
                    lb = logbuf = lb_tmp;
                }

                if ( 0x08==c ) {
                    //if ( lb != logbuf ) // Sent 0x07 from device
                    --lb;
                }
                else if ( 0x1f<c && 0x7f>c ) {
                  *lb = c;
                  ++lb;
                }
                else if ( '\n'==c ) {
                    logbuf[strlen(logbuf)] = '\n';
                    //if ( 0x0a==c )
                    {
                        if ( ts ) {
                            fwritets(lf);
                        }

                        fwrite(logbuf, 1, strlen(logbuf), lf);

                        // ToDo
                        if ( lblen > MAX_LENGTH - 2 ) {
                            lblen = MAX_LENGTH - 2;
                            // reallocにする
                            free(logbuf);
                            //lb = logbuf = (char *)realloc(
                            //    logbuf, sizeof(char) * (MAX_LENGTH));
                            lb = logbuf = (char*)malloc(MAX_LENGTH);
                        }

                        memset( lb = logbuf, '\0', MAX_LENGTH );
                    }
                }
            }

            if ( 0x0a==c ) {
                //excflag = false;
                //transmission(STDOUT_FILENO, comm, sprintf(comm, "%s", RESET));
                transmission(STDOUT_FILENO, RESET, strlen(RESET));
            }

            //if ( 0x21==c && cflag && !excflag ) {
            //    comlen = 0;
            //    excflag = true;
            //    transmission(STDOUT_FILENO, comm, sprintf(comm, "\b%s%c", COLOR_COMMENT, c));
            //}

            //if ( excflag && 0x07!=c )
            //    comlen++;

            if ( 0x07==c ) {
                bsflag = false;
            }

            //if ( 0x08==c ) {
            //    if ( excflag )
            //        comlen-=2;
            //    if ( excflag && 0>=comlen ) {
            //        transmission(STDOUT_FILENO, comm, sprintf(comm, "%s", RESET));
            //        //memset( io = s, '\0', MAX_LENGTH );
            //        io++;
            //        excflag = false;
            //    }
            //}

            //if ( !excflag && cflag )
            if ( cflag ) {
                coloring(c);
            }
        }

        // if new data is available on the console, send it to the serial port
        if (read(STDIN_FILENO, &c, 1) > 0) {
            if ( 0x1b==c )                 escflag = true; // ^
            else if ( 0x5b==c && escflag ) spflag  = true; // ^[
            else if ( 0x33==c && spflag )  tilflag = true; // ^[3
            else if ( 0x7e==c && tilflag ) {               // ^[3~
                c = 0x7f;
                escflag = spflag = tilflag = false;
            }
            else {
                escflag = spflag = false;
            }

            if ( endcode == c )                   break; // hang up
            if ( 0x00 == c )                  c = 0x7f;  // BS on Vimterminal

            if ( 0x7f == c )             bsflag = 3;     // BS on Vimterminal

            transmission(fd, &c, 1);
        }

        /* 100 microsecond */
        usleep(100);
    }

    close(fd);

    if (logflag) {
        if ( ts ) {
            fwritets(lf);
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

bool ishex(char c) {
    return ('a' <= c && c <= 'f') ||
           ('A' <= c && c <= 'F') ||
           ('0' <= c && c <= '9');
}

int numlen(int num) {
    int cnt = 1;
    while (num /= 10) {
        ++cnt;
    }
    return cnt;
}

bool hosttoip(char *dstaddr, char *optarg) {
    char addr[15+1];
    char *arg = malloc(strlen(optarg)+1);
    struct hostent *host = NULL;
    sscanf(optarg, "%[^ :\n]", arg);
    arg[strlen(arg)] = '\0';
    host = gethostbyname(arg);  // getaddrinfo()を検討
    if (host == NULL) {
        return false;
    }
    if (host->h_length != 4) {
        sisterr("%serror:%s Only IPv4 supported\n", E_RED, RESET);
        return false;
    }
    sprintf(addr, "%d.%d.%d.%d",
        (unsigned char)*(host->h_addr_list[0] + 0),
        (unsigned char)*(host->h_addr_list[0] + 1),
        (unsigned char)*(host->h_addr_list[0] + 2),
        (unsigned char)*(host->h_addr_list[0] + 3));
    pack_space_cpy(dstaddr, addr);
    free(arg);
    return true;
}

void pack_space_cpy(char *dstaddr, const char *addr) {
    int i = 0;
    const char *p = addr;
    while (*p) {
        if (!isspace(*p)) {
            dstaddr[i++] = *p;
        }
        ++p;
    }
    dstaddr[i] = '\0';
}

bool correct_ipaddr_format(const char *addr) {
    // XXX.XXX.XXX.XXX:XXXXX || XXX.XXX.XXX.XXX
    enum { OUT_OF_RANGE = -2 };
    regex_t preg;
    const char *format = "^ *(2[0-4][0-9]|1[0-9]{2}|[1-9][0-9]|[1-8])[.]((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9]) *(: *[1-9][0-9]{4}|: *[1-9][0-9]{3}|: *[1-9][0-9]{2}|: *[1-9][0-9]{1}|: *[0-9]|) *$";
    int rc = regcomp(&preg, format, REG_NOSUB | REG_EXTENDED | REG_NEWLINE);
    if (rc != 0) {
        char msg[100];
        regerror(rc, &preg, msg, 100);
        regfree(&preg);
        sisterr("%serror:%s Failed regcomp(): %s\"%S\"%s\n", E_RED, RESET, E_YELLOW, msg, RESET);
        return false;
    }
    rc = regexec(&preg, addr, 0, 0, 0);
    regfree(&preg);

    if (rc == 0) {
        if (pull_port_num(addr) == OUT_OF_RANGE) {
            return false;
        }
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

    if (sscanf(addr, "%*[^:]:%d", &port) != 1) {
        return NONE_PORT;
    }
    if (port > MAX_PORT_NUM) {
        return OUT_OF_RANGE;
    }
    return port;
}

void transmission(int _fd, const void* _buf, size_t _len) {
    if ( -1 == write(_fd, _buf, _len) ) {
        sisterr("%serror:%s Failed write()\n", E_RED, RESET);
        //exit(EXIT_FAILURE);
    }
}


void fwritets(FILE *lf) {
    static struct timespec now;
    static struct tm       tm;
    static char date[82];
    clock_gettime(CLOCK, &now);
    localtime_r(&now.tv_sec, &tm);
    sprintf(date, "[%d-%02d-%02d %02d:%02d:%02d.%03d] ",
        tm.tm_year+1900, tm.tm_mon+1, tm.tm_mday,
        tm.tm_hour, tm.tm_min, tm.tm_sec,
        (int) now.tv_nsec / 1000000);
    fwrite(date, 1, strlen(date), lf);
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
    while ((p = strstr(str, before)) != NULL) {
        *p = '\0';
        p += (int)strlen(before);
        strcat(str, after);
        strcat(str, p);
    }
}

int syntaxCheck(char *str) {
    int hi_num;
    for (hi_num = 0; hi_num < params_len; ++hi_num) {
        if ( regexec(&params[hi_num].regex, str, 0, 0, 0) == 0 ) {
            return hi_num;
        }
    }
    return -1;
}

void repaint(const char *color) {
    io = s;
    int  i  = 0;
    char bs = 0x08;
    char tmp[MAX_LENGTH];
    char str[MAX_LENGTH + 32];
    while (*io) {
        tmp[i++] = *io++;
        transmission(STDOUT_FILENO, &bs, 1);
    }
    if (tmp[i]!='\0') {
        tmp[i]='\0';
    }
    transmission(STDOUT_FILENO, str, sprintf(str, "%s%s%s", color, tmp, RESET));
}

void coloring(char c) {
    if ( (/*0x08!=c &&*/ 0x21>c && !bsflag) ) {
        memset( io = s, '\0', MAX_LENGTH );
        return;
    }

    //en route
    if ( bsflag ) {
        if ( 2==bsflag-- /*&& '\0'!=s[0]*/) {
            *io++ = '\0';
            //*io++ = 0x20;
        } else {
            io--;
        }

        if ( bsflag ) {
            return;
        }
    }
    /*else if ( 0x08==c ) {
        io--;
        if ( '\0'==s )
            memset( io = s, '\0', MAX_LENGTH );
        return;
    } //*/
    else if ( strlen(s) < MAX_LENGTH - 1 ) {
        *io++ = c;
    }
    else {
        memset( io = s, '\0', MAX_LENGTH );
        return;
    }

    int checked = syntaxCheck(s);
    if (checked >= 0) {
        repaint(params[checked].color);
    }

    //        case HL_SLASH:
    //            sprintf(s, "/");
    //            repaint(COLOR_SLASH);
    //            memset( io = s, '\0', sizeof(s) );
    //            break;

}

void setSignal(int p_signame) {
    if ( signal(p_signame, sigcatch) == SIG_ERR ) {
        sisterr("%serror:%s SIG_ERR\n", E_RED, RESET);
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

