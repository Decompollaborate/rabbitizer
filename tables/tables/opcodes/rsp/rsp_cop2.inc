/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21-----------------------------------------0
    |  = COP2   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC2  | ---   | CFC2  | ---   | MTC2  | ---   | CTC2  | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | *1    | *1    | *1    | *1    | *1    | *1    | *1    | *1    |
 11 | *1    | *1    | *1    | *1    | *1    | *1    | *1    | *1    |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
       *1 = COP2 VU
*/

    // OP rt, vs[index]
    RAB_DEF_OPCODE_EXT(
        rsp, RSP_COP2, 0x00, mfc2, RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_vs_index),
        modifies_rt: true
    )
    RAB_DEF_OPCODE_EXT(
        rsp, RSP_COP2, 0x04, mtc2, RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_vs_index),
        reads_rt: true
    )

    // OP rt, cop2cd
    RAB_DEF_OPCODE_EXT(
        rsp, RSP_COP2, 0x02, cfc2, RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
        modifies_rt: true
    )
    RAB_DEF_OPCODE_EXT(
        rsp, RSP_COP2, 0x06, ctc2, RSP,
        operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
        reads_rt: true
    )

    RAB_DEF_OPCODE_REDIRECT(0x10..=0x1F, rsp, coprocessor2_vu)
