#!/usr/bin/env python3

def vt(x: int) -> int:
    assert x in range(1<<7)
    return x << 16

def vs(x: int) -> int:
    assert x in range(1<<7)
    return x << 8

def vd(x: int) -> int:
    assert x in range(1<<7)
    return x << 0

VFPU0 = 0b011000 << 26
VFPU1 = 0b011001 << 26

def vfpu0_fmt_tp(x: int) -> int:
    assert x in range(1<<5)
    fmt = (x & 0b11100) >> 2
    t = (x & 0b00010) >> 1
    p = (x & 0b00001) >> 0
    return (fmt << 23) | (t << 15) | (p << 7)

VT = vt(0)
VS = vs(1 << 4)
VD = vd(1 << 6)

def do_vfpu0():
    print("    # VFPU0")

    for i in range(1<<5):
        print(f"    .word 0x{VFPU0:08X} | 0x{vfpu0_fmt_tp(i):08X} | 0x{VT:08X} | 0x{VS:08X} | 0x{VD:08X}")

def do_vfp1():
    print("    # VFPU1")

    for i in range(1<<5):
        print(f"    .word 0x{VFPU1:08X} | 0x{vfpu0_fmt_tp(i):08X} | 0x{VT:08X} | 0x{VS:08X} | 0x{VD:08X}")

do_vfp1()
