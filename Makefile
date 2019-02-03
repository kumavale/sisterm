DESTDIR =
PREFIX  = /usr/local
PROGRAM = sist

$(PROGRAM): sisterm.c
	@gcc -o sist sisterm.c

install: $(PROGRAM)
	@install -s $(PROGRAM) $(PREFIX)/bin

uninstall:
	@rm -f $(PREFIX)/bin/sist
