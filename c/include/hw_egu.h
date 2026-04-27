#ifndef HW_EGU_H
#define HW_EGU_H
#include <stdint.h>

// initialize egu's with the priority levels 3,5,6,7
// NOTE: events cannot be triggered before this function runs!
void egu_init(void);

// trigger EGU to kick off a given priority p
// if p is not one of the valid priorities (3,5,6,7) this function does nothing.
void c_trigger_egu(uint8_t p);


#endif
