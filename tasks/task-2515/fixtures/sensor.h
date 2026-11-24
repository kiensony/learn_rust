#include <stdint.h>
struct packet { int16_t reading; uint8_t scale; };
int32_t track_sensor_value(struct packet);
