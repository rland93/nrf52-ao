#include "hw_gpio.h"

#include "nrf_gpio.h"

#define LED_R_PIN NRF_GPIO_PIN_MAP(0, 26)
#define LED_G_PIN NRF_GPIO_PIN_MAP(0, 30)
#define LED_B_PIN NRF_GPIO_PIN_MAP(0, 6)

// LEDs are current-sink to 3v3 rail: clear pin = sink = LED on.

void hw_gpio_led_init(void) {
  nrf_gpio_cfg_output(LED_R_PIN);
  nrf_gpio_cfg_output(LED_G_PIN);
  nrf_gpio_cfg_output(LED_B_PIN);
  nrf_gpio_pin_set(LED_R_PIN);
  nrf_gpio_pin_set(LED_G_PIN);
  nrf_gpio_pin_set(LED_B_PIN);
}

void c_gpio_led_set(bool r, bool g, bool b) {
  if (r) {
    nrf_gpio_pin_clear(LED_R_PIN);
  } else {
    nrf_gpio_pin_set(LED_R_PIN);
  }

  if (g) {
    nrf_gpio_pin_clear(LED_G_PIN);
  } else {
    nrf_gpio_pin_set(LED_G_PIN);
  }

  if (b) {
    nrf_gpio_pin_clear(LED_B_PIN);
  } else
    nrf_gpio_pin_set(LED_B_PIN);
}
