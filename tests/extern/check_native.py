import ctypes as c
import sys


class NativeResult(c.Structure):
    _fields_ = [("status", c.c_int8), ("total", c.c_double)]


class Point(c.Structure):
    _fields_ = [("fraction", c.c_float), ("distance", c.c_double)]


class Packet(c.Structure):
    _fields_ = [
        ("tag", c.c_uint8),
        ("pos", Point),
        ("delta", c.c_int16),
        ("active", c.c_bool),
        ("serial", c.c_uint64),
    ]


def bind(library, name, result, *params):
    symbol = getattr(library, name)
    symbol.argtypes = params
    symbol.restype = result
    return symbol


exports = c.CDLL(sys.argv[1])
fixture = c.CDLL(sys.argv[2])
compute = bind(exports, "AbyssCustomExport_42", NativeResult, c.c_double, c.c_int32)
update = bind(exports, "abyss_update", None, c.POINTER(c.c_int32), c.c_int32)
only_called_from_c = bind(exports, "abyss_only_called_from_c", c.c_int32, c.c_int32)
local_export = bind(exports, "abyss_local_export", c.c_int32, c.c_int32, c.c_int32)
global_status = bind(exports, "abyss_global_status", c.c_int32, c.c_int32)

value = compute(1.25, -7)
assert (value.status, value.total) == (-7, -4.5)
updated = c.c_int32()
update(c.byref(updated), -5)
assert updated.value == -5
assert only_called_from_c(12) == 10
assert local_export(-10, 4) == 5
assert global_status(-2) == 51
assert global_status(3) == 82

source = Packet(19, Point(1.25, 6.0), -100, True, 99)
layout = bind(
    fixture, "c_layout", c.c_bool, c.POINTER(Packet), c.c_uint64,
    c.POINTER(c.c_double), c.POINTER(c.c_int16), c.POINTER(c.c_bool), c.POINTER(c.c_uint64),
)
assert layout(
    c.byref(source), c.sizeof(Packet),
    c.cast(c.byref(source, Packet.pos.offset + Point.distance.offset), c.POINTER(c.c_double)),
    c.cast(c.byref(source, Packet.delta.offset), c.POINTER(c.c_int16)),
    c.cast(c.byref(source, Packet.active.offset), c.POINTER(c.c_bool)),
    c.cast(c.byref(source, Packet.serial.offset), c.POINTER(c.c_uint64)),
)
packet = bind(fixture, "c_packet", Packet, Packet)(source)
assert (packet.tag, packet.pos.distance, packet.delta, packet.active, packet.serial) == (20, 12.0, -98, False, 100)
