/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31----------26-25--------21 -----------------------------------------5----------0
    |   = COP1    |   = D     |                                         | function  |
    -------6------------5-----------------------------------------------------6------
    |---000---|---001---|---010---|---011---|---100---|---101---|---110---|---111---| lo
000 | ADD.D   | SUB.D   | MUL.D   | DIV.D   | SQRT.D  | ABS.D   | MOV.D   | NEG.D   |
001 |ROUND.L.D|TRUNC.L.D| CEIL.L.D|FLOOR.L.D|ROUND.W.D|TRUNC.W.D| CEIL.W.D|FLOOR.W.D|
010 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
011 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
100 | CVT.S.D | ---     | ---     | ---     | CVT.W.D | CVT.L.D | ---     | ---     |
101 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
110 | C.F.D   | C.UN.D  | C.EQ.D  | C.UEQ.D | C.OLT.D | C.ULT.D | C.OLE.D | C.ULE.D |
111 | C.DF.D  | C.NGLE.D| C.SEQ.D | C.NGL.D | C.LT.D  | C.NGE.D | C.LE.D  | C.NGT.D |
 hi |---------|---------|---------|---------|---------|---------|---------|---------|
*/

    // OP fd, fs, ft
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x00, add_d, add.d,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Add
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x01, sub_d, sub.d,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Sub
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x02, mul_d, mul.d,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Multiply
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x03, div_d, div.d,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Divide

    // OP fd, fs
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x04, sqrt_d, sqrt.d, MIPS_II,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Square Root
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x05, abs_d, abs.d,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Absolute Value
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x06, mov_d, mov.d,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Move
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x07, neg_d, neg.d,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Negate

    RAB_DEF_OPCODE_ALTNAME(
        core, 0x08, round_l_d, round.l.d, MIPS_III,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Round to Long Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x09, trunc_l_d, trunc.l.d, MIPS_III,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Truncate to Long Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0A, ceil_l_d, ceil.l.d, MIPS_III,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Ceiling Convert to Long Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0B, floor_l_d, floor.l.d, MIPS_III,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Floor Convert to Long Fixed-Point

    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0C, round_w_d, round.w.d, MIPS_II,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Round to Word Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0D, trunc_w_d, trunc.w.d, MIPS_II,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Truncate to Word Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0E, ceil_w_d, ceil.w.d, MIPS_II,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Ceiling Convert to Word Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0F, floor_w_d, floor.w.d, MIPS_II,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Floor Convert to Word Fixed-Point

    // OP fd, fs
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x20, cvt_s_d, cvt.s.d,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x24, cvt_w_d, cvt.w.d,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x25, cvt_l_d, cvt.l.d, MIPS_III,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true
    )

    // OP fs, ft
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x30, c_f_d, c.f.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x31, c_un_d, c.un.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x32, c_eq_d, c.eq.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x33, c_ueq_d, c.ueq.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x34, c_olt_d, c.olt.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x35, c_ult_d, c.ult.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x36, c_ole_d, c.ole.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x37, c_ule_d, c.ule.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )

    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x38, c_df_d, c.df.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x39, c_ngle_d, c.ngle.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x3A, c_seq_d, c.seq.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x3B, c_ngl_d, c.ngl.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x3C, c_lt_d, c.lt.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x3D, c_nge_d, c.nge.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x3E, c_le_d, c.le.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_MIPS1_ALTNAME(
        core, 0x3F, c_ngt_d, c.ngt.d,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
