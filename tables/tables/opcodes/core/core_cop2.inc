/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 ----------------------------------------0
    |  = COP0   |   fmt   |                                         |
    ------6----------5-----------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | MFC2  | ---   | CFC2  | ---   | MTC2  | ---   | CTC2  | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP rt, cop2cd
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP2, 0x00, mfc2,
        operands: Operand::arr2(Operand::core_rt, Operand::core_cop2d),
        modifies_rt: true
    )
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP2, 0x04, mtc2,
        operands: Operand::arr2(Operand::core_rt, Operand::core_cop2d),
        reads_rt: true
    )

    // OP rt, cop2cd
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP2, 0x02, cfc2,
        operands: Operand::arr2(Operand::core_rt, Operand::core_cop2cd),
        modifies_rt: true
    )
    RAB_DEF_OPCODE_MIPS1(
        core, CORE_COP2, 0x06, ctc2,
        operands: Operand::arr2(Operand::core_rt, Operand::core_cop2cd),
        reads_rt: true
    )
