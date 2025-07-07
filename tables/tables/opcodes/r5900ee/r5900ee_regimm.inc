/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26----------20-------16------------------------------0
    | = REGIMM  |          |   rt    |                              |
    ------6---------------------5------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | BLTZ  | BGEZ  | BLTZL | BGEZL | ---   | ---   | ---   | ---   |
 01 | TGEI  | TGEIU | TLTI  | TLTIU | TEQI  | ---   | TNEI  | ---   |
 10 | BLTZAL| BGEZAL|BLTZALL|BGEZALL| ---   | ---   | ---   | ---   |
 11 | MTSAB | MTSAH | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // The other instructions are implemented using the main CORE table

    // OP rs, IMM
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x18, mtsab, R5900EE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        reads_rs: true
    )
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x19, mtsah, R5900EE,
        operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
        instr_type: InstrType::REGIMM,
        reads_rs: true
    )

    RAB_DEF_OPCODE_REDIRECT(_, none, regimm)
