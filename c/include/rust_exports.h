#ifndef RUST_EXPORTS_H
#define RUST_EXPORTS_H

#include <stdint.h>

extern void rust_egu_dispatcher(uint8_t);
extern void rust_app_main(void);
extern void rust_post_heartbeat_event(void);
extern void rust_timer3_event_handler(uint8_t channel);
extern void rust_sys_event_handler(void);

#endif
