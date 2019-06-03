#define RESET             "\033[0m"
#define STEELBLUE         "\033[38;5;067m"
#define COLOR_COMMENT      STEELBLUE

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
