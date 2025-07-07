/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26---------------------------------------------------0
    |  opcode   |                                                   |
    ------6----------------------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | *1    | *2    | J     | JAL   | BEQ   | BNE   | BLEZ  | BGTZ  |
001 | ADDI  | ADDIU | SLTI  | SLTIU | ANDI  | ORI   | XORI  | LUI   |
010 | *3    | *4    |  *5   | ---   | BEQL  | BNEL  | BLEZL | BGTZL |
011 | DADDI |DADDIU | LDL   | LDR   |  *6   | ---   | LQ    | SQ    |
100 | LB    | LH    | LWL   | LW    | LBU   | LHU   | LWR   | LWU   |
101 | SB    | SH    | SWL   | SW    | SDL   | SDR   | SWR   | CACHE |
110 | ---   | LWC1  | ---   | PREF  | ---   | ---   | LQC2  | LD    |
111 | ---   | SWC1  | ---   | ---   | ---   | ---   | SQC2  | SD    |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL, see SPECIAL list    *2 = REGIMM, see REGIMM list
     *3 = COP0                         *4 = COP1
     *5 = COP2                         *6 = MMI table
*/

    // The other instructions are implemented using the main CORE table


    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1E, lq, R5900EE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_rs),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::QUADWORD)
    ) // Load Quadword
    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x1F, sq, R5900EE,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_rs),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::QUADWORD)
    ) // Store Quadword

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x36, lqc2, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vft, Operand::core_immediate_rs),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: Some(AccessType::QUADWORD)
    )

    RAB_DEF_OPCODE_EXT(
        r5900ee, 0x3E, sqc2, R5900EE,
        operands: Operand::arr2(Operand::r5900ee_vft, Operand::core_immediate_rs),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: Some(AccessType::QUADWORD)
    )

    RAB_DEF_OPCODE_REDIRECT(0x00, r5900ee, special)
    RAB_DEF_OPCODE_REDIRECT(0x01, r5900ee, regimm)
    RAB_DEF_OPCODE_REDIRECT(0x10, r5900ee, coprocessor0)
    RAB_DEF_OPCODE_REDIRECT(0x11, r5900ee, coprocessor1)
    RAB_DEF_OPCODE_REDIRECT(0x12, r5900ee, coprocessor2)
    RAB_DEF_OPCODE_REDIRECT(0x1C, r5900ee, mmi)
    RAB_DEF_OPCODE_REDIRECT(_, none, normal)
