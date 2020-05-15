//For serial
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>

//For telnet
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <signal.h>
#include <netdb.h>

#include <stdio.h>
#include <time.h>
#include <regex.h>
#include <sys/types.h>
#include <getopt.h>
#include <ctype.h>


typedef int bool;
#define true  1
#define false 0

// Signal action
void setSignal(int);
void sigcatch();

// Number of digits
int numlen(int num);


/**
 *  @brief     Check hex
 *
 *  @param     1 character
 *  @return
 */
bool ishex(char c);

// A string consisting of a single character repeat
char *loopc(const char c, int n);

// Change Hostname to IP address
bool hosttoip(char *dstaddr, char *optarg);

//
void pack_space_cpy(char *dstaddr, const char *addr);

//
bool correct_ipaddr_format(const char *addr);

// Pull IP address addr from dstaddr(en route)
void store_address(char *addr, const char *dstaddr);
//
int pull_port_num(const char *addr);

// Alternative to write()
void transmission(int, const void*, size_t);

// Append time stamp to logfile
void fwritets(FILE *lf);

// Keyboard Hit Check
int kbhit();

// Check syntax
int syntaxCheck(char *str);

// Replace before to after
void replace(char *str, const char *before, const char *after);

// Syntax OK -> repaint
void repaint(const char *color);

// Control coloring
void coloring(char c);

// Error end processing
void abort_exit(int fd, int when, const struct termios *termptr);

// Show version
// ( major.minor.tweak )
void version();

// Show usage
void usage(char *argv[]);
