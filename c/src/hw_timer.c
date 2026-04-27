#include "hw_timer.h"

#include <nrfx_timer.h>

#include "interrupt_priorities.h"
#include "nrf_timer.h"
#include "rust_exports.h"

nrfx_timer_t TIMER3_SYS = NRFX_TIMER_INSTANCE(NRF_TIMER3);

// statics
static void timer3_handler(nrf_timer_event_t event_type, void* p_context);

void TIMER3_IRQHandler(void) { nrfx_timer_irq_handler(&TIMER3_SYS); }

// initialize TIMER3 (the 'sys timer')
void init_sys_timer3(void) {
  nrfx_timer_config_t cfg = {.frequency = 31250U,
                             .mode = NRF_TIMER_MODE_TIMER,
                             .bit_width = NRF_TIMER_BIT_WIDTH_32,
                             .interrupt_priority = SYS_TIMER3_IRQ_PRIORITY,
                             .p_context = NULL};
  nrfx_timer_init(&TIMER3_SYS, &cfg, timer3_handler);
}

// handler for timer 3 (the 'sys' timer). Calls into Rust to handle the event
// it passes the channel number to rust handler.
static void timer3_handler(nrf_timer_event_t event_type, void* p_context) {
  (void)p_context;  // unused
  if (event_type == NRF_TIMER_EVENT_COMPARE0) {
    rust_timer3_event_handler(0);
  }
  if (event_type == NRF_TIMER_EVENT_COMPARE1) {
    rust_timer3_event_handler(1);
  }
  if (event_type == NRF_TIMER_EVENT_COMPARE2) {
    rust_timer3_event_handler(2);
  }
  if (event_type == NRF_TIMER_EVENT_COMPARE3) {
    rust_timer3_event_handler(3);
  }
  if (event_type == NRF_TIMER_EVENT_COMPARE4) {
    rust_timer3_event_handler(4);
  }
  if (event_type == NRF_TIMER_EVENT_COMPARE5) {
    rust_timer3_event_handler(5);
  }
}

// get the current ticks for a channel
uint32_t c_timer3_current_ticks(uint8_t channel) {
  return nrfx_timer_capture(&TIMER3_SYS, (nrf_timer_cc_channel_t)channel);
}

void c_timer3_set(uint8_t channel, uint32_t ticks) {
  nrfx_timer_compare(&TIMER3_SYS, (nrf_timer_cc_channel_t)channel, ticks, true);
}

void c_timer3_clear(uint8_t channel) {
  nrfx_timer_compare_int_disable(&TIMER3_SYS, channel);
}

void c_timer3_start(void) { nrfx_timer_enable(&TIMER3_SYS); }

void c_timer3_stop(void) { nrfx_timer_disable(&TIMER3_SYS); }
