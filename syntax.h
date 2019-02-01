
#define RESET                 "\e[0m"
#define BLACK                 "\e[38;5;000m"
#define MAROON                "\e[38;5;001m"
#define GREEN                 "\e[38;5;002m"
#define OLIVE                 "\e[38;5;003m"
#define NAVY                  "\e[38;5;004m"
#define PURPLE                "\e[38;5;005m"
#define TEAL                  "\e[38;5;006m"
#define SILVER                "\e[38;5;007m"
#define GREY                  "\e[38;5;008m"
#define RED                   "\e[38;5;009m"
#define LIME                  "\e[38;5;010m"
#define YELLOW                "\e[38;5;011m"
#define BLUE                  "\e[38;5;012m"
#define FUCHSIA               "\e[38;5;013m"
#define AQUA                  "\e[38;5;014m"
#define WHITE                 "\e[38;5;015m"
#define GREY0                 "\e[38;5;016m"
#define NAVYBLUE              "\e[38;5;017m"
#define DARKBLUE              "\e[38;5;018m"
#define BLUE3_1               "\e[38;5;019m"
#define BLUE3_2               "\e[38;5;020m"
#define BLUE1                 "\e[38;5;021m"
#define DARKGREEN             "\e[38;5;022m"
#define DEEPSKYBLUE4_1        "\e[38;5;023m"
#define DEEPSKYBLUE4_2        "\e[38;5;024m"
#define DEEPSKYBLUE4_3        "\e[38;5;025m"
#define DODGERBLUE3           "\e[38;5;026m"
#define DODGERBLUE2           "\e[38;5;027m"
#define GREEN4                "\e[38;5;028m"
#define SPRINGGREEN4          "\e[38;5;029m"
#define TURQUISE4             "\e[38;5;030m"
#define DEEPSKYBLUE3_1        "\e[38;5;031m"
#define DEEPSKYBLUE3_2        "\e[38;5;032m"
#define DODGERBLUE1           "\e[38;5;033m"
#define GREEN3_1              "\e[38;5;034m"
#define SPRINGGREEN3_1        "\e[38;5;035m"
#define DARKCYAN              "\e[38;5;036m"
#define LIGHTSEAGREEN         "\e[38;5;037m"
#define DEEPSKYBLUE2          "\e[38;5;038m"
#define DEEPSKYBLUE1          "\e[38;5;039m"
#define GREEN3_2              "\e[38;5;040m"
#define SPRINGGREEN3_2        "\e[38;5;041m"
#define SPRINGGREEN2_1        "\e[38;5;042m"
#define CYAN3                 "\e[38;5;043m"
#define DARKTURQUOISE         "\e[38;5;044m"
#define TURQUISE2             "\e[38;5;045m"
#define GREEN1                "\e[38;5;046m"
#define SPRINGGREEN2_2        "\e[38;5;047m"
#define SPRINGGREEN1          "\e[38;5;048m"
#define MEDIUMSPRINGGREEN     "\e[38;5;049m"
#define CYAN2                 "\e[38;5;050m"
#define CYAN1                 "\e[38;5;051m"
#define DARKRED               "\e[38;5;052m"
#define DEEPPINK4             "\e[38;5;053m"
#define PURPLE4_1             "\e[38;5;054m"
#define PURPLE4_2             "\e[38;5;055m"
#define PURPLE3               "\e[38;5;056m"
#define BLUEVIOLET            "\e[38;5;057m"
#define ORANGE4               "\e[38;5;058m"
#define GREY37                "\e[38;5;059m"

// ToDo change
#define BOLDBLACK     "\e[1m\e[30m"      // Bold Black
#define BOLDRED       "\e[1m\e[31m"      // Bold Red
#define BOLDGREEN     "\e[1m\e[32m"      // Bold Green
#define BOLDYELLOW    "\e[1m\e[33m"      // Bold Yellow
#define BOLDBLUE      "\e[1m\e[34m"      // Bold Blue
#define BOLDMAGENTA   "\e[1m\e[35m"      // Bold Magenta
#define BOLDCYAN      "\e[1m\e[36m"      // Bold Cyan
#define BOLDWHITE     "\e[1m\e[37m"      // Bold White


#define HL_CISCO            1
#define HL_COND             2
#define HL_KEYWORD          3
#define HL_PROTOCOL         4
#define HL_CONFIGURE        5
#define HL_FUNCTION         6
#define HL_COMMENT          7
#define HL_STRING           8
#define HL_INTERFACE        9
#define HL_ACTION           10
#define HL_VAR              11
#define HL_IPV4             12
#define HL_IPV6             13


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

//CISCO   = cisco
//COND    = match|eq|neq|gt|lt|ge|le|range
//KEYWORD = speed|duplex|negotiation|delay|bandwidth|preempt|priority|timers
//KEYWORD = logging|log|login|password|username|user|license|host|hostname|system
//KEYWORD = address|network|route|neighbor|redistribute|default-gateway|community
//KEYWORD = version|class|switchport|clock|name|minimum|maximum|level|size
//KEYWORD = established|source|destination|allowed
//KEYWORD = timeout|threshold|frequency|keepalive|average|weights|mtu|tunnel
//KEYWORD = privilege|secret
