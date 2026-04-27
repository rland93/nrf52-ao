#ifndef HW_TIMER_H
#define HW_TIMER_H

#include <stdbool.h>
#include <nrfx_timer.h>

// called from C to initialize
void init_sys_timer3(void);

// get current ticks for a channel
uint32_t    c_timer3_current_ticks(uint8_t channel);

// set compare value for a channel; trigger interrupt when timer reaches this value.
void        c_timer3_set(uint8_t channel, uint32_t ticks);

// clear the interrupt for a channel
void        c_timer3_clear(uint8_t channel);

// get the value for a channel
uint32_t    c_timer3_capture(uint8_t channel);

// enable the interrupt for a channel
void        c_timer3_enable_compare_int(uint8_t channel);

// start the timer
void        c_timer3_start(void);

// stop the timer
void        c_timer3_stop(void);

#endif // HW_TIMER_H
