/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
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


    RAB_DEF_OPCODE(
        r5900, 0x1E, lq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    ) // Load Quadword
    RAB_DEF_OPCODE(
        r5900, 0x1F, sq, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    ) // Store Quadword

    RAB_DEF_OPCODE(
        r5900, 0x36, lqc2, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vft, Operand::core_immediate_base),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true
    )

    RAB_DEF_OPCODE(
        r5900, 0x3E, sqc2, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr2(Operand::r5900_vft, Operand::core_immediate_base),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true
    )
