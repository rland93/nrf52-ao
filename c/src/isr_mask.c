/// functions to mask ISR
#include "isr_mask.h"

#include "nrf.h"
#include "rust_exports.h"

uint32_t c_enter_critical(void) {
  uint32_t primask = __get_PRIMASK();
  __disable_irq();
  return primask;
}

void c_exit_critical(uint32_t primask) { __set_PRIMASK(primask); }
