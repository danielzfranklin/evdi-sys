#include <stdio.h>
#include <stdint.h>
#include "../vendor/evdi/library/evdi_lib.h"
#include "wrapper.h"
#include <stdarg.h>

#define UNUSED(x) (void)(x)

struct wrapper_log_cb current_log_cb = {
    .function = NULL,
    .user_data = NULL
};

#define MAX_LOG_MSG_SIZE 1000

void wrapper_logging_cb(void * _user_data, const char *fmt, ...) {
    UNUSED(_user_data);

    void (*fn)(void *user_data, const char *msg) = current_log_cb.function;
    void *user_data = current_log_cb.user_data;

    va_list args;
    va_start(args, fmt);

    if (fn != NULL) {
        char buf[MAX_LOG_MSG_SIZE];
        vsnprintf(buf, MAX_LOG_MSG_SIZE, fmt, args);

        (fn)(user_data, buf);
    }

    va_end(args);
}

void wrapper_evdi_set_logging(struct wrapper_log_cb cb) {
    current_log_cb = cb;

    struct evdi_logging logging;
    logging.function = &wrapper_logging_cb;

    evdi_set_logging(logging);
}
