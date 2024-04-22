/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
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

    // The other instructions are implemented using the main CPU table

    // OP vt, offset_14(base)

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x32, lv_s, lv.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .readsRs=true
    ) // Load Single word to Vfpu

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3A, sv_s, sv.s,
        .operands={RAB_OPERAND_r4000allegrex_s_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .modifiesRs=true
    ) // Store Single word to Vfpu

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x36, lv_q, lv.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .readsRs=true
    ) // Load Single word to Vfpu

    // OP vt, offset_14(base), maybe_wb

    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        r4000allegrex, 0x3E, sv_q, sv.q,
        .operands={RAB_OPERAND_r4000allegrex_q_vt_imm, RAB_OPERAND_r4000allegrex_offset14_base_maybe_wb},
        .instrType=RABBITIZER_INSTR_TYPE_I,
        .modifiesRs=true
    ) // Store Single word to Vfpu
