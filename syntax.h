
#define RESET                 "\e[0m"
#define UNDERLINE             "\e[4m"
#define DEFAULT_F             "\e[39m"
#define DEFAULT_B             "\e[49m"

// Use 8-bit Colours format (bat only 4-bit used)
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
#define HL_SPACE            14


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

//ciscoprotocol = ipv4 ipv6 tcp udp icmp echo
//ciscoprotocol = http https www dhcp domain nameserver ssh telnet ftp ftp-data
//ciscoprotocol = ntp snmp snmptrap syslog
//ciscoprotocol = smtp pop2 pop3
//ciscoprotocol = klogin kshell login rlogin sunrpc
//ciscoprotocol = mpls rip isis ospf ospfv3 eigrp bgp hsrp vrrp ipsla
//ciscoprotocol = isdn dial hdlc frame-relay atm
//ciscoprotocol = igmp multicast broadcast
//ciscoprotocol = rsa pki isakmp ipsec ike esp gre vpn mvpn pppoe
//ciscoprotocol = qos cef pim ahp tacacs
//ciscoprotocol = cdp lldp vtp spanning-tree lacp dot1q l2tun ethernet
//ciscoprotocol = aaa aaa-server
//ciscoprotocol = /traps\?/

//ciscoconfigure = activate set default redundancy prefe ron tag
//ciscoconfigure = inside outside input output static export import

//ciscofunction = service crypto encapsulation standby mode in out
//ciscofunction = storm-control snmp-server group nat banner mask seq metric
//ciscofunction = add-route shape rd route-target as-path remote-as
//ciscofunction = access-list access-class access-group prefix-list
//ciscofunction = passive-interface distribute-list permit subnet-zero
//ciscofunction = /channel\-\(group\|protocol\)/

//ciscocomment = /!.*$/
//ciscocomment = /no\s.*$/
//ciscocomment = /description.*$/
//ciscocomment = /remark.*$/
//ciscocomment = /\s*#.*$/

//ciscostring = /\"[^"]*\"/

//ciscointerface = /^\(interface\|vlan\|line\|router\|track\)\s.*\d$/
//ciscointerface = /^ip\s\(sla\|vrf\)\s.*\d$/
//ciscointerface = /^monitor\ssession\s\d\+$/
//ciscointerface = /^\(class\|policy\|route\)\-map\s.*$/
//ciscointerface = /^ip\saccess\-list\s\(standard\|extended\)\s.*$/
//ciscointerface = /^vrf\s\(definition\|context\)\s.*$/
//ciscointerface = /^address\-family\sipv.*$/

//ciscoaction = disable deny shutdown down none

//ciscovar = trunk access full full-duplex auto active monitor
//ciscovar = any enable disable
//ciscovar = pvst mst rapid-pvst \transparent server client
//ciscovar = /\d\+[mg]\?/

//ciscoipv4 = /\(25[0-5]\|2[0-4]\d\|[01]\?\d\{1,2}\)\(\.\(25[0-5]\|2[0-4]\d\|[01]\?\d\{1,2}\)\)\{3}\(\/[0-9]\{1,2\}\)\?/
