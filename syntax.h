
#define RESET             "\033[0m"
#define BOLD              "\033[1m"
#define UNDERLINE         "\033[4m"
#define DEFAULT_F         "\033[39m"
#define DEFAULT_B         "\033[49m"

#define BLACK             "\033[38;5;000m"
#define MAROON            "\033[38;5;001m"
#define GREEN             "\033[38;5;002m"
//#define GREEN             "\033[38;2;0;176;107m"
#define OLIVE             "\033[38;5;003m"
#define NAVY              "\033[38;5;004m"
#define PURPLE            "\033[38;5;005m"
//#define PURPLE            "\033[38;2;153;0;153m"
#define TEAL              "\033[38;5;006m"
#define SILVER            "\033[38;5;007m"
#define GREY              "\033[38;5;008m"
#define RED               "\033[38;5;009m"
//#define RED               "\033[38;2;255;75;0m"
#define LIME              "\033[38;5;010m"
#define YELLOW            "\033[38;5;011m"
//#define YELLOW            "\033[38;2;242;170;0m"
#define BLUE              "\033[38;5;012m"
//#define BLUE              "\033[38;2;25;113;255m"
#define FUCHSIA           "\033[38;5;013m"
#define AQUA              "\033[38;5;014m"
#define WHITE             "\033[38;5;015m"
#define SPRINGGREEN       "\033[38;5;048m"
#define STEELBLUE         "\033[38;5;067m"
#define CORNFLOWERBLUE    "\033[38;5;069m"
#define YELLOW3           "\033[38;5;148m"
#define MEDIUMORCHID      "\033[38;5;207m"
#define ORANGE            "\033[38;5;214m"
#define DEEPPINK          "\033[38;5;197m"
#define MIDIUMPURPLE1     "\033[38;5;141m"
#define STEELBLUE1        "\033[38;5;81m"
#define DARKORANGE        "\033[38;5;208m"
#define CORNSILK1         "\033[38;5;230m"


enum HiLight {
  HL_VENDORS,
  HL_COMMAND,
  HL_COND,
  HL_KEYWORD,
  HL_PROTOCOL,
  HL_CONFIGURE,
  HL_FUNCTION,
  HL_COMMENT,
  HL_STRING,
  HL_INTERFACE,
  HL_ACTION,
  HL_VAR,
  HL_IPV4_NET,
  HL_IPV4_SUB,
  HL_IPV4_WILD,
  HL_IPV6,
  HL_SPACE,
  HL_EMPHASIS,
  HL_POSITIVE,
  HL_URL,
  HL_SLASH,
  HL_MAX
};

/*
enum BackGround {
  DARK,
  LIGHT,
  NONE
}; //*/
