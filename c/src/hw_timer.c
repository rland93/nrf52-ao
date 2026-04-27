#include "hw_timer.h"

#include <nrfx_timer.h>

#include "rust_exports.h"

nrfx_timer_t TIMER_HEARTBEAT = NRFX_TIMER_INSTANCE(NRF_TIMER1);

// timer1 IRQ handler
void timer_event_handler(nrf_timer_event_t event_type, void* p_context) {
  // arg unused.
  (void)p_context;
  if (event_type == NRF_TIMER_EVENT_COMPARE0) {
    rust_post_heartbeat_event();
  }
}

// initialize the timer
void init_heartbeat_timer(void) {
  nrfx_timer_config_t timer_cfg = NRFX_TIMER_DEFAULT_CONFIG(31250U);
  nrfx_timer_init(&TIMER_HEARTBEAT, &timer_cfg, timer_event_handler);
  uint32_t ticks = nrfx_timer_ms_to_ticks(&TIMER_HEARTBEAT, 500);

  nrfx_timer_extended_compare(&TIMER_HEARTBEAT, NRF_TIMER_CC_CHANNEL0, ticks,
                              // reset automatically so as to generate another
                              // event in 500ms
                              NRF_TIMER_SHORT_COMPARE0_CLEAR_MASK, true);

  nrfx_timer_enable(&TIMER_HEARTBEAT);
}

void TIMER1_IRQHandler(void) { nrfx_timer_irq_handler(&TIMER_HEARTBEAT); }
