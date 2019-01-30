DESTDIR =
PREFIX  = /usr/local
PROGRAM = sisterm

all :
	$(PROGRAM)

$(PROGRAM) : main.c
	gcc -o sisterm main.c

clean :
	rm -f $(PROGRAM)

install : $(PROGRAM)
	install -s $(PROGRAM) $(PREFIX)/bin
