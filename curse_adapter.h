#ifndef EE_CURSE_ADAPTER_H
#define EE_CURSE_ADAPTER_H

#include <stdbool.h>
#include "new_curse.h"

/* Screen state */
extern WINDOW * restrict edit_window;    /* Main editing window */
extern WINDOW * restrict status_window;  /* Status line window */
extern bool screen_changed;             /* Screen update flag */

/* Initialization */
_Bool screen_init(void);
void screen_end(void);

/* Window operations */
void create_windows(void);
void refresh_screen(void);
void clear_screen(void);
void clear_to_eol(void);

/* Cursor operations */
void move_cursor(const int row, const int col);
void get_cursor_pos(int * restrict row, int * restrict col);

/* Output operations */
void put_char(const int ch);
void put_string(const char * restrict str);
void set_attribute(const int attr);
void clear_attribute(const int attr);

/* Input operations */ 
int get_key(void);
void flush_input(void);

#endif /* EE_CURSE_ADAPTER_H */
