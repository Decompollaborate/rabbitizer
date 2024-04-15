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
VFPU3 = 0b011011 << 26
VFPU4 = 0b111000 << 26
VFPU5 = 0b111001 << 26
VFPU6 = 0b111010 << 26
VFPU7 = 0b111011 << 26

def func_start(name: str):
    print(f".type {name},@function")
    print(f".globl {name}")
    print(f"{name}:")

def func_end(name: str):
    print()
    print(f"    jr $ra")
    print(f".size {name}, . - {name}")


def vfpu0_fmt_tp(x: int) -> int:
    assert x in range(1<<5)
    fmt = (x & 0b11100) >> 2
    t = (x & 0b00010) >> 1
    p = (x & 0b00001) >> 0
    return (fmt << 23) | (t << 15) | (p << 7)

def do_vfpu0():
    func_start("vfpu0_all")
    print("    # VFPU0")

    VT = vt(0)
    VS = vs(1 << 4)
    VD = vd(1 << 6)

    for i in range(1<<5):
        print(f"    .word 0x{VFPU0:08X} | 0x{vfpu0_fmt_tp(i):08X} | 0x{VT:08X} | 0x{VS:08X} | 0x{VD:08X}")
    func_end("vfpu0_all")

def do_vfp1():
    func_start("vfpu1_all")
    print("    # VFPU1")

    VT = vt(0)
    VS = vs(1 << 4)
    VD = vd(1 << 6)

    for i in range(1<<5):
        print(f"    .word 0x{VFPU1:08X} | 0x{vfpu0_fmt_tp(i):08X} | 0x{VT:08X} | 0x{VS:08X} | 0x{VD:08X}")
    func_end("vfpu1_all")

def do_vfp3():
    func_start("vfpu3_all")
    print("    # VFPU3")

    VT = vt(1 << 6)
    VS = vs(1 << 4)
    VD = vd(1 << 0)

    for i in range(1<<5):
        print(f"    .word 0x{VFPU3:08X} | 0x{vfpu0_fmt_tp(i):08X} | 0x{VT:08X} | 0x{VS:08X} | 0x{VD:08X}")
    func_end("vfpu3_all")

def do_vfp3_vcmp():
    func_start("vcmp_all")
    print("    # vcmp.")

    VT = vt(1 << 6)
    VS = vs(1 << 3)

    i = 0

    for cond in range(1<<4):
        for i in range(4):
            print(f"    .word 0x{VFPU3:08X} | 0x{vfpu0_fmt_tp(i):08X} | 0x{VT:08X} | 0x{VS:08X} | 0x{cond:08X}")
    func_end("vcmp_all")

do_vfp1()
