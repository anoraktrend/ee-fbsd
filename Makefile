DEFINES =	-DSYS5  -DBSD_SELECT  -DCURSES -DNCURSE -DHAS_UNISTD  -DHAS_STDLIB -DHAS_CTYPE -DHAS_SYS_IOCTL -DHAS_SYS_WAIT   -DSLCT_HDR -s 



all :	curses

curses :	ee.c
	cc ee.c -o ee $(CFLAGS) -lncursesw 

ee :	ee.o new_curse.o
	cc -o ee ee.o new_curse.o $(CFLAGS) 

ee.o :	ee.c 
	cc -c ee.c $(DEFINES) $(CFLAGS) 

new_curse.o :	new_curse.c new_curse.h
	cc new_curse.c -c $(DEFINES) $(CFLAGS)

