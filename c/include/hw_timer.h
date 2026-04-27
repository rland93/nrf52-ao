#ifndef HW_TIMER_H
#define HW_TIMER_H

#include <stdbool.h>
#include <nrfx_timer.h>

// timer1 IRQ handler
void timer_event_handler(nrf_timer_event_t event_type, void* p_context);

// initialize the timer
void init_heartbeat_timer(void);

#endif // HW_TIMER_H
