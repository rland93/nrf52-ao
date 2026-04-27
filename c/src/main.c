#include "hw_egu.h"
#include "hw_gpio.h"
#include "hw_timer.h"
#include "rust_exports.h"

int main(void) {
  hw_gpio_led_init();
  egu_init();
  init_heartbeat_timer();

  // rust app does not return.
  rust_app_main();
}
