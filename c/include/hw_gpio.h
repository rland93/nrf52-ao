#ifndef HW_GPIO_H
#define HW_GPIO_H

#include <stdbool.h>

void hw_gpio_led_init(void);
void c_gpio_led_set(bool r, bool g, bool b);

#endif
