#ifndef NRFX_CONFIG_H__
#define NRFX_CONFIG_H__

// project overrides (e.g. enabling a driver, CPU freq) go above the includes.

// timer
#define NRFX_TIMER_ENABLED 1
#define NRFX_TIMER1_ENABLED 1

// pdm microphone
#define NRFX_PDM_ENABLED 1
#define NRFX_PDM_DEFAULT_CONFIG_IRQ_PRIORITY 6

// lf clocks
#define NRFX_CLOCK_ENABLED 1
#define NRFX_CLOCK_CONFIG_LF_SRC 1
#define NRFX_CLOCK_CONFIG_LF_CAL_ENABLED 1
#define NRFX_RTC_ENABLED 1

#include <nrfx_config_common.h>
#include <nrfx_config_nrf52840.h>

#endif
