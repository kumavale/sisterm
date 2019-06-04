PREFIX  ?= /usr/local/bin
PROGRAM := sist
CFLAGS  ?= -Wall -Wextra -Wwrite-strings -pedantic -Ofast -march=native -s -lrt

$(PROGRAM): sisterm.c sisterm.h palette.h
	@gcc $(CFLAGS) -o $(PROGRAM) sisterm.c

install: $(PROGRAM)
	@install -s $(PROGRAM) $(PREFIX)/

uninstall:
	@rm -f $(PREFIX)/$(PROGRAM)
