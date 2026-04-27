// hw_egu.c
//
#include <nrfx_egu.h>

#include "rust_exports.h"

static nrfx_egu_t egu0_p3 = NRFX_EGU_INSTANCE(NRF_EGU0);
static nrfx_egu_t egu1_p5 = NRFX_EGU_INSTANCE(NRF_EGU1);
static nrfx_egu_t egu2_p6 = NRFX_EGU_INSTANCE(NRF_EGU2);
static nrfx_egu_t egu3_p7 = NRFX_EGU_INSTANCE(NRF_EGU3);

static void egu_p3_handler(uint8_t, void *);
static void egu_p5_handler(uint8_t, void *);
static void egu_p6_handler(uint8_t, void *);
static void egu_p7_handler(uint8_t, void *);

// set up and configure the EGU for our 4 prio levels.
// priority levels are:
//
// - 0 reserved by softdevice
// - 1 reserved by softdevice
// - 2 reserved by softdevice
// - 3 xhigh, egu0
// - 4 reserved by softdevice
// - 5 high, egu1
// - 6 medium, egu2
// - 7 low, egu3
//
void egu_init() {
  nrfx_egu_init(&egu0_p3, 3, egu_p3_handler, NULL);
  nrfx_egu_int_enable(&egu0_p3, NRF_EGU_INT_ALL);

  nrfx_egu_init(&egu1_p5, 5, egu_p5_handler, NULL);
  nrfx_egu_int_enable(&egu1_p5, NRF_EGU_INT_ALL);

  nrfx_egu_init(&egu2_p6, 6, egu_p6_handler, NULL);
  nrfx_egu_int_enable(&egu2_p6, NRF_EGU_INT_ALL);

  nrfx_egu_init(&egu3_p7, 7, egu_p7_handler, NULL);
  nrfx_egu_int_enable(&egu3_p7, NRF_EGU_INT_ALL);
}

// trigger EGU to kick off a given priority p
// if p is not one of the valid priorities (3,5,6,7) this function does nothing.
void c_trigger_egu(uint8_t p) {
  switch (p) {
    case 3: {
      nrfx_egu_trigger(&egu0_p3, 0);
      break;
    }
    case 5: {
      nrfx_egu_trigger(&egu1_p5, 0);
      break;
    }
    case 6: {
      nrfx_egu_trigger(&egu2_p6, 0);
      break;
    }
    case 7: {
      nrfx_egu_trigger(&egu3_p7, 0);
      break;
    }
    default: {
      break;
    }
  }
}

static void egu_p3_handler(uint8_t event_idx, void *p_context) {
  (void)event_idx;
  (void)p_context;
  rust_egu_dispatcher(3);
}

static void egu_p5_handler(uint8_t event_idx, void *p_context) {
  (void)event_idx;
  (void)p_context;
  rust_egu_dispatcher(5);
}

static void egu_p6_handler(uint8_t event_idx, void *p_context) {
  (void)event_idx;
  (void)p_context;
  rust_egu_dispatcher(6);
}
static void egu_p7_handler(uint8_t event_idx, void *p_context) {
  (void)event_idx;
  (void)p_context;
  rust_egu_dispatcher(7);
}

void SWI0_EGU0_IRQHandler(void) { nrfx_egu_irq_handler(&egu0_p3); }
void SWI1_EGU1_IRQHandler(void) { nrfx_egu_irq_handler(&egu1_p5); }
void SWI2_EGU2_IRQHandler(void) { nrfx_egu_irq_handler(&egu2_p6); }
void SWI3_EGU3_IRQHandler(void) { nrfx_egu_irq_handler(&egu3_p7); }
