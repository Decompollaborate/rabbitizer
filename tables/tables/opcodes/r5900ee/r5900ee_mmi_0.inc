/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------10--------6-5--------0
    |          |                              |function  |   MMI0   |
    ------6----------------------------------------------------6-----
    |---00--|---01--|---10--|---11--| lo
000 |PADDW  | PSUBW | PCGTW | PMAXW |
001 |PADDH  | PSUBH | PCGTH | PMAXH |
010 |PADDB  | PSUBB | PCGTB | ---   |
011 | ---   | ---   | ---   | ---   |
100 |PADDSW |PSUBSW |PEXTLW | PPACW |
101 |PADDSH |PSUBSH |PEXTLH | PPACH |
110 |PADDSB |PSUBSB |PEXTLB | PPACB |
111 | ---   | ---   | PEXT5 | PPAC5 |
 hi |-------|-------|-------|-------|
*/

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x00, paddw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x01, psubw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x02, pcgtw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for Greater Than Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x03, pmaxw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MAXimum Word

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x04, paddh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x05, psubh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x06, pcgth, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for Greater Than Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x07, pmaxh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel MAXimum Halfword

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x08, paddb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD Byte
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x09, psubb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract Byte
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x0A, pcgtb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel Compare for Greater Than Byte

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x10, paddsw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD with Signed saturation Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x11, psubsw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract with Signed saturation Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x12, pextlw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Lower from Word
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x13, ppacw, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel PACk to Word

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x14, paddsh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD with Signed saturation Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x15, psubsh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract with Signed saturation Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x16, pextlh, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Lower from Halfword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x17, ppach, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel PACk to Halfword

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x18, paddsb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel ADD with Signed saturation Bytes
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x19, psubsb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel SUBtract with Signed saturation Bytes
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1A, pextlb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel EXTend Lower from Bytes
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1B, ppacb, R5900EE,
        operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true
    ) // Parallel PACk to Bytes

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1E, pext5, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel EXTend from 5-bits
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1F, ppac5, R5900EE,
        operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
        modifies_rd: true,
        reads_rt: true
    ) // Parallel PACk to 5-bits
