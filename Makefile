DESTDIR =
PREFIX  = /usr/local
PROGRAM = sist

$(PROGRAM) : sisterm.c
	@gcc -o sist sisterm.c

clean :
	rm -f $(PROGRAM)

install : $(PROGRAM)
	install -s $(PROGRAM) $(PREFIX)/bin
