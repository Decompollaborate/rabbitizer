/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP vt[elementlow], offset(vs)
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x00, lbv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x01, lsv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x02, llv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x03, ldv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x04, lqv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x05, lrv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x06, lpv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x07, luv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x08, lhv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x09, lfv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0B, ltv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
