DESTDIR =
PREFIX  = /usr/local
PROGRAM = sist

$(PROGRAM): sisterm.c
	@gcc -Ofast -march=native -s -o sist sisterm.c

install: $(PROGRAM)
	@install -s $(PROGRAM) $(PREFIX)/bin

uninstall:
	@rm -f $(PREFIX)/bin/sist
