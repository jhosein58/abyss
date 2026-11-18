#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    float fraction;
    double distance;
} Point;

typedef struct {
    uint8_t tag;
    Point pos;
    int16_t delta;
    bool active;
    uint64_t serial;
} Packet;

typedef struct {
    int8_t value;
} Tiny;

typedef Packet (*PacketCallback)(Packet, double);

typedef struct {
    uint16_t id;
    PacketCallback update;
} Handler;

static int8_t state;
static Handler handler;

void c_reset(void) { state = 0; }
void c_store(int8_t value) { state = value; }
int8_t c_read(void) { return state; }
bool c_bool(bool value) { return !value; }
int16_t *c_identity_pointer(int16_t **value) { return *value; }

Tiny c_tiny(Tiny value) {
    ++value.value;
    return value;
}

Packet c_packet(Packet value) {
    ++value.tag;
    value.pos.distance *= 2;
    value.delta += 2;
    value.active = !value.active;
    ++value.serial;
    return value;
}

bool c_layout(Packet *p, uint64_t size, double *distance, int16_t *delta, bool *active, uint64_t *serial) {
    return sizeof(*p) == size && &p->pos.distance == distance && &p->delta == delta &&
        &p->active == active && &p->serial == serial && p->tag == 19 &&
        p->pos.fraction == 1.25f && *distance == 6.0 && *delta == -100 && *active && *serial == 99;
}

int32_t c_invoke(int32_t value, int32_t (*apply)(int32_t)) { return apply(value); }
void c_register(Handler value) { handler = value; }
Packet c_dispatch(Packet *value) {
    if (handler.id != 6) abort();
    return handler.update(*value, 3.0);
}

int32_t c_double(int32_t value) { return value * 2; }

typedef int32_t (*Unary)(int32_t);
Unary c_get_mapper(void) { return c_double; }
void c_call_select(void (*select)(void)) { select(); }

double C_ABI_Mixed_widths_0123456789(int8_t a, uint16_t b, uint16_t c, int32_t d, uint8_t e,
    int64_t f, uint32_t g, uint64_t h, float i, double j, bool k) {
    if (a != -5 || b != 65000 || c != 30000 || d != -90000 || e != 250 ||
        f != -INT64_C(4294967296) || g != UINT32_C(4000000000) || h != (UINT64_C(1) << 63) ||
        i != 1.25f || j != -2.5 || !k) abort();
    return j - i;
}
