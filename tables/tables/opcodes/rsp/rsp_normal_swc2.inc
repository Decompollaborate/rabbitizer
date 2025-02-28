/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26-------------------15-------11---------------------0
    |   SWC2    |                   |   rd    |                     |
    ------6------------------------------5---------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | sbv   | ssv   | slv   | sdv   | sqv   | srv   | spv   | suv   |
 01 | shv   | sfv   | swv   | stv   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP vt[elementlow], offset(vs)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x00, sbv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store byte in vt[e] to memory. (e : 0-15)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x01, ssv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store short in vt[e] to memory. (e : 0-14 by 2)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x02, slv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store word in vt[e] to memory. (e : 0-12 by 4)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x03, sdv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store double word in vt[e] to memory. (e : 0,8)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x04, sqv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store quad word in vt[e] to memory. (e : 0)
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x05, srv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store rest
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x06, spv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store packed
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x07, suv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store unpack
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x08, shv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store half
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x09, sfv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store fourth
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x0A, swv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store wrap
    RAB_DEF_OPCODE_EXTENSION(
        rsp, 0x0B, stv, RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    ) // store transpose
