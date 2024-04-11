/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <string.h>
#include <stdlib.h>
#include <assert.h>

#include "expected_disasm_utils.h"

#define TEST_ENTRY_C(word, imm, expected) TEST_ENTRY(RABBITIZER_INSTRCAT_R4000ALLEGREX, word, imm, expected,)

const TestEntry test_entries[] = {
    TEST_ENTRY_C(0x00801017, NULL, "clo         $v0, $a0"),
    TEST_ENTRY_C(0x00801016, NULL, "clz         $v0, $a0"),
    TEST_ENTRY_C(0x00C7001C, NULL, "madd        $a2, $a3"),
    TEST_ENTRY_C(0x00C7001D, NULL, "maddu       $a2, $a3"),
    TEST_ENTRY_C(0x00C7002E, NULL, "msub        $a2, $a3"),
    TEST_ENTRY_C(0x00C7002F, NULL, "msubu       $a2, $a3"),
    TEST_ENTRY_C(0x0085102C, NULL, "max         $v0, $a0, $a1"),
    TEST_ENTRY_C(0x0085102D, NULL, "min         $v0, $a0, $a1"),
    TEST_ENTRY_C(0x0085100B, NULL, "movn        $v0, $a0, $a1"),
    TEST_ENTRY_C(0x0085100A, NULL, "movz        $v0, $a0, $a1"),
    TEST_ENTRY_C(0x7C822080, NULL, "ext         $v0, $a0, 2, 5"),
    TEST_ENTRY_C(0x7C8221C0, NULL, "ext         $v0, $a0, 7, 5"),
    TEST_ENTRY_C(0x7C823084, NULL, "ins         $v0, $a0, 2, 5"),
    TEST_ENTRY_C(0x7C8259C4, NULL, "ins         $v0, $a0, 7, 5"),
    TEST_ENTRY_C(0x7C041420, NULL, "seb         $v0, $a0"),
    TEST_ENTRY_C(0x7C041620, NULL, "seh         $v0, $a0"),
    TEST_ENTRY_C(0x7C041520, NULL, "bitrev      $v0, $a0"),
    TEST_ENTRY_C(0x00241182, NULL, "rotr        $v0, $a0, 6"),
    TEST_ENTRY_C(0x002414C2, NULL, "rotr        $v0, $a0, 19"),
    TEST_ENTRY_C(0x00A41046, NULL, "rotrv       $v0, $a0, $a1"),
    TEST_ENTRY_C(0x7C0410A0, NULL, "wsbh        $v0, $a0"),
    TEST_ENTRY_C(0x7C0410E0, NULL, "wsbw        $v0, $a0"),

#if 0
    TEST_ENTRY_C(0xBC840000, NULL, "cache       IXIN, 0x0($a0)"),
    TEST_ENTRY_C(0xBC860000, NULL, "cache       IXUN, 0x0($a0)"),
    TEST_ENTRY_C(0xBC880000, NULL, "cache       IHIN, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8A0000, NULL, "cache       IF, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8B0000, NULL, "cache       IFL, 0x0($a0)"),
    TEST_ENTRY_C(0xBC940000, NULL, "cache       DXWBIN, 0x0($a0)"),
    TEST_ENTRY_C(0xBC960000, NULL, "cache       DXUN, 0x0($a0)"),
    TEST_ENTRY_C(0xBC980000, NULL, "cache       DCDEX, 0x0($a0)"),
    TEST_ENTRY_C(0xBC990000, NULL, "cache       DHIN, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9A0000, NULL, "cache       DHWB, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9B0000, NULL, "cache       DHWBIN, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9C0000, NULL, "cache       DCDEXL, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9E0000, NULL, "cache       DF, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9F0000, NULL, "cache       DFL, 0x0($a0)"),
#else
    TEST_ENTRY_C(0xBC840000, NULL, "cache       0x04, 0x0($a0)"),
    TEST_ENTRY_C(0xBC860000, NULL, "cache       0x06, 0x0($a0)"),
    TEST_ENTRY_C(0xBC880000, NULL, "cache       0x08, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8A0000, NULL, "cache       0x0A, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8B0000, NULL, "cache       0x0B, 0x0($a0)"),
    TEST_ENTRY_C(0xBC940000, NULL, "cache       0x14, 0x0($a0)"),
    TEST_ENTRY_C(0xBC960000, NULL, "cache       0x16, 0x0($a0)"),
    TEST_ENTRY_C(0xBC980000, NULL, "cache       0x18, 0x0($a0)"),
    TEST_ENTRY_C(0xBC990000, NULL, "cache       0x19, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9A0000, NULL, "cache       0x1A, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9B0000, NULL, "cache       0x1B, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9C0000, NULL, "cache       0x1C, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9E0000, NULL, "cache       0x1E, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9F0000, NULL, "cache       0x1F, 0x0($a0)"),
#endif

    TEST_ENTRY_C(0x0000000F, NULL, "sync"),
    TEST_ENTRY_C(0xC0820000, NULL, "ll          $v0, 0x0($a0)"),
    TEST_ENTRY_C(0xE0850000, NULL, "sc          $a1, 0x0($a0)"),

    TEST_ENTRY_C(0xBC800000, NULL, "cache       0x00, 0x0($a0)"),
    TEST_ENTRY_C(0xBC810000, NULL, "cache       0x01, 0x0($a0)"),
    TEST_ENTRY_C(0xBC820000, NULL, "cache       0x02, 0x0($a0)"),
    TEST_ENTRY_C(0xBC830000, NULL, "cache       0x03, 0x0($a0)"),
    TEST_ENTRY_C(0xBC850000, NULL, "cache       0x05, 0x0($a0)"),
    TEST_ENTRY_C(0xBC870000, NULL, "cache       0x07, 0x0($a0)"),
    TEST_ENTRY_C(0xBC890000, NULL, "cache       0x09, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8C0000, NULL, "cache       0x0C, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8D0000, NULL, "cache       0x0D, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8E0000, NULL, "cache       0x0E, 0x0($a0)"),
    TEST_ENTRY_C(0xBC8F0000, NULL, "cache       0x0F, 0x0($a0)"),
    TEST_ENTRY_C(0xBC900000, NULL, "cache       0x10, 0x0($a0)"),
    TEST_ENTRY_C(0xBC910000, NULL, "cache       0x11, 0x0($a0)"),
    TEST_ENTRY_C(0xBC920000, NULL, "cache       0x12, 0x0($a0)"),
    TEST_ENTRY_C(0xBC930000, NULL, "cache       0x13, 0x0($a0)"),
    TEST_ENTRY_C(0xBC950000, NULL, "cache       0x15, 0x0($a0)"),
    TEST_ENTRY_C(0xBC970000, NULL, "cache       0x17, 0x0($a0)"),
    TEST_ENTRY_C(0xBC9D0000, NULL, "cache       0x1D, 0x0($a0)"),

    TEST_ENTRY_C(0x70000000, NULL, "sleep"),
    TEST_ENTRY_C(0x70020024, NULL, "mfie        $v0"),
    TEST_ENTRY_C(0x70000024, NULL, "mfie        $zero"),
    TEST_ENTRY_C(0x70000026, NULL, "mtie        $zero"),
    TEST_ENTRY_C(0x70040026, NULL, "mtie        $a0"),
};

size_t test_entries_len = ARRAY_COUNT(test_entries);
