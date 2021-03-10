#ifndef EVDI_SYS_WRAPPER_H
#define EVDI_SYS_WRAPPER_H

struct wrapper_log_cb {
    void (*function)(void *user_data, const char *msg);
    void *user_data;
} wrapper_log_cb;

void wrapper_evdi_set_logging(struct wrapper_log_cb cb);

#endif //EVDI_SYS_WRAPPER_H
