#ifndef ISR_MASK_H
#define ISR_MASK_H
#include <stdint.h>

// enter critical section; returns PRIMASK.
uint32_t c_enter_critical(void);

// exit critical section; pass PRIMASK in to restore.
void c_exit_critical(uint32_t primask);

#endif
