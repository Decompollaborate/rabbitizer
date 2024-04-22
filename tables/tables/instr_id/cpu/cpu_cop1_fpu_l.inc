/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31----------26-25--------21 -----------------------------------------5----------0
    |   = COP1    |   = L     |                                         | function  |
    -------6------------5-----------------------------------------------------6------
    |---000---|---001---|---010---|---011---|---100---|---101---|---110---|---111---| lo
000 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
001 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
010 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
011 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
100 | CVT.S.L | CVT.D.L | ---     | ---     | ---     | ---     | ---     | ---     |
101 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
110 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
111 | ---     | ---     | ---     | ---     | ---     | ---     | ---     | ---     |
 hi |---------|---------|---------|---------|---------|---------|---------|---------|
*/

    // OP fd, fs
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        cpu, 0x20, cvt_s_l, cvt.s.l,
        .operands={RAB_OPERAND_cpu_fd, RAB_OPERAND_cpu_fs},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isFloat=true,
        .modifiesFd=true,
        .readsFs=true
    )
    RABBITIZER_DEF_INSTR_ID_ALTNAME(
        cpu, 0x21, cvt_d_l, cvt.d.l,
        .operands={RAB_OPERAND_cpu_fd, RAB_OPERAND_cpu_fs},
        .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN,
        .isFloat=true,
        .isDouble=true,
        .modifiesFd=true,
        .readsFs=true
    )
