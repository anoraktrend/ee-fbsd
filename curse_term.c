#include "curse_term.h"
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <curses.h>
#include <term.h>

struct termios orig_termios;
char term_buffer[2048];
char *term_strings;

char *cursor_address;
char *clear_screen;
char *clr_eol;
char *enter_standout;
char *exit_standout;
char *scroll_forward;
char *scroll_reverse;
char *insert_line;
char *delete_line;
char *keypad_xmit;
char *keypad_local;
char *init_1string;
char *exit_ca_mode;
char *enter_ca_mode;

int phys_x = 0;
int phys_y = 0;
int cursor_vis = 1;
int term_modes = 0;

void move_phys_cursor(int y, int x) {
    if (cursor_address) {
        tputs(tgoto(cursor_address, x, y), 1, outputc);
        phys_x = x;
        phys_y = y;
    }
}

void save_cursor(void) {
    if (cursor_invisible)
        tputs(cursor_invisible, 1, outputc);
    cursor_vis = 0;
}

void restore_cursor(void) {
    if (cursor_visible)
        tputs(cursor_visible, 1, outputc);
    cursor_vis = 1;
}

int term_init(void) {
    char *termtype = getenv("TERM");
    if (!termtype) termtype = "vt100";
    
    // Load terminal capabilities
    if (tgetent(term_buffer, termtype) != 1) {
        return -1;
    }
    
    // Get terminal size
    LINES = tgetnum("li");
    COLS = tgetnum("co");
    if (LINES <= 0) LINES = 24;
    if (COLS <= 0) COLS = 80;
    
    // Get capability strings
    term_strings = term_buffer;
    cursor_address = tgetstr("cm", &term_strings);
    clear_screen = tgetstr("cl", &term_strings);
    clr_eol = tgetstr("ce", &term_strings);
    enter_standout = tgetstr("so", &term_strings);
    exit_standout = tgetstr("se", &term_strings);
    
    // Get cursor capability strings
    cursor_invisible = tgetstr("vi", &term_strings);
    cursor_visible = tgetstr("ve", &term_strings);
    cursor_normal = tgetstr("vs", &term_strings);
    
    // Get additional capability strings
    scroll_forward = tgetstr("sf", &term_strings);
    scroll_reverse = tgetstr("sr", &term_strings);
    insert_line = tgetstr("al", &term_strings);
    delete_line = tgetstr("dl", &term_strings);
    keypad_xmit = tgetstr("ks", &term_strings);
    keypad_local = tgetstr("ke", &term_strings);
    init_1string = tgetstr("is", &term_strings);
    exit_ca_mode = tgetstr("te", &term_strings);
    enter_ca_mode = tgetstr("ti", &term_strings);
    
    // Initialize terminal
    if (init_1string)
        tputs(init_1string, 1, outputc);
    
    // Enter cursor addressing mode
    term_enter_ca();
    return 0;
}

void term_enter_ca(void) {
    if (enter_ca_mode) {
        tputs(enter_ca_mode, 1, outputc);
        term_modes |= TERM_MODE_ALTSCR;
    }
}

void term_exit_ca(void) {
    if (exit_ca_mode) {
        tputs(exit_ca_mode, 1, outputc);
        term_modes &= ~TERM_MODE_ALTSCR;
    }
}

void term_scroll(int lines) {
    if (lines > 0 && scroll_forward) {
        while (lines--)
            tputs(scroll_forward, 1, outputc);
    } else if (lines < 0 && scroll_reverse) {
        while (lines++)
            tputs(scroll_reverse, 1, outputc);
    }
}

void term_setmode(int mode) {
    if ((mode & TERM_MODE_KEYPAD) && !(term_modes & TERM_MODE_KEYPAD)) {
        if (keypad_xmit)
            tputs(keypad_xmit, 1, outputc);
    }
    term_modes |= mode;
}

void term_clearmode(int mode) {
    if ((mode & TERM_MODE_KEYPAD) && (term_modes & TERM_MODE_KEYPAD)) {
        if (keypad_local)
            tputs(keypad_local, 1, outputc);
    }
    term_modes &= ~mode;
}

int term_setup(void) {
    struct termios new_termios;
    
    // Save original terminal state
    if (tcgetattr(STDIN_FILENO, &orig_termios) < 0)
        return -1;
        
    // Modify terminal settings
    new_termios = orig_termios;
    new_termios.c_lflag &= ~(ICANON | ECHO | ISIG);
    new_termios.c_iflag &= ~(IXON | ICRNL);
    new_termios.c_cc[VMIN] = 1;
    new_termios.c_cc[VTIME] = 0;
    
    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &new_termios) < 0)
        return -1;
        
    return 0;
}

void term_reset(void) {
    // Exit cursor addressing mode first
    term_exit_ca();
    
    // Restore terminal modes
    term_clearmode(TERM_MODE_KEYPAD);
    
    // Restore terminal settings
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios);
}

int outputc(int c) {
    return write(STDOUT_FILENO, &c, 1);
}
