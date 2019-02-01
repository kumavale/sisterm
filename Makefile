DESTDIR =
PREFIX  = /usr/local
PROGRAM = sisterm

$(PROGRAM) : sisterm.c
	@gcc -o sisterm sisterm.c

clean :
	rm -f $(PROGRAM)

install : $(PROGRAM)
	install -s $(PROGRAM) $(PREFIX)/bin
