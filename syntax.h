
#define RESET         "\e[38;5;000m"     // Reset
#define BLACK         "\e[38;5;001m"     // Black
#define RED           "\e[38;5;002m"     // Red
#define GREEN         "\e[38;5;003m"     // Green
#define YELLOW        "\e[38;5;004m"     // Yellow
#define BLUE          "\e[38;5;005m"     // Blue
#define MAGENTA       "\e[38;5;006m"     // Magenta
#define CYAN          "\e[38;5;007m"     // Cyan
#define WHITE         "\e[38;5;008m"     // White

// ToDo change
#define BOLDBLACK     "\e[1m\e[30m"      // Bold Black
#define BOLDRED       "\e[1m\e[31m"      // Bold Red
#define BOLDGREEN     "\e[1m\e[32m"      // Bold Green
#define BOLDYELLOW    "\e[1m\e[33m"      // Bold Yellow
#define BOLDBLUE      "\e[1m\e[34m"      // Bold Blue
#define BOLDMAGENTA   "\e[1m\e[35m"      // Bold Magenta
#define BOLDCYAN      "\e[1m\e[36m"      // Bold Cyan
#define BOLDWHITE     "\e[1m\e[37m"      // Bold White


#define HL_COND       RED
#define HL_KEYWORD    RED
#define HL_PROTOCOL   RED
#define HL_CONFIGURE  RED
#define HL_FUNCTION   RED
#define HL_COMMENT    RED
#define HL_STRING     RED
#define HL_INTERFACE  RED
#define HL_ACTION     RED
#define HL_VAR        RED
#define HL_IPV4       RED
#define HL_IPV6       RED


//enum palette {
//  BLACK,
//  RED,
//  GREEN,
//  YELLOW,
//  BLUE,
//  MAGENTA,
//  CYAN,
//  WHITE,
//  COLOR_MAX
//};

//COND    = match|eq|neq|gt|lt|ge|le|range
//KEYWORD = speed|duplex|negotiation|delay|bandwidth|preempt|priority|timers
//KEYWORD = logging|log|login|password|username|user|license|host|hostname|system
//KEYWORD = address|network|route|neighbor|redistribute|default-gateway|community
//KEYWORD = version|class|switchport|clock|name|minimum|maximum|level|size
//KEYWORD = established|source|destination|allowed
//KEYWORD = timeout|threshold|frequency|keepalive|average|weights|mtu|tunnel
//KEYWORD = privilege|secret
