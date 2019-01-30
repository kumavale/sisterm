DESTDIR =
PREFIX  = /usr/local
PROGRAM = sisterm

all :
	$(PROGRAM)

$(PROGRAM) : sisterm.c
	gcc -o sisterm main.c

clean :
	rm -f $(PROGRAM)

install : $(PROGRAM)
	install -s $(PROGRAM) $(PREFIX)/bin
