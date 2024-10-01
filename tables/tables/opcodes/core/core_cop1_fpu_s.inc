/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31----------26-25--------21 -----------------------------------------5----------0
    |   = COP1    |   = S     |                                         | function  |
    -------6------------5-----------------------------------------------------6------
    |---000---|---001---|---010---|---011---|---100---|---101---|---110---|---111---| lo
000 | ADD.S   | SUB.S   | MUL.S   | DIV.S   | SQRT.S  | ABS.S   | MOV.S   | NEG.S   |
001 |ROUND.L.S|TRUNC.L.S| CEIL.L.S|FLOOR.L.S|ROUND.W.S|TRUNC.W.S| CEIL.W.S|FLOOR.W.S|
010 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
011 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
100 | ---     | CVT.D.S | ---     | ---     | CVT.W.S | CVT.L.S | ---     | ---     |
101 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
110 | C.F.S   | C.UN.S  | C.EQ.S  | C.UEQ.S | C.OLT.S | C.ULT.S | C.OLE.S | C.ULE.S |
111 | C.SF.S  | C.NGLE.S| C.SEQ.S | C.NGL.S | C.LT.S  | C.NGE.S | C.LE.S  | C.NGT.S |
 hi |---------|---------|---------|---------|---------|---------|---------|---------|
*/

    // OP fd, fs, ft
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x00, add_s, add.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Add
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x01, sub_s, sub.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Sub
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x02, mul_s, mul.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Multiply
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x03, div_s, div.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true
    ) // Floating-Point Divide

    // OP fd, fs
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x04, sqrt_s, sqrt.s, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Square Root
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x05, abs_s, abs.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Absolute Value
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x06, mov_s, mov.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Move
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x07, neg_s, neg.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Negate

    RAB_DEF_OPCODE_ALTNAME(
        core, 0x08, round_l_s, round.l.s, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Round to Long Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x09, trunc_l_s, trunc.l.s, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Truncate to Long Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0A, ceil_l_s, ceil.l.s, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Ceiling Convert to Long Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0B, floor_l_s, floor.l.s, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Floor Convert to Long Fixed-Point

    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0C, round_w_s, round.w.s, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Round to Word Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0D, trunc_w_s, trunc.w.s, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Truncate to Word Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0E, ceil_w_s, ceil.w.s, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Ceiling Convert to Word Fixed-Point
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x0F, floor_w_s, floor.w.s, IsaVersion::MIPS_II, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    ) // Floating-Point Floor Convert to Word Fixed-Point

    // OP fd, fs
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x21, cvt_d_s, cvt.d.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x24, cvt_w_s, cvt.w.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x25, cvt_l_s, cvt.l.s, IsaVersion::MIPS_III, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    )

    // OP fs, ft
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x30, c_f_s, c.f.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x31, c_un_s, c.un.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x32, c_eq_s, c.eq.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x33, c_ueq_s, c.ueq.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x34, c_olt_s, c.olt.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x35, c_ult_s, c.ult.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x36, c_ole_s, c.ole.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x37, c_ule_s, c.ule.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )

    RAB_DEF_OPCODE_ALTNAME(
        core, 0x38, c_sf_s, c.sf.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x39, c_ngle_s, c.ngle.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x3A, c_seq_s, c.seq.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x3B, c_ngl_s, c.ngl.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x3C, c_lt_s, c.lt.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x3D, c_nge_s, c.nge.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x3E, c_le_s, c.le.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x3F, c_ngt_s, c.ngt.s, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true
    )
