PREFIX  ?= /usr/local/bin
PROGRAM := sist
CFLAGS  ?= -Wall -Wextra -Wwrite-strings -pedantic -Ofast -march=native -s

$(PROGRAM): sisterm.c
	@gcc $(CFLAGS) -o $(PROGRAM) sisterm.c

install: $(PROGRAM)
	@install -s $(PROGRAM) $(PREFIX)/

uninstall:
	@rm -f $(PREFIX)/$(PROGRAM)
