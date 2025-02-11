#ifndef CURSE_TYPES_H
#define CURSE_TYPES_H

#include "config.h"

struct _line {
    struct _line *next_screen;
    struct _line *prev_screen;
    char *row;
    char *attributes;
    int last_char;
    int changed;
    int scroll;
    int number;
};

/* Color definitions */
#define COLOR_BLACK   0
#define COLOR_RED     1
#define COLOR_GREEN   2
#define COLOR_YELLOW  3
#define COLOR_BLUE    4
#define COLOR_MAGENTA 5
#define COLOR_CYAN    6
#define COLOR_WHITE   7

#define COLOR_PAIRS   64
extern char color_pairs[COLOR_PAIRS];

typedef struct WIND {
    int SR;             /* starting row */
    int SC;             /* starting column */
    int LC;             /* last column */
    int LX;             /* last cursor column position */
    int LY;             /* last cursor row position */
    int Attrib;         /* attributes active in window */
    int Num_lines;      /* number of lines */
    int Num_cols;       /* number of columns */
    int scroll_up;      /* number of lines moved */
    int scroll_down;
    int SCROLL_CLEAR;   /* indicates window has been scrolled/cleared */
    int z_order;        /* Window stacking order */
    int visible;        /* Window visibility flag */
    struct _line *first_line;
    struct _line **line_array;
} WINDOW;

#endif /* CURSE_TYPES_H */
