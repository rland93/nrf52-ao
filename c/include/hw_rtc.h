#ifndef HW_RTC_H
#define HW_RTC_H

#include <nrfx_rtc.h>
#include <stdint.h>

// called from C to initialize the RTC
void hw_rtc_init(void);

void c_rtc_start(void);
void c_rtc_stop(void);

// read current counter value
uint32_t c_rtc_read_current_ticks(void);

// set CC0
void c_rtc_set_cc0(uint32_t ticks);

// clear CC0 interrupt
void c_rtc_clear_cc0(void);


#endif // HW_RTC_H
