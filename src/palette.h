#define RESET             "\033[0m"
#define ERROR_RED         "\033[38;5;196m"
#define ERROR_BLUE        "\033[38;5;021m"
#define ERROR_YELLOW      "\033[38;5;011m"

typedef struct {
    const char *key;
    const char *val;
} map;

enum ANSI_COLORS {
    AC_BLACK,
    AC_RED,
    AC_GREEN,
    AC_YELLOW,
    AC_BLUE,
    AC_MAGENTA,
    AC_CYAN,
    AC_WHITE,
    AC_MAX,
};

map ansi_colors[] = {
    { "BLACK",   "\033[30m" },
    { "RED",     "\033[31m" },
    { "GREEN",   "\033[32m" },
    { "YELLOW",  "\033[33m" },
    { "BLUE",    "\033[34m" },
    { "MAGENTA", "\033[35m" },
    { "CYAN",    "\033[36m" },
    { "WHITE",   "\033[37m" },
};
