/// functions to mask ISR
#include "isr_mask.h"

#include "nrf.h"           // IWYU pragma: keep
#include "rust_exports.h"  // IWYU pragma: keep

uint32_t c_enter_critical(void) {
  uint32_t primask = __get_PRIMASK();
  __disable_irq();
  return primask;
}

void c_exit_critical(uint32_t primask) { __set_PRIMASK(primask); }
