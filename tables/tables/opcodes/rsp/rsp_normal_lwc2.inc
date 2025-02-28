/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26-------------------15-------11---------------------0
    |   LWC2    |                   |   rd    |                     |
    ------6------------------------------5---------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | lbv   | lsv   | llv   | ldv   | lqv   | lrv   | lpv   | luv   |
 01 | lhv   | lfv   |!lwv   | ltv   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP vt[elementlow], offset(vs)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x00, lbv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load byte from memory into vt[e]. (e : 0-15)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x01, lsv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load short from memory into vt[e]. (e : 0-14 by 2)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x02, llv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load word from memory into vt[e]. (e : 0-12 by 4)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x03, ldv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load double word from memory into vt[e]. (e : 0,8)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x04, lqv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load quad word from memory into vt[e]. (e : 0)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x05, lrv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load rest
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x06, lpv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load packed
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x07, luv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load unpack
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x08, lhv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load half
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x09, lfv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load fourth
    RAB_DEF_OPCODE_EXTENSION_GATED(
        rsp, 0x0A, lwv, RSP, RspViceMsp,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load wrap
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x0B, ltv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // load transpose
