#ifndef EE_ERROR_H
#define EE_ERROR_H

#include <stddef.h>
#include "common.h"

/* Error state */
extern _Thread_local ee_error_t last_error;
extern _Thread_local char error_msg[256];

/* Error handling */
void ee_set_error(ee_error_t err, const char *msg);
const char *ee_get_error_msg(void);
ee_error_t ee_get_last_error(void);
void ee_clear_error(void);

/* Error checking macros */
#define CHECK_NULL(ptr) \
    do { \
        if ((ptr) == NULL) { \
            ee_set_error(EE_ERR_NULL_PTR, "NULL pointer"); \
            return EE_ERR_NULL_PTR; \
        } \
    } while(0)

#define CHECK_RANGE(val, min, max) \
    do { \
        if ((val) < (min) || (val) > (max)) { \
            ee_set_error(EE_ERR_INVALID_ARG, "Value out of range"); \
            return EE_ERR_INVALID_ARG; \
        } \
    } while(0)

#endif /* EE_ERROR_H */
