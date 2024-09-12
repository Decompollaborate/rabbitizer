/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP vt[elementlow], offset(vs)
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x00, sbv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x01, ssv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x02, slv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x03, sdv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x04, sqv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x05, srv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x06, spv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x07, suv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x08, shv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x09, sfv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0B, stv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )

    RABBITIZER_DEF_INSTR_ID(
        rsp, -0x07, swv,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
