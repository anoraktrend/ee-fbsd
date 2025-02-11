#include "curse_impl.h"
#include "common.h"
#include "new_curse.h"
#include "curse_types.h"
#include "curse_term.h"
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "error.h"

/* Window layering */
typedef struct winlist {
    WINDOW *win;
    int z_order;          // Lower numbers are on top
    struct winlist *next;
    struct winlist *prev;
} WINLIST;

static WINLIST *top_window = NULL;
static int next_z_order = 0;
static int current_z = 0;

WINDOW *curscr = NULL;
WINDOW *stdscr = NULL;
int LINES = 0;
int COLS = 0;

/* Box drawing characters */
#define ACS_VLINE    '|'
#define ACS_HLINE    '-'
#define ACS_ULCORNER '+'
#define ACS_URCORNER '+'
#define ACS_LLCORNER '+'
#define ACS_LRCORNER '+'

void register_window(WINDOW *win) {
    WINLIST *node = malloc(sizeof(WINLIST));
    if (!node) return;
    
    node->win = win;
    node->z_order = next_z_order++;
    node->next = top_window;
    node->prev = NULL;
    if (top_window)
        top_window->prev = node;
    top_window = node;
}

void set_window_order(WINDOW *win, int z) {
    WINLIST *node = top_window;
    while (node) {
        if (node->win == win) {
            node->z_order = z;
            current_z = z + 1;
            break;
        }
        node = node->next;
    }
}

int get_window_order(WINDOW *win) {
    WINLIST *node = top_window;
    while (node) {
        if (node->win == win)
            return node->z_order;
        node = node->next;
    }
    return -1;
}

void sort_windows(void) {
    if (!top_window || !top_window->next) return;
    
    // Simple bubble sort by z_order
    int swapped;
    do {
        swapped = 0;
        WINLIST *node = top_window;
        while (node->next) {
            if (node->z_order > node->next->z_order) {
                // Swap window pointers
                WINDOW *tmp_win = node->win;
                int tmp_z = node->z_order;
                node->win = node->next->win;
                node->z_order = node->next->z_order;
                node->next->win = tmp_win;
                node->next->z_order = tmp_z;
                swapped = 1;
            }
            node = node->next;
        }
    } while (swapped);
}

void unregister_window(WINDOW *win) {
    WINLIST *node = top_window;
    while (node) {
        if (node->win == win) {
            if (node->prev)
                node->prev->next = node->next;
            if (node->next)
                node->next->prev = node->prev;
            if (node == top_window)
                top_window = node->next;
            free(node);
            break;
        }
        node = node->next;
    }
}

void bring_to_top(WINDOW *win) {
    set_window_order(win, current_z++);
    sort_windows();
    touchwin(win);
}

NODISCARD
WINDOW * newwin(const int lines, const int cols, 
                const int start_l, const int start_c) {
    CHECK_RANGE(lines, 1, LINES);
    CHECK_RANGE(cols, 1, COLS); 
    CHECK_RANGE(start_l, 0, LINES - lines);
    CHECK_RANGE(start_c, 0, COLS - cols);

    WINDOW *win = calloc(1, sizeof(WINDOW));
    if (!win) {
        ee_set_error(EE_ERR_ALLOC_FAILED, "Failed to allocate window");
        return NULL;
    }

    if (win_init(win, lines, cols, start_l, start_c) != EE_SUCCESS) {
        free(win);
        return NULL;
    }

    register_window(win);
    return win;
}

static ee_error_t win_init(WINDOW * const NONNULL win,
                          const int lines, const int cols,
                          const int start_l, const int start_c) {
    CHECK_NULL(win);
    
    win->Num_lines = lines;
    win->Num_cols = cols;
    win->SR = start_l;
    win->SC = start_c;
    
    // Allocate line array with error handling
    win->line_array = calloc(lines, sizeof(struct _line *));
    if (!win->line_array) {
        ee_set_error(EE_ERR_ALLOC_FAILED, "Failed to allocate line array");
        return EE_ERR_ALLOC_FAILED;
    }
    
    // Initialize lines with error handling
    for (int i = 0; i < lines; i++) {
        win->line_array[i] = alloc_line(cols);
        if (!win->line_array[i]) {
            win_free_lines(win, i);
            ee_set_error(EE_ERR_ALLOC_FAILED, "Failed to allocate line");
            return EE_ERR_ALLOC_FAILED;
        }
    }
    
    return EE_SUCCESS;
}

static void win_free_lines(WINDOW * const NONNULL win, const int count) {
    assert(win != NULL);
    assert(win->line_array != NULL);
    
    for (int i = 0; i < count; i++) {
        if (win->line_array[i]) {
            free(win->line_array[i]->row);
            free(win->line_array[i]->attributes);
            free(win->line_array[i]);
        }
    }
    free(win->line_array);
}

void delwin(WINDOW *window) {
    if (!window) return;
    unregister_window(window);
    win_free_lines(window, window->Num_lines);
    free(window);
}

static struct _line *alloc_line(int cols) {
    struct _line *line = (struct _line *)malloc(sizeof(struct _line));
    if (!line) return NULL;
    
    line->row = (char *)calloc(cols + 1, sizeof(char));
    line->attributes = (char *)calloc(cols + 1, sizeof(char));
    if (!line->row || !line->attributes) {
        free(line->row);
        free(line->attributes);
        free(line);
        return NULL;
    }
    
    line->next_screen = line->prev_screen = NULL;
    line->last_char = 0;
    line->changed = 0;
    line->scroll = 0;
    
    return line;
}

void initscr(void) {
    if (curscr || stdscr) return;  // Already initialized
    
    // Initialize terminal capabilities
    if (term_init() < 0 || term_setup() < 0) {
        return;
    }
    
    // Create standard screen
    stdscr = newwin(LINES, COLS, 0, 0);
    if (!stdscr) return;
    
    // Create physical screen
    curscr = newwin(LINES, COLS, 0, 0);
    if (!curscr) {
        delwin(stdscr);
        stdscr = NULL;
        return;
    }
}

void endwin(void) {
    if (curscr) {
        delwin(curscr);
        curscr = NULL;
    }
    if (stdscr) {
        delwin(stdscr);
        stdscr = NULL;
    }
    term_reset();
}

void wrefresh(WINDOW *window) {
    if (!window || !curscr) return;
    
    // Compare window contents with physical screen
    for (int i = 0; i < window->Num_lines; i++) {
        struct _line *win_line = window->line_array[i];
        struct _line *cur_line = curscr->line_array[i + window->SR];
        
        if (win_line->changed) {
            // Copy changed line to physical screen
            memcpy(cur_line->row + window->SC, 
                   win_line->row, 
                   window->Num_cols);
            memcpy(cur_line->attributes + window->SC,
                   win_line->attributes,
                   window->Num_cols);
            cur_line->changed = 1;
        }
    }
    
    // Update cursor position
    Position(window, window->LY, window->LX);
    
    // Perform physical screen update
    doupdate();
}

void Position(WINDOW *window, int row, int col) {
    if (!window) return;
    window->LY = row;
    window->LX = col;
    
    if (window == stdscr) {
        phys_y = row;
        phys_x = col;
        if (cursor_address) {
            move_phys_cursor(row, col);
        }
    }
}

void wmove(WINDOW *window, int row, int column) {
    if (!window || row >= window->Num_lines || column >= window->Num_cols) 
        return;
    Position(window, row, column);
}

void wstandout(WINDOW *window) {
    if (!window) return;
    window->Attrib |= A_STANDOUT;
}

void wstandend(WINDOW *window) {
    if (!window) return;
    window->Attrib &= ~A_STANDOUT;
}

void waddch(WINDOW *window, int c) {
    if (!window || window->LY >= window->Num_lines || window->LX >= window->Num_cols)
        return;
        
    struct _line *line = window->line_array[window->LY];
    Char_out(c, window->Attrib, line->row, line->attributes, window->LX);
    line->changed = 1;
    
    if (window->LX < window->Num_cols - 1)
        window->LX++;
}

void Char_out(int newc, int newatt, char *line, char *attrib, int offset) {
    line[offset] = newc;
    attrib[offset] = newatt;
}

// Add window movement functions
void winsertln(WINDOW *window) {
    if (!window || window->LY >= window->Num_lines) return;
    
    // Shift lines down
    for (int i = window->Num_lines - 1; i > window->LY; i--) {
        struct _line *dest = window->line_array[i];
        struct _line *src = window->line_array[i-1];
        memcpy(dest->row, src->row, window->Num_cols);
        memcpy(dest->attributes, src->attributes, window->Num_cols);
        dest->changed = 1;
    }
    
    // Clear new line
    struct _line *newline = window->line_array[window->LY];
    memset(newline->row, ' ', window->Num_cols);
    memset(newline->attributes, 0, window->Num_cols);
    newline->changed = 1;
}

void wscroll(WINDOW *window, int lines) {
    if (!window || lines == 0) return;
    
    if (lines > 0) {  // Scroll up
        for (int i = 0; i < window->Num_lines - lines; i++) {
            struct _line *dest = window->line_array[i];
            struct _line *src = window->line_array[i + lines];
            memcpy(dest->row, src->row, window->Num_cols);
            memcpy(dest->attributes, src->attributes, window->Num_cols);
            dest->changed = 1;
        }
        // Clear newly exposed lines
        for (int i = window->Num_lines - lines; i < window->Num_lines; i++) {
            struct _line *line = window->line_array[i];
            memset(line->row, ' ', window->Num_cols);
            memset(line->attributes, 0, window->Num_cols);
            line->changed = 1;
        }
    } else {  // Scroll down
        lines = -lines;
        for (int i = window->Num_lines - 1; i >= lines; i--) {
            struct _line *dest = window->line_array[i];
            struct _line *src = window->line_array[i - lines];
            memcpy(dest->row, src->row, window->Num_cols);
            memcpy(dest->attributes, src->attributes, window->Num_cols);
            dest->changed = 1;
        }
        // Clear newly exposed lines
        for (int i = 0; i < lines; i++) {
            struct _line *line = window->line_array[i];
            memset(line->row, ' ', window->Num_cols);
            memset(line->attributes, 0, window->Num_cols);
            line->changed = 1;
        }
    }
    window->SCROLL_CLEAR = SCROLL;
}

void waddstr(WINDOW *window, char *string) {
    if (!window || !string) return;
    
    while (*string) {
        if (window->LX >= window->Num_cols) {
            if (window->LY < window->Num_lines - 1) {
                window->LX = 0;
                window->LY++;
            } else {
                wscroll(window, 1);
                window->LX = 0;
            }
        }
        waddch(window, *string++);
    }
}

void werase(WINDOW *window) {
    if (!window) return;
    
    for (int i = 0; i < window->Num_lines; i++) {
        struct _line *line = window->line_array[i];
        memset(line->row, ' ', window->Num_cols);
        memset(line->attributes, 0, window->Num_cols);
        line->changed = 1;
    }
    window->LX = window->LY = 0;
    window->SCROLL_CLEAR = CLEAR;
}

int wgetch(WINDOW *window) {
    if (!window) return ERR;
    
    char c;
    if (read(STDIN_FILENO, &c, 1) != 1)
        return ERR;
        
    if (c == '\033') {  // Escape sequence
        char seq[3];
        if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\033';
        if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\033';
        
        if (seq[0] == '[') {
            switch (seq[1]) {
                case 'A': return KEY_UP;
                case 'B': return KEY_DOWN;
                case 'C': return KEY_RIGHT;
                case 'D': return KEY_LEFT;
                case 'H': return KEY_HOME;
            }
        }
        return '\033';
    }
    
    return c;
}

void wborder(WINDOW *window, char ls, char rs, char ts, char bs, 
            char tl, char tr, char bl, char br) {
    if (!window) return;
    
    /* Use defaults if char is 0 */
    ls = ls ? ls : ACS_VLINE;
    rs = rs ? rs : ACS_VLINE;
    ts = ts ? ts : ACS_HLINE;
    bs = bs ? bs : ACS_HLINE;
    tl = tl ? tl : ACS_ULCORNER;
    tr = tr ? tr : ACS_URCORNER;
    bl = bl ? bl : ACS_LLCORNER;
    br = br ? br : ACS_LRCORNER;

    /* Draw borders */
    struct _line *top = window->line_array[0];
    struct _line *bottom = window->line_array[window->Num_lines - 1];
    
    /* Corners */
    top->row[0] = tl;
    top->row[window->Num_cols - 1] = tr;
    bottom->row[0] = bl;
    bottom->row[window->Num_cols - 1] = br;
    
    /* Top and bottom edges */
    for (int i = 1; i < window->Num_cols - 1; i++) {
        top->row[i] = ts;
        bottom->row[i] = bs;
    }
    
    /* Side edges */
    for (int i = 1; i < window->Num_lines - 1; i++) {
        struct _line *line = window->line_array[i];
        line->row[0] = ls;
        line->row[window->Num_cols - 1] = rs;
        line->changed = 1;
    }
    
    top->changed = bottom->changed = 1;
}

void box(WINDOW *window, char verch, char horch) {
    wborder(window, verch, verch, horch, horch, 0, 0, 0, 0);
}

// Add color support functions
void init_pair(short pair, short f, short b) {
    if (pair < 1 || pair > COLOR_PAIRS) return;
    // Store color pair in global table
    color_pairs[pair] = (f & 0xF) | ((b & 0xF) << 4);
}

int COLOR_PAIR(int n) {
    return ((n & 0xFF) << 8);
}

void doupdate(void) {
    save_cursor();
    
    // Update all windows from bottom to top
    sort_windows();
    WINLIST *node = top_window;
    
    // Find lowest z-order window
    while (node && node->next)
        node = node->next;
    
    // Update windows from bottom to top
    while (node) {
        WINDOW *win = node->win;
        if (win != curscr) {
            wrefresh(win);
        }
        node = node->prev;
    }
    
    restore_cursor();
    move_phys_cursor(phys_y, phys_x);
}

// ...remaining functions...
