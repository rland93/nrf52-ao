#include "hw_rtc.h"

#include "nrfx_rtc.h"
#include "rust_exports.h"

// the channel to use for sys events on the RTC
#define RTCSYS_CHANNEL 0

// static driver instance for RTC0
static nrfx_rtc_t RTC0_SYS = NRFX_RTC_INSTANCE(RTCSYS_CHANNEL);

// RTC0 irq handler
static void rtc0_handler(nrfx_rtc_int_type_t int_type);

// called from C to initialize the RTC
void hw_rtc_init(void) {
  nrfx_rtc_config_t config = NRFX_RTC_DEFAULT_CONFIG;
  config.interrupt_priority = 2;
  nrfx_rtc_init(&RTC0_SYS, &config, rtc0_handler);
  return;
}

// RTC0 irq handler
static void rtc0_handler(nrfx_rtc_int_type_t int_type) {
  if (int_type == NRFX_RTC_INT_COMPARE0) {
    rust_sys_event_handler();
  }
  return;
}

// called from C to start the RTC
void c_rtc_start(void) {
  nrfx_rtc_enable(&RTC0_SYS);
  return;
}

// called from C to stop the RTC
void c_rtc_stop(void) {
  nrfx_rtc_disable(&RTC0_SYS);
  return;
}

// read current counter value
uint32_t c_rtc_read_current_ticks(void) {
  uint32_t ticks = nrfx_rtc_counter_get(&RTC0_SYS);
  return ticks;
}

// set CC0
void c_rtc_set_cc0(uint32_t ticks) {
  nrfx_rtc_cc_set(&RTC0_SYS, RTCSYS_CHANNEL, ticks, true);
  return;
}

// clear CC0 interrupt
void c_rtc_clear_cc0(void) {
  nrfx_rtc_cc_disable(&RTC0_SYS, RTCSYS_CHANNEL);
  return;
}
