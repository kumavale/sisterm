CC = gcc
RM ?= rm -f
PREFIX ?= /usr/local/bin
sist-CFLAGS = -pedantic
CFLAGS ?= -Wall -Wextra -Wwrite-strings -Ofast -march=native -s -MMD -MP
CFLAGS += $(sist-CFLAGS)
LDADD = -lrt
SRCDIR = ./src
OBJDIR = ./build
SOURCES := $(wildcard $(SRCDIR)/*.c)
INCLUDES := $(wildcard $(SRCDIR)/*.h)
OBJECTS = $(SOURCES:$(SRCDIR)/%.c=$(OBJDIR)/%.o)
TARGET = sist
DEPENDS = $(OBJECTS:.o=.d)

all: $(TARGET)

$(TARGET): $(OBJECTS)
	@$(CC) -o $@ $(OBJECTS)
	@echo "$(CC) -o $@ $(OBJECTS) $(LDADD) => OK"
	@echo "Compiled complete!"

$(OBJECTS): $(OBJDIR)/%.o : $(SRCDIR)/%.c
	@if [ ! -d $(OBJDIR) ]; \
		then mkdir -p $(OBJDIR) && echo "mkdir -p $(OBJDIR) => OK"; \
	fi
	@$(CC) $(CFLAGS) $(LDADD) -o $@ -c $<
	@echo "$(CC) $(CFLAGS) -o $@ -c $< => OK"

install: $(TARGET)
	@install -s $(TARGET) $(PREFIX)/
	@echo "install -s $(TARGET) $(PREFIX)/ => OK"

uninstall:
	@$(RM) $(PREFIX)/$(TARGET)
	@echo "$(RM) $(PREFIX)/$(TARGET) => OK"

clean:
	@$(RM) $(OBJECTS) $(TARGET) $(DEPENDS)
	@echo "$(RM) $(OBJECTS) $(TARGET) $(DEPENDS) => OK"
	@$(RM) -r $(OBJDIR)
	@echo "$(RM) -r $(OBJDIR) => OK"
	@echo "Cleanup complete!"

-include $(DEPENDS)

.PHONY: all clean
