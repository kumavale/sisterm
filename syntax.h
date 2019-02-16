
#define RESET             "\e[0m"
#define UNDERLINE         "\e[4m"
#define DEFAULT_F         "\e[39m"
#define DEFAULT_B         "\e[49m"

#define BLACK             "\e[38;5;000m"
#define MAROON            "\e[38;5;001m"
#define GREEN             "\e[38;5;002m"
//#define GREEN             "\e[38;2;0;176;107m"
#define OLIVE             "\e[38;5;003m"
#define NAVY              "\e[38;5;004m"
#define PURPLE            "\e[38;5;005m"
//#define PURPLE            "\e[38;2;153;0;153m"
#define TEAL              "\e[38;5;006m"
#define SILVER            "\e[38;5;007m"
#define GREY              "\e[38;5;008m"
#define RED               "\e[38;5;009m"
//#define RED               "\e[38;2;255;75;0m"
#define LIME              "\e[38;5;010m"
#define YELLOW            "\e[38;5;011m"
//#define YELLOW            "\e[38;2;242;170;0m"
#define BLUE              "\e[38;5;012m"
//#define BLUE              "\e[38;2;25;113;255m"
#define FUCHSIA           "\e[38;5;013m"
#define AQUA              "\e[38;5;014m"
#define WHITE             "\e[38;5;015m"
#define SPRINGGREEN       "\e[38;5;048m"
#define STEELBLUE         "\e[38;5;067m"
#define CORNFLOWERBLUE    "\e[38;5;069m"
#define YELLOW3           "\e[38;5;148m"
#define MEDIUMORCHID      "\e[38;5;207m"
#define ORANGE            "\e[38;5;214m"
#define DEEPPINK          "\e[38;5;197m"
#define MIDIUMPURPLE1     "\e[38;5;141m"
#define STEELBLUE1        "\e[38;5;81m"
#define DARKORANGE        "\e[38;5;208m"
#define CORNSILK1         "\e[38;5;230m"


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


#define COND "^(match|eq|neq|gt|lt|ge|le|range)$"

#define KEYWORD "^(giants|speed|duplex|negotiation|delay|bandwidth|preempt|priority|timers|username|user|license|host|system|systems|address|network|route|routers|neighbor|neighbors|redistribute|default[-]gateway|community|version|class|name|minimum|maximum|level|size|established|source|destination|allowed|timeout|threshold|frequency|keepalive|average|weights|mtu|tunnel|privilege|secret)$"

#define PROTOCOL "^(ipv4|ipv6|tcp|udp|icmp|echo|dhcp|domain|nameserver|ssh|telnet|ntp|snmp|snmptrap|syslog|smtp|pop2|pop3|klogin|kshell|rlogin|sunrpc|mpls|rip|isis|ospf|ospfv3|eigrp|bgp|hsrp|vrrp|ipsla|isdn|dial|dialog|hdlc|frame[-]relay|atm|igmp|multicast|broadcast|rsa|pki|isakmp|ipsec|ike|esp|gre|vpn|mvpn|pppoe|qos|cef|pim|ahp|tacacs|cdp|lldp|vtp|spanning[-]tree|lacp|dot1q|l2tun|ethernet|aaa|aaa[-]server)$"

//#define CONFIGURE  activate set default redundancy prefe ron tag
//inside outside input output static export import

//#define FUNCTION  service crypto encapsulation standby mode in out
// storm-control snmp-server group nat banner mask seq metric
// add-route shape rd route-target as-path remote-as
// access-list access-class access-group prefix-list
// passive-interface distribute-list permit subnet-zero
// /channel\-\(group\|protocol\)/

#define CONFIRM  "^(y|yes)$"

//#define COMMENT  "^(!.*)$"
// description.*$
// remark.*$
// *#.*$

// CISCO
#define COMMAND "^(exit|end|configure|interface|show|line|copy|username|hostname|password|login|service|ip|crypt|transport|clock|ntp|logging|snmp[-]server|vtp|vlan|name|switch|switchport|router|channel[-]group|port[-]channel|spanning[-]tree|instance|revision|mac|storm[-]control|cdp|lldp|version|offset[-]list|auto[-]summary|auto[-]cost|area|summary[-]address|distribute[-]list|redistribute|default[-]information|passive[-]interface|vrrp|standby|access[-](list|class)|reload|monitor|mls)$"

#define POSITIVE "^([[]?up[]]?|enable|enabled|active)$"
//#define NEGATIVE "^(down|disable|disabled|no|not|invalid)$"

#define STRING "^(\".*\"|\'.*\')$"

//#define URL "^(((([A-Za-z]{3,9}:(?:[/][/])?)(?:[-;:&=[+][ ],[0-9A-Fa-f]]+@)?[A-Za-z0-9.-]+|(?:www.|[-;:&=[+][ ],[0-9A-Fa-f_]]+@)[A-Za-z0-9.-]+)((?:[/][[+~\%[/].[0-9A-Fa-f_]-_]*)?[?]?(?:[-[+]=&;%@.[0-9A-Fa-f_]_]*)#?(?:[0-9A-Fa-f]*))?))$"

#define EMPHASIS "^(no|not|[[]?confirm[]]?|warning|warnings|failed|failures|error|errors|crash)$"

//#define INTERFACE "^((Tengigabit|Gigabit|Fast|)Ethernet[0-9]/[0-9]+|(Fa|Gi)[0-9]/[0-9]?[0-9])$"
#define INTERFACE "^((Tengigabit|Gigabit|Fast|)Ethernet|(Fa|Gi))$"

//#define NUMBER

#define ACTION  "^(disable|deny|shutdown|[[]?down[]]?|[[]?administratively[]]?|none)$"

#define VAR     "^(trunk|access|full[-]duplex|full|auto[-](duplex|speed)|auto|monitor|any|disable|pvst|mst|rapid[-]pvst|transparent|server|client)$"

#define VENDORS "^(cisco|aruba|juniper|huawei|arista|riverbed|netscout|yamaha|mellanox)$"

#define IPV4_NET  "^(2[0-4][0-9]|1[0-9]{2}|[1-9][0-9]|[1-8])[.]((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$"
#define IPV4_SUB  "^((25[0-5]|24[89])[.])((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$"
#define IPV4_WILD "^(0[.])((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])[.]){2}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$"

#define IPV6      "^((([0-9A-Fa-f]{1,4}:){7}(:|([0-9A-Fa-f]{1,4})))|(([0-9A-Fa-f]{1,4}:){6}(:|((25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})([.](25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})){3})|(:[0-9A-Fa-f]{1,4})))|(([0-9A-Fa-f]{1,4}:){5}((:((25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})([.](25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})){3})?)|((:[0-9A-Fa-f]{1,4}){1,2})))|(([0-9A-Fa-f]{1,4}:){4}(:[0-9A-Fa-f]{1,4}){0,1}(:((25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})([.](25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})){3})?))|(([0-9A-Fa-f]{1,4}:){4}(:[0-9A-Fa-f]{1,4}){0,1}((:[0-9A-Fa-f]{1,4}){1,2}))|(([0-9A-Fa-f]{1,4}:){3}(:[0-9A-Fa-f]{1,4}){0,2}(:((25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})([.](25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})){3})?))|(([0-9A-Fa-f]{1,4}:){3}(:[0-9A-Fa-f]{1,4}){0,2}((:[0-9A-Fa-f]{1,4}){1,2}))|(([0-9A-Fa-f]{1,4}:){2}(:[0-9A-Fa-f]{1,4}){0,3}(:((25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})([.](25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})){3})?))|(([0-9A-Fa-f]{1,4}:){2}(:[0-9A-Fa-f]{1,4}){0,3}((:[0-9A-Fa-f]{1,4}){1,2}))|(([0-9A-Fa-f]{1,4}:)(:[0-9A-Fa-f]{1,4}){0,4}(:((25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})([.](25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})){3})?))|(([0-9A-Fa-f]{1,4}:)(:[0-9A-Fa-f]{1,4}){0,4}((:[0-9A-Fa-f]{1,4}){1,2}))|(:(:[0-9A-Fa-f]{1,4}){0,5}((:((25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})([.](25[0-5]|2[0-4][0-9]|[01]?[0-9]{1,2})){3})?)|((:[0-9A-Fa-f]{1,4}){1,2}))))$"
