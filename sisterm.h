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

#include <stdio.h>
#include <time.h>
#include <regex.h>
#include <sys/types.h>
#include <getopt.h>


// Signal action
void setSignal(int);
void sigcatch();

// Alternative to write()
void transmission(int, const void*, size_t);

// Keyboard Hit Check
int kbhit();

// All regcomp
int regcompAll();

// Check syntax
int syntaxCheck(char *str);

// Check define color length
int checkDefColorLen();

// Syntax OK -> repaint
void repaint(const char *color);

// Control coloring
void coloring(char c);

// Show version
// ( major.minor.tweak )
void version();

// Show usage
void usage(char *argv[]);
