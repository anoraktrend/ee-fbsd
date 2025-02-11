#ifndef EE_CURSE_TERM_H
#define EE_CURSE_TERM_H

#include <stdbool.h>
#include <termios.h>
#include "config.h"

/* Terminal capabilities */
extern char * restrict tgetstr(const char * restrict id, char ** restrict area);
extern int tgetent(char * restrict bp, const char * restrict name);
extern int tgetnum(const char * restrict id);
extern int tgetflag(const char * restrict id);
extern int tputs(const char * restrict str, int affcnt, int (*putc)(int));

/* Terminal state */
extern struct termios orig_termios;           /* Original terminal state */
extern char term_buffer[2048];                /* Termcap buffer */
extern char * restrict term_strings;          /* String table pointer */
extern bool cursor_vis;                       /* Cursor visibility state */

/* Capability strings */
extern const char * restrict cursor_address;  /* Cursor positioning */
extern const char * restrict clear_screen;    /* Clear entire screen */
extern const char * restrict clr_eol;         /* Clear to end of line */
extern const char * restrict enter_standout;  /* Begin standout mode */
extern const char * restrict exit_standout;   /* End standout mode */

/* Function declarations */
int term_init(void);
void term_reset(void);
int term_setup(void);
int outputc(int c);

#endif /* EE_CURSE_TERM_H */
