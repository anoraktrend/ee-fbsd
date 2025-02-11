#ifndef EE_CURSE_IMPL_H
#define EE_CURSE_IMPL_H

#include "curse_types.h"
#include "common.h"

/* Window initialization */
NODISCARD WINDOW *newwin(const int lines, const int cols,
                        const int start_l, const int start_c);

static ee_error_t win_init(WINDOW * const NONNULL win,
                          const int lines, const int cols, 
                          const int start_l, const int start_c);

static void win_free_lines(WINDOW * const NONNULL win, const int count);

/* Window operations */
void delwin(WINDOW * const NULLABLE win);
void wrefresh(WINDOW * const NULLABLE win);
void wclear(WINDOW * const NULLABLE win);
void wmove(WINDOW * const NULLABLE win, const int y, const int x);

/* Attribute handling */
void wattron(WINDOW * const NULLABLE win, const int attrs);
void wattroff(WINDOW * const NULLABLE win, const int attrs);
void wattrset(WINDOW * const NULLABLE win, const int attrs);

#endif /* EE_CURSE_IMPL_H */
