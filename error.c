#include "error.h"
#include <string.h>

_Thread_local ee_error_t last_error = EE_SUCCESS;
_Thread_local char error_msg[256] = {0};

void ee_set_error(ee_error_t err, const char *msg) {
    last_error = err;
    if (msg) {
        strncpy(error_msg, msg, sizeof(error_msg) - 1);
        error_msg[sizeof(error_msg) - 1] = '\0';
    } else {
        error_msg[0] = '\0';
    }
}

const char *ee_get_error_msg(void) {
    return error_msg;
}

ee_error_t ee_get_last_error(void) {
    return last_error;
}

void ee_clear_error(void) {
    last_error = EE_SUCCESS;
    error_msg[0] = '\0';
}
