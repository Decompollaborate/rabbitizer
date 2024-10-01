/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31----------26-25--------21 -----------------------------------------5----------0
    |   = COP1    |   = W     |                                         | function  |
    -------6------------5-----------------------------------------------------6------
    |---000---|---001---|---010---|---011---|---100---|---101---|---110---|---111---| lo
000 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
001 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
010 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
011 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
100 | CVT.S.W | CVT.D.W | ---     | ---     | ---     | ---     | ---     | ---     |
101 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
110 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
111 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
 hi |---------|---------|---------|---------|---------|---------|---------|---------|
*/

    // OP fd, fs
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x20, cvt_s_w, cvt.s.w, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true
    )
    RAB_DEF_OPCODE_ALTNAME(
        core, 0x21, cvt_d_w, cvt.d.w, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true
    )
