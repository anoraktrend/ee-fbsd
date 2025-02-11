#ifndef EE_COMMON_H
#define EE_COMMON_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/* Error codes */
typedef enum {
    EE_SUCCESS = 0,
    EE_ERR_NULL_PTR = -1,
    EE_ERR_INVALID_ARG = -2,
    EE_ERR_ALLOC_FAILED = -3,
    EE_ERR_IO_ERROR = -4
} ee_error_t;

/* Status codes */
typedef enum {
    STATUS_OK = 0,
    STATUS_ERROR = 1,
    STATUS_EOF = 2
} status_t;

/* Function attributes */
#if defined(__GNUC__) || defined(__clang__)
#define NODISCARD __attribute__((warn_unused_result))
#define NONNULL _Nonnull
#define NULLABLE _Nullable 
#else
#define NODISCARD
#define NONNULL
#define NULLABLE
#endif

#define ARRAY_SIZE(x) (sizeof(x) / sizeof((x)[0]))

#endif /* EE_COMMON_H */
