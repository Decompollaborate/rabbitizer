/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26---------------------------------------------------0
    |  opcode   |                                                   |
    ------6----------------------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | *1    | *2    | J     | JAL   | BEQ   | BNE   | BLEZ  | BGTZ  |
001 | ADDI  | ADDIU | SLTI  | SLTIU | ANDI  | ORI   | XORI  | LUI   |
010 | *3    | *4    | *5    | ---   | BEQL  | BNEL  | BLEZL | BGTZL |
011 | *6    | *7    | ---   | *8    | *9    | ---   | ---   | *10   |
100 | LB    | LH    | LWL   | LW    | LBU   | LHU   | LWR   | ---   |
101 | SB    | SH    | SWL   | SW    | ---   | ---   | SWR   | CACHE |
110 | LL    | LWC1  | LV.S  | ---   | *11   | ---   | LV.Q  | *12   |
111 | SC    | SWC1  | SV.S  | ---   | *13   | *14   | SV.Q  | *15   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = SPECIAL, see SPECIAL list
     *2 = REGIMM, see REGIMM list
     *3 = COP0
     *4 = COP1
     *5 = COP2
     *6 = VFPU0
     *7 = VFPU1
     *8 = VFPU3
     *9 = SPECIAL2
     *10 = SPECIAL3
     *11 = VFPU4
     *12 = VFPU5
     *13 = VFPU6 (Matrix operations)
     *14 = QUADLR
     *15 = VFPU7
*/

    // The other instructions are implemented using the main CORE table

    // OP vt, offset_14(base)

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_NORMAL, 0x32, lv_s, lv.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vt_imm, Operand::r4000allegrex_offset14_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Load Single word to Vfpu

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_NORMAL, 0x3A, sv_s, sv.s, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_s_vt_imm, Operand::r4000allegrex_offset14_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Store Single word to Vfpu

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_NORMAL, 0x36, lv_q, lv.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vt_imm, Operand::r4000allegrex_offset14_base),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Load Single word to Vfpu

    // OP vt, offset_14(base), maybe_wb

    RAB_DEF_OPCODE_EXT_ALTNAME(
        r4000allegrex, R4000ALLEGREX_NORMAL, 0x3E, sv_q, sv.q, R4000ALLEGREX,
        operands: Operand::arr2(Operand::r4000allegrex_q_vt_imm, Operand::r4000allegrex_offset14_base_maybe_wb),
        instr_type: InstrType::I,
        reads_rs: true
    ) // Store Single word to Vfpu

    RAB_DEF_OPCODE_REDIRECT(0x00, r4000allegrex, special)
    RAB_DEF_OPCODE_REDIRECT(0x01, r4000allegrex, regimm)

    RAB_DEF_OPCODE_REDIRECT(0x1C, r4000allegrex, special2)
    RAB_DEF_OPCODE_REDIRECT(0x1F, r4000allegrex, special3)

    RAB_DEF_OPCODE_REDIRECT(0x10, r4000allegrex, coprocessor0)
    RAB_DEF_OPCODE_REDIRECT(0x11, r4000allegrex, coprocessor1)
    RAB_DEF_OPCODE_REDIRECT(0x12, r4000allegrex, coprocessor2)

    RAB_DEF_OPCODE_REDIRECT(0x18, r4000allegrex, vfpu0)
    RAB_DEF_OPCODE_REDIRECT(0x19, r4000allegrex, vfpu1)
    RAB_DEF_OPCODE_REDIRECT(0x1B, r4000allegrex, vfpu3)
    RAB_DEF_OPCODE_REDIRECT(0x34, r4000allegrex, vfpu4)
    RAB_DEF_OPCODE_REDIRECT(0x37, r4000allegrex, vfpu5)
    RAB_DEF_OPCODE_REDIRECT(0x3C, r4000allegrex, vfpu6)

    RAB_DEF_OPCODE_REDIRECT(0x3D, r4000allegrex, quadlr)

    RAB_DEF_OPCODE_REDIRECT(0x3F, r4000allegrex, vfpu7)

    RAB_DEF_OPCODE_REDIRECT(_, none, normal)
