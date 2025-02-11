#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include "curse_adapter.h"
#include "curse_term.h"

WINDOW * restrict edit_window = NULL;
WINDOW * restrict status_window = NULL;
bool screen_changed = false;

_Bool screen_init(void) {
    if (initscr() == NULL) {
        return false;
    }
    
    create_windows();
    
    noecho();
    raw();
    keypad(stdscr, true);
    
    return true;
}

void screen_end(void) {
    if (status_window != NULL) {
        delwin(status_window);
        status_window = NULL;
    }
    if (edit_window != NULL) {
        delwin(edit_window);
        edit_window = NULL; 
    }
    endwin();
}

void create_windows(void) {
    // Create main editing window (everything except last line)
    edit_window = newwin(LINES - 1, COLS, 0, 0);
    
    // Create status line window (last line)
    status_window = newwin(1, COLS, LINES - 1, 0);
    
    if (edit_window != NULL) {
        keypad(edit_window, true);
        wrefresh(edit_window);
    }
    
    if (status_window != NULL) {
        keypad(status_window, true);
        wrefresh(status_window);
    }
}

void refresh_screen(void) {
    if (!screen_changed) {
        return;
    }
    
    if (edit_window != NULL) {
        wrefresh(edit_window);
    }
    if (status_window != NULL) {
        wrefresh(status_window);
    }
    screen_changed = false;
}

void clear_screen(void) {
    if (edit_window != NULL) {
        werase(edit_window);
    }
    if (status_window != NULL) {
        werase(status_window);
    }
    screen_changed = true;
}

void clear_to_eol(void) {
    if (edit_window != NULL) {
        wclrtoeol(edit_window);
    }
    screen_changed = true;
}

void move_cursor(int row, int col) {
    if (edit_window != NULL) {
        wmove(edit_window, row, col);
    }
    screen_changed = true;
}

void get_cursor_pos(int *row, int *col) {
    if (edit_window != NULL) {
        if (row) *row = edit_window->_cury;
        if (col) *col = edit_window->_curx;
    }
}

void put_char(int ch) {
    if (edit_window != NULL) {
        waddch(edit_window, ch);
    }
    screen_changed = true;
}

void put_string(const char *str) {
    if (edit_window != NULL) {
        waddstr(edit_window, (char *)str);
    }
    screen_changed = true;
}

void set_attribute(int attr) {
    if (edit_window != NULL) {
        wattron(edit_window, attr);
    }
}

void clear_attribute(int attr) {
    if (edit_window != NULL) {
        wattroff(edit_window, attr);
    }
}

int get_key(void) {
    if (edit_window == NULL) {
        return -1;
    }
    int c = wgetch(edit_window);
    if (c == ERR) return -1;
    return c;
}

void flush_input(void) {
    flushinp();
}
