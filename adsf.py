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
VFPU4 = 0b110100 << 26
VFPU5 = 0b110111 << 26
VFPU6 = 0b111100 << 26
VFPU7 = 0b111111 << 26

def func_start(name: str):
    print(f".type {name},@function")
    print(f".globl {name}")
    print(f"{name}:")

def func_end(name: str):
    print()
    print(f"    jr $ra")
    print(f".size {name}, . - {name}")


def tp(x: int) -> int:
    assert x in range(1<<2), f"0x{x:X}"
    t = (x & 0b00010) >> 1
    p = (x & 0b00001) >> 0
    return (t << 15) | (p << 7)

def vfpu0_fmt_tp(x: int) -> int:
    assert x in range(1<<5), f"0x{x:X}"
    fmt = (x & 0b11100) >> 2
    t = (x & 0b00010) >> 1
    p = (x & 0b00001) >> 0
    return (fmt << 23) | (t << 15) | (p << 7)

def vfpu4_fmt(x: int) -> int:
    assert x in range(1<<2), f"0x{x:X}"
    return x << 24

def vfpu4_fmt_fmt(x: int) -> int:
    assert x in range(1<<3), f"0x{x:X}"
    return x << 21

def vfpu4_fmt_fmt_fmt(x: int) -> int:
    assert x in range(1<<2), f"0x{x:X}"
    return x << 19

def vfpu4_fmt_fmt_fmt_fmt_tp(x: int) -> int:
    assert x in range(1<<5), f"0x{x:X}"
    fmt = (x & 0b11100) >> 2
    t = (x & 0b00010) >> 1
    p = (x & 0b00001) >> 0
    return (fmt << 16) | (t << 15) | (p << 7)

def vfpu6_fmt(x: int) -> int:
    assert x in range(1<<3), f"0x{x:X}"
    return x << 23

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

def do_vfp3_vcmp_zero():
    func_start("vcmp_zero")
    print("    # vcmp zero")

    VT = vt(0)
    VS = vs(0)

    i = 0

    for cond in range(1<<4):
        for i in range(4):
            print(f"    .word 0x{VFPU3:08X} | 0x{vfpu0_fmt_tp(i):08X} | 0x{VT:08X} | 0x{VS:08X} | 0x{cond:08X}")
    func_end("vcmp_zero")

def do_vfp4_fmt0():
    func_start("vfpu4_fmt0_all")
    print("    # VFPU4 FMT0")

    VT = vt(1 << 6)
    VS = vs(1 << 4)
    VD = vd(1 << 0)

    fmt = vfpu4_fmt(0)

    for j in range(1<<3):
        for k in range(1<<2):
            for l in range(1<<5):
                print(f"    .word 0x{VFPU4:08X} | 0x{fmt:08X} | 0x{vfpu4_fmt_fmt(j):08X} | 0x{vfpu4_fmt_fmt_fmt(k):08X} | 0x{vfpu4_fmt_fmt_fmt_fmt_tp(l):08X} | 0x{VS:08X} | 0x{VD:08X}")
    func_end("vfpu4_fmt0_all")

def do_vfp4_fmt2():
    func_start("vfpu4_fmt2_all")
    print("    # VFPU4 FMT2")

    VT = vt(1 << 6)
    VS = vs(1 << 4)
    VD = vd(1 << 0)

    fmt = vfpu4_fmt(2)

    for j in range(1<<3):
        for k in range(1<<2):
            for l in range(1<<5):
                print(f"    .word 0x{VFPU4:08X} | 0x{fmt:08X} | 0x{vfpu4_fmt_fmt(j):08X} | 0x{vfpu4_fmt_fmt_fmt(k):08X} | 0x{vfpu4_fmt_fmt_fmt_fmt_tp(l):08X} | 0x{VS:08X} | 0x{VD:08X}")
    func_end("vfpu4_fmt2_all")

def do_vfp4_fmt3():
    func_start("vfpu4_fmt3_all")
    print("    # VFPU4 FMT3")

    VT = vt(1 << 6)
    VS = vs(1 << 4)
    VD = vd(1 << 0)

    fmt = vfpu4_fmt(3)

    for j in range(1<<8):
        print(f"    .word 0x{VFPU4:08X} | 0x{fmt:08X} | 0x{j<<16:08X} | 0x{tp(0):08X} | 0x{VS:08X} | 0x{VD:08X}")
    func_end("vfpu4_fmt3_all")

def do_vfp5():
    func_start("vfpu5_all")
    print("    # VFPU5")

    VT = vt(1 << 6)
    VS = vs(1 << 4)
    VD = vd(1 << 0)

    for i in range(1<<2):
        fmt = i << 24
        if i != 3:
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x000000:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x000001:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x000010:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x000100:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x001000:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x010000:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x100000:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0x800000:08X}")
            print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{0xF00000:08X}")
        else:
            for j in range(1<<1):
                print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{j << 23:08X} | 0x{VT:08X} | 0x{0x0000:08X}")
                print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{j << 23:08X} | 0x{VT:08X} | 0x{0x0001:08X}")
                print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{j << 23:08X} | 0x{VT:08X} | 0x{0x0010:08X}")
                print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{j << 23:08X} | 0x{VT:08X} | 0x{0x0100:08X}")
                print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{j << 23:08X} | 0x{VT:08X} | 0x{0x1000:08X}")
                print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{j << 23:08X} | 0x{VT:08X} | 0x{0x8000:08X}")
                print(f"    .word 0x{VFPU5:08X} | 0x{fmt:08X} | 0x{j << 23:08X} | 0x{VT:08X} | 0x{0xF000:08X}")
    func_end("vfpu5_all")

def do_vfpu6():
    func_start("vfpu6_all")
    print("    # VFPU6")

    VT = vt(0)
    VS = vs(1 << 4)
    VD = vd(1 << 6)

    for i in range(1<<3):
        if i == 7:
            for j in range(1<<2):
                if j == 0:
                    for k in range(1 << 3):
                        for t in range(1 << 1):
                            for p in range(1 << 1):
                                print(f"    .word 0x{VFPU6:08X} | 0x{i << 23:08X} | 0x{j << 21:08X} | 0x{k << 16:08X} | 0x{t << 15:08X} | 0x{VS:08X} | 0x{p << 7:08X} | 0x{VD:08X}")
                else:
                    for t in range(1 << 1):
                        for p in range(1 << 1):
                            print(f"    .word 0x{VFPU6:08X} | 0x{i << 23:08X} | 0x{j << 21:08X} | 0x{VT:08X} | 0x{t << 15:08X} | 0x{VS:08X} | 0x{p << 7:08X} | 0x{VD:08X}")
        else:
            for t in range(1 << 1):
                for p in range(1 << 1):
                    print(f"    .word 0x{VFPU6:08X} | 0x{i << 23:08X} | 0x{VT:08X} | 0x{t << 15:08X} | 0x{VS:08X} | 0x{p << 7:08X} | 0x{VD:08X}")
    func_end("vfpu6_all")

do_vfp3_vcmp_zero()
