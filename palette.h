// palette.h:2 削除
#define COLOR_COMMENT      STEELBLUE

typedef struct {
    const char *key;
    const char *val;
} map;

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
