#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2026 Decompollaborate
# SPDX-License-Identifier: MIT

"""
This file should remain compatible with the version listed in the
"requires-python" version from the pyproject.toml file.
At the time of writing: Python 3.4
"""

import rabbitizer
import sys

class TestEntry:
    # word: int
    # immOverride: Optional[str]
    # expectedStr: str
    # category: rabbitizer.Enum
    # gnuMode: bool = True

    def __init__(self, word, immOverride, expectedStr, category, gnuMode = True):
        self.word = word
        self.immOverride = immOverride
        self.expectedStr = expectedStr
        self.category = category
        self.gnuMode = gnuMode


TEST_ENTRIES_PLAIN = [
    TestEntry(0x3C088001, None,                  "lui         $t0, 0x8001", rabbitizer.InstrCategory.CPU),
    TestEntry(0x25080E60, None,                  "addiu       $t0, $t0, 0xE60", rabbitizer.InstrCategory.CPU),
    TestEntry(0x3C090002, None,                  "lui         $t1, 0x2", rabbitizer.InstrCategory.CPU),
    TestEntry(0x25298DE0, None,                  "addiu       $t1, $t1, -0x7220", rabbitizer.InstrCategory.CPU),
    TestEntry(0xAD000000, None,                  "sw          $zero, 0x0($t0)", rabbitizer.InstrCategory.CPU),
    TestEntry(0xAD000004, None,                  "sw          $zero, 0x4($t0)", rabbitizer.InstrCategory.CPU),
    TestEntry(0x21080008, None,                  "addi        $t0, $t0, 0x8", rabbitizer.InstrCategory.CPU),
    TestEntry(0x2129FFF8, None,                  "addi        $t1, $t1, -0x8", rabbitizer.InstrCategory.CPU),
    TestEntry(0x1520FFFB, None,                  "bnez        $t1, . + 4 + (-0x5 << 2)", rabbitizer.InstrCategory.CPU),
    TestEntry(0x00000000, None,                  "nop", rabbitizer.InstrCategory.CPU),
    TestEntry(0x3C0A8000, None,                  "lui         $t2, 0x8000", rabbitizer.InstrCategory.CPU),
    TestEntry(0x254A0494, None,                  "addiu       $t2, $t2, 0x494", rabbitizer.InstrCategory.CPU),
    TestEntry(0x3C1D8002, None,                  "lui         $sp, 0x8002", rabbitizer.InstrCategory.CPU),
    TestEntry(0x01400008, None,                  "jr          $t2", rabbitizer.InstrCategory.CPU),
    TestEntry(0x27BDF8C0, None,                  "addiu       $sp, $sp, -0x740", rabbitizer.InstrCategory.CPU),

    TestEntry(0x3C018001, None,                  "lui         $at, 0x8001", rabbitizer.InstrCategory.CPU),
    TestEntry(0x03E00008, None,                  "jr          $ra", rabbitizer.InstrCategory.CPU),
    TestEntry(0xAC24E190, None,                  "sw          $a0, -0x1E70($at)", rabbitizer.InstrCategory.CPU),

    TestEntry(0x3C018001, "%hi(D_8000E190)",     "lui         $at, %hi(D_8000E190)", rabbitizer.InstrCategory.CPU),
    TestEntry(0xAC24E190, "%lo(D_8000E190)",     "sw          $a0, %lo(D_8000E190)($at)", rabbitizer.InstrCategory.CPU),

    TestEntry(0x0C001F24, None,                  "jal         func_80007C90", rabbitizer.InstrCategory.CPU),
    TestEntry(0x0C001F24, "some_func",           "jal         some_func", rabbitizer.InstrCategory.CPU),

    TestEntry(0x8F99805C, None,                  "lw          $t9, -0x7FA4($gp)", rabbitizer.InstrCategory.CPU),
    TestEntry(0x8F99805C, "%call16(strcmp)",     "lw          $t9, %call16(strcmp)($gp)", rabbitizer.InstrCategory.CPU),

    TestEntry(0x8F858028, None,                  "lw          $a1, -0x7FD8($gp)", rabbitizer.InstrCategory.CPU),
    TestEntry(0x8F858028, "%got(STR_10007C90)",  "lw          $a1, %got(STR_10007C90)($gp)", rabbitizer.InstrCategory.CPU),

    TestEntry(0x00435022, None,                  "sub         $t2, $v0, $v1", rabbitizer.InstrCategory.CPU),
    TestEntry(0x00025022, None,                  "neg         $t2, $v0", rabbitizer.InstrCategory.CPU),

    TestEntry(0x00E41823, None,                  "subu        $v1, $a3, $a0", rabbitizer.InstrCategory.CPU),
    TestEntry(0x00041823, None,                  "negu        $v1, $a0", rabbitizer.InstrCategory.CPU),

    TestEntry(0x42000010, None,                  "rfe", rabbitizer.InstrCategory.CPU),

    # Invalid instructions
    TestEntry(0x44444444, None,                  ".word       0x44444444                   # cfc1        $a0, $8 # 00000444 <InstrIdType: CPU_COP1>", rabbitizer.InstrCategory.CPU),
    TestEntry(0x77777777, None,                  ".word       0x77777777                   # INVALID     $k1, $s7, 0x7777 # 00000000 <InstrIdType: CPU_NORMAL>", rabbitizer.InstrCategory.CPU),
    TestEntry(0xEEEEEEEE, None,                  ".word       0xEEEEEEEE                   # INVALID     $s7, $t6, -0x1112 # 00000000 <InstrIdType: CPU_NORMAL>", rabbitizer.InstrCategory.CPU),
]


TEST_ENTRIES_R3000GTE = [
    TestEntry(0x4A180001, None, "rtps", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A280030, None, "rtpt", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A680029, None, "dpcl", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A780010, None, "dpcs", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4AF8002A, None, "dpct", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A980011, None, "intpl", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4AC8041E, None, "ncs", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4AD80420, None, "nct", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4AE80413, None, "ncds", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4AF80416, None, "ncdt", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B08041B, None, "nccs", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B18043F, None, "ncct", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B280414, None, "cdp", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B38041C, None, "cc", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B400006, None, "nclip", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B58002D, None, "avsz3", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B68002E, None, "avsz4", rabbitizer.InstrCategory.R3000GTE),

    TestEntry(0x4A400012, None, "mvmva       0, 0, 0, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4AA00428, None, "sqr         0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B70000C, None, "op          0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B90003D, None, "gpf         0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4BA0003E, None, "gpl         0", rabbitizer.InstrCategory.R3000GTE),

    TestEntry(0x4A486012, None, "mvmva       1, 0, 0, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A48E012, None, "mvmva       1, 0, 1, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A496012, None, "mvmva       1, 0, 2, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A49E012, None, "mvmva       1, 0, 3, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A41E012, None, "mvmva       0, 0, 3, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A480012, None, "mvmva       1, 0, 0, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A488012, None, "mvmva       1, 0, 1, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A490012, None, "mvmva       1, 0, 2, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A498012, None, "mvmva       1, 0, 3, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A482012, None, "mvmva       1, 0, 0, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A48A012, None, "mvmva       1, 0, 1, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A492012, None, "mvmva       1, 0, 2, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A49A012, None, "mvmva       1, 0, 3, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4A6412, None, "mvmva       1, 1, 0, 3, 1", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4A6012, None, "mvmva       1, 1, 0, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4AE012, None, "mvmva       1, 1, 1, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4B6012, None, "mvmva       1, 1, 2, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4BE012, None, "mvmva       1, 1, 3, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4A0012, None, "mvmva       1, 1, 0, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4A8012, None, "mvmva       1, 1, 1, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4B0012, None, "mvmva       1, 1, 2, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4B8012, None, "mvmva       1, 1, 3, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4A2012, None, "mvmva       1, 1, 0, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4AA012, None, "mvmva       1, 1, 1, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4B2012, None, "mvmva       1, 1, 2, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4BA012, None, "mvmva       1, 1, 3, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4DA412, None, "mvmva       1, 2, 3, 1, 1", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4C6012, None, "mvmva       1, 2, 0, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4CE012, None, "mvmva       1, 2, 1, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4D6012, None, "mvmva       1, 2, 2, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4DE012, None, "mvmva       1, 2, 3, 3, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4C0012, None, "mvmva       1, 2, 0, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4C8012, None, "mvmva       1, 2, 1, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4D0012, None, "mvmva       1, 2, 2, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4D8012, None, "mvmva       1, 2, 3, 0, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4C2012, None, "mvmva       1, 2, 0, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4CA012, None, "mvmva       1, 2, 1, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4D2012, None, "mvmva       1, 2, 2, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4A4DA012, None, "mvmva       1, 2, 3, 1, 0", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4AA80428, None, "sqr         1", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B78000C, None, "op          1", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4B98003D, None, "gpf         1", rabbitizer.InstrCategory.R3000GTE),
    TestEntry(0x4BA8003E, None, "gpl         1", rabbitizer.InstrCategory.R3000GTE),
]

TEST_ENTRIES_R4000ALLEGREX = [
    TestEntry(0x00801017, None, "clo         $v0, $a0", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x00801016, None, "clz         $v0, $a0", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x00C7001C, None, "madd        $a2, $a3", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x00C7001D, None, "maddu       $a2, $a3", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x00C7002E, None, "msub        $a2, $a3", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x00C7002F, None, "msubu       $a2, $a3", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x0085102C, None, "max         $v0, $a0, $a1", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x0085102D, None, "min         $v0, $a0, $a1", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x0085100B, None, "movn        $v0, $a0, $a1", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x0085100A, None, "movz        $v0, $a0, $a1", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C822080, None, "ext         $v0, $a0, 2, 5", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C8221C0, None, "ext         $v0, $a0, 7, 5", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C823084, None, "ins         $v0, $a0, 2, 5", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C8259C4, None, "ins         $v0, $a0, 7, 5", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C041420, None, "seb         $v0, $a0", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C041620, None, "seh         $v0, $a0", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C041520, None, "bitrev      $v0, $a0", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x00241182, None, "rotr        $v0, $a0, 6", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x002414C2, None, "rotr        $v0, $a0, 19", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x00A41046, None, "rotrv       $v0, $a0, $a1", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C0410A0, None, "wsbh        $v0, $a0", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x7C0410E0, None, "wsbw        $v0, $a0", rabbitizer.InstrCategory.R4000ALLEGREX),

    TestEntry(0xBC840000, None, "cache       0x04, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC860000, None, "cache       0x06, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC880000, None, "cache       0x08, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC8A0000, None, "cache       0x0A, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC8B0000, None, "cache       0x0B, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC940000, None, "cache       0x14, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC960000, None, "cache       0x16, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC980000, None, "cache       0x18, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC990000, None, "cache       0x19, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC9A0000, None, "cache       0x1A, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC9B0000, None, "cache       0x1B, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC9C0000, None, "cache       0x1C, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC9E0000, None, "cache       0x1E, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC9F0000, None, "cache       0x1F, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),

    TestEntry(0x0000000F, None, "sync", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xC0820000, None, "ll          $v0, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xE0850000, None, "sc          $a1, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),

    TestEntry(0xBC800000, None, "cache       0x00, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC810000, None, "cache       0x01, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC820000, None, "cache       0x02, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC830000, None, "cache       0x03, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC850000, None, "cache       0x05, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC870000, None, "cache       0x07, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC890000, None, "cache       0x09, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC8C0000, None, "cache       0x0C, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC8D0000, None, "cache       0x0D, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC8E0000, None, "cache       0x0E, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC8F0000, None, "cache       0x0F, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC900000, None, "cache       0x10, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC910000, None, "cache       0x11, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC920000, None, "cache       0x12, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC930000, None, "cache       0x13, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC950000, None, "cache       0x15, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC970000, None, "cache       0x17, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0xBC9D0000, None, "cache       0x1D, 0x0($a0)", rabbitizer.InstrCategory.R4000ALLEGREX),

    TestEntry(0x70000000, None, "sleep", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x70020024, None, "mfie        $v0", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x70000024, None, "mfie        $zero", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x70000026, None, "mtie        $zero", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x70040026, None, "mtie        $a0", rabbitizer.InstrCategory.R4000ALLEGREX),

    # Allegrex removed 64 bits FPU instructions
    TestEntry(0x468020A1, None, ".word       0x468020A1                   # INVALID     $s4, $zero, 0x20A1 # 00000000 <InstrIdType: R4000ALLEGREX_COP1_FPUW>", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x46002088, None, ".word       0x46002088                   # INVALID     $s0, $zero, 0x2088 # 00000000 <InstrIdType: R4000ALLEGREX_COP1_FPUS>", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x46002089, None, ".word       0x46002089                   # INVALID     $s0, $zero, 0x2089 # 00000000 <InstrIdType: R4000ALLEGREX_COP1_FPUS>", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x4600208A, None, ".word       0x4600208A                   # INVALID     $s0, $zero, 0x208A # 00000000 <InstrIdType: R4000ALLEGREX_COP1_FPUS>", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x4600208B, None, ".word       0x4600208B                   # INVALID     $s0, $zero, 0x208B # 00000000 <InstrIdType: R4000ALLEGREX_COP1_FPUS>", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x460020A1, None, ".word       0x460020A1                   # INVALID     $s0, $zero, 0x20A1 # 00000000 <InstrIdType: R4000ALLEGREX_COP1_FPUS>", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x460020A5, None, ".word       0x460020A5                   # INVALID     $s0, $zero, 0x20A5 # 00000000 <InstrIdType: R4000ALLEGREX_COP1_FPUS>", rabbitizer.InstrCategory.R4000ALLEGREX),

    TestEntry(0x46002085, None, "abs.s       $f2, $f4", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x46042080, None, "add.s       $f2, $f4, $f4", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x45000008, None, "bc1f        . + 4 + (0x8 << 2)", rabbitizer.InstrCategory.R4000ALLEGREX),
    TestEntry(0x46100030, None, "c.f.s       $f0, $f16", rabbitizer.InstrCategory.R4000ALLEGREX),
]

TEST_ENTRIES_R5900 = [
    TestEntry(0x4A000038, None, "vcallms     0x0", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A004038, None, "vcallms     0x800", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A008038, None, "vcallms     0x1000", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A008838, None, "vcallms     0x1100", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A009038, None, "vcallms     0x1200", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A009838, None, "vcallms     0x1300", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A00a038, None, "vcallms     0x1400", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A07FFF8, None, "vcallms     0xFFF8", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A080038, None, "vcallms     0x10000", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A1F8038, None, "vcallms     0x3F000", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A1FFFB8, None, "vcallms     0x3FFF0", rabbitizer.InstrCategory.R5900),

    TestEntry(0x70001030, None, "pmfhl.lw    $v0", rabbitizer.InstrCategory.R5900),
    TestEntry(0x70001070, None, "pmfhl.uw    $v0", rabbitizer.InstrCategory.R5900),
    TestEntry(0x700010B0, None, "pmfhl.slw   $v0", rabbitizer.InstrCategory.R5900),
    TestEntry(0x700010F0, None, "pmfhl.lh    $v0", rabbitizer.InstrCategory.R5900),
    TestEntry(0x70001130, None, "pmfhl.sh    $v0", rabbitizer.InstrCategory.R5900),
    TestEntry(0x70000031, None, "pmthl.lw    $zero", rabbitizer.InstrCategory.R5900),

    TestEntry(0x4B020BFE, None, "vilwr.x     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A820BFE, None, "vilwr.y     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A420BFE, None, "vilwr.z     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A220BFE, None, "vilwr.w     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4B020BFF, None, "viswr.x     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A820BFF, None, "viswr.y     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A420BFF, None, "viswr.z     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),
    TestEntry(0x4A220BFF, None, "viswr.w     $vi2, ($vi1)", rabbitizer.InstrCategory.R5900),

    TestEntry(0x4A0307B2, None, "viaddi      $vi3, $vi0, -0x2", rabbitizer.InstrCategory.R5900),

    TestEntry(0x48500800, None, "cfc2.ni     $s0, $vi1", rabbitizer.InstrCategory.R5900),
    TestEntry(0x48500801, None, "cfc2.i      $s0, $vi1", rabbitizer.InstrCategory.R5900),
]

TEST_ENTRIES_R5900_TRUNC_CVT = [
    TestEntry(0x4600600D,  None, ".word       0x4600600D                   # trunc.w.s   $f0, $f12 # 00000000 <InstrIdType: CPU_COP1_FPUS>", rabbitizer.InstrCategory.R5900, gnuMode=True),
    TestEntry(0x46006024,  None, ".word       0x46006024                   # cvt.w.s     $f0, $f12 # 00000000 <InstrIdType: CPU_COP1_FPUS>", rabbitizer.InstrCategory.R5900, gnuMode=True),
    TestEntry(0x4600600D, None, "trunc.w.s   $f0, $f12", rabbitizer.InstrCategory.R5900, gnuMode=False),
    TestEntry(0x46006024, None, "cvt.w.s     $f0, $f12", rabbitizer.InstrCategory.R5900, gnuMode=False),
]


ALL_TEST_ENTRIES = [
    ("plain", TEST_ENTRIES_PLAIN),
    ("r3000gte", TEST_ENTRIES_R3000GTE),
    ("r4000allegrex", TEST_ENTRIES_R4000ALLEGREX),
    ("r5900", TEST_ENTRIES_R5900),
    ("r5900 trunc cvt", TEST_ENTRIES_R5900_TRUNC_CVT),
]

# uv run --no-config --no-build --no-sync tests/python/disasm_test.py
def test_func():
    print("Running Python", sys.version)
    total_errors = 0
    for test_name, entries in ALL_TEST_ENTRIES:
        print()
        print("Testing", test_name)
        errorCount = 0
        for entry in entries:
            rabbitizer.config.toolchainTweaks_gnuMode = entry.gnuMode
            instr = rabbitizer.Instruction(entry.word, category=entry.category)
            disassembly = instr.disassemble(entry.immOverride)
            if disassembly != entry.expectedStr:
                word = "0x" + hex(entry.word)[2:].upper()
                print("Error on word", word, ". Expected", entry.expectedStr, "got", disassembly)
                errorCount += 1
                total_errors += 1
        print()
        print("Finish testing", test_name)

        test_entries_len = len(entries)
        print(errorCount, "errors out of", test_entries_len, "entries.", ((test_entries_len - errorCount) / test_entries_len * 100.0), "% correct.")

        print()

    print("Total errors:", total_errors)
    exit(total_errors != 0)


if __name__ == "__main__":
    test_func()
