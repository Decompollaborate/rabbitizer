/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_CPU, word, imm, expected,)

const TestEntry test_entries[] = {
    TEST_ENTRY_C(0x3C088001, NULL,                  "lui         $t0, 0x8001"),
    TEST_ENTRY_C(0x25080E60, NULL,                  "addiu       $t0, $t0, 0xE60"),
    TEST_ENTRY_C(0x3C090002, NULL,                  "lui         $t1, 0x2"),
    TEST_ENTRY_C(0x25298DE0, NULL,                  "addiu       $t1, $t1, -0x7220"),
    TEST_ENTRY_C(0xAD000000, NULL,                  "sw          $zero, 0x0($t0)"),
    TEST_ENTRY_C(0xAD000004, NULL,                  "sw          $zero, 0x4($t0)"),
    TEST_ENTRY_C(0x21080008, NULL,                  "addi        $t0, $t0, 0x8"),
    TEST_ENTRY_C(0x2129FFF8, NULL,                  "addi        $t1, $t1, -0x8"),
    TEST_ENTRY_C(0x1520FFFB, NULL,                  "bnez        $t1, . + 4 + (-0x5 << 2)"),
    TEST_ENTRY_C(0x00000000, NULL,                  "nop"),
    TEST_ENTRY_C(0x3C0A8000, NULL,                  "lui         $t2, 0x8000"),
    TEST_ENTRY_C(0x254A0494, NULL,                  "addiu       $t2, $t2, 0x494"),
    TEST_ENTRY_C(0x3C1D8002, NULL,                  "lui         $sp, 0x8002"),
    TEST_ENTRY_C(0x01400008, NULL,                  "jr          $t2"),
    TEST_ENTRY_C(0x27BDF8C0, NULL,                  "addiu       $sp, $sp, -0x740"),

    TEST_ENTRY_C(0x3C018001, NULL,                  "lui         $at, 0x8001"),
    TEST_ENTRY_C(0x03E00008, NULL,                  "jr          $ra"),
    TEST_ENTRY_C(0xAC24E190, NULL,                  "sw          $a0, -0x1E70($at)"),

    TEST_ENTRY_C(0x3C018001, "%hi(D_8000E190)",     "lui         $at, %hi(D_8000E190)"),
    TEST_ENTRY_C(0xAC24E190, "%lo(D_8000E190)",     "sw          $a0, %lo(D_8000E190)($at)"),

    TEST_ENTRY_C(0x0C001F24, NULL,                  "jal         func_80007C90"),
    TEST_ENTRY_C(0x0C001F24, "some_func",           "jal         some_func"),

    TEST_ENTRY_C(0x8F99805C, NULL,                  "lw          $t9, -0x7FA4($gp)"),
    TEST_ENTRY_C(0x8F99805C, "%call16(strcmp)",     "lw          $t9, %call16(strcmp)($gp)"),

    TEST_ENTRY_C(0x8F858028, NULL,                  "lw          $a1, -0x7FD8($gp)"),
    TEST_ENTRY_C(0x8F858028, "%got(STR_10007C90)",  "lw          $a1, %got(STR_10007C90)($gp)"),

    TEST_ENTRY_C(0x00435022, NULL,                  "sub         $t2, $v0, $v1"),
    TEST_ENTRY_C(0x00025022, NULL,                  "neg         $t2, $v0"),

    TEST_ENTRY_C(0x00E41823, NULL,                  "subu        $v1, $a3, $a0"),
    TEST_ENTRY_C(0x00041823, NULL,                  "negu        $v1, $a0"),

    TEST_ENTRY_C(0x42000010, NULL,                  "rfe"),

    // Invalid instructions
    TEST_ENTRY_C(0x44444444, NULL,                  ".word       0x44444444                   # cfc1        $a0, $8 # 00000444 <InstrIdType: CPU_COP1>"),
    TEST_ENTRY_C(0x77777777, NULL,                  ".word       0x77777777                   # INVALID     $k1, $s7, 0x7777 # 00000000 <InstrIdType: CPU_NORMAL>"),
    TEST_ENTRY_C(0xEEEEEEEE, NULL,                  ".word       0xEEEEEEEE                   # INVALID     $s7, $t6, -0x1112 # 00000000 <InstrIdType: CPU_NORMAL>"),
};

size_t test_entries_len = ARRAY_COUNT(test_entries);
