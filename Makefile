CC?=		gcc
CFLAGS+=	-O2 -Wall 
PREFIX?=	/usr/local
BINDIR=		${PREFIX}/bin
MANDIR=		${PREFIX}/share/man/man1

# Check for ncurses
LIBS=		-lncurses
CFLAGS+=	-DHAS_NCURSES -DHAS_UNISTD -DHAS_STDARG -DHAS_STDLIB

# Source files
SRCS=		ee.c
OBJS=		${SRCS:.c=.o}
PROG=		ee

# Installation mode
BINMODE=	755
MANMODE=	644

# Main targets
all: ${PROG}

${PROG}: ${OBJS}
	${CC} ${CFLAGS} -o ${PROG} ${OBJS} ${LIBS}

install: ${PROG}
	install -d ${DESTDIR}${BINDIR}
	install -m ${BINMODE} ${PROG} ${DESTDIR}${BINDIR}
	ln -sf ${PROG} ${DESTDIR}${BINDIR}/ree
	install -d ${DESTDIR}${MANDIR}
	install -m ${MANMODE} ee.1 ${DESTDIR}${MANDIR}

clean:
	rm -f ${PROG} ${OBJS} core *.core

distclean: clean
	rm -f .depend

depend: ${SRCS}
	mkdep ${CFLAGS} ${SRCS}

lint:
	lint -hx ${SRCS}

tags:
	ctags ${SRCS}

.PHONY: all install clean distclean depend lint tags

# Default make rules
.c.o:
	${CC} ${CFLAGS} -c $<

