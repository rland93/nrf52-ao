#ifndef RUST_EXPORTS_H
#define RUST_EXPORTS_H

#include <stdint.h>

extern void rust_egu_dispatcher(uint8_t);
extern void rust_app_main(void);
extern void rust_post_heartbeat_event(void);

#endif
