#define COLOR_VENDORS      BOLD AQUA
#define COLOR_COMMAND      CORNSILK1
#define COLOR_COND         SILVER
#define COLOR_KEYWORD      MAROON
#define COLOR_PROTOCOL     OLIVE
#define COLOR_STRING       "\033[30m"
#define COLOR_INTERFACE    "\033[30m"
#define COLOR_ACTION       UNDERLINE FUCHSIA
#define COLOR_VAR          TEAL
#define COLOR_IPV4_NET     "\033[30m"
#define COLOR_IPV4_SUB     YELLOW3
#define COLOR_IPV4_WILD    LIME
#define COLOR_COMMENT      STEELBLUE
#define COLOR_IPV6         SPRINGGREEN
#define COLOR_EMPHASIS     ORANGE
#define COLOR_POSITIVE     DARKORANGE
#define COLOR_URL          UNDERLINE STEELBLUE1
#define COLOR_SLASH        DEEPPINK

//#define BLACK    "\033[30m"
//#define RED      "\033[31m"
//#define GREEN    "\033[32m"
//#define YELLOW   "\033[33m"
//#define BLUE     "\033[34m"
//#define MAGENTA  "\033[35m"
//#define CYAN     "\033[36m"
//#define WHITE    "\033[37m"

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
