/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31---------26------------------------------10------6-5----------0
    |0 1 1 1 0 0|                             |   fmt   |1 1 0 0 0 0|
    ------6----------------------------------------5----------6------
    |-------00------|-------01------|-------10------|-------11------|
000 | PMFHL.LW      | PMFHL.UW      | PMFHL.SLW     | PMFHL.LH      |
001 | PMFHL.SH      | ---           | ---           | ---           |
010 | ---           | ---           | ---           | ---           |
011 | ---           | ---           | ---           | ---           |
100 | ---           | ---           | ---           | ---           |
101 | ---           | ---           | ---           | ---           |
110 | ---           | ---           | ---           | ---           |
111 | ---           | ---           | ---           | ---           |
 hi |---------------|---------------|---------------|---------------|
*/

    RAB_DEF_OPCODE_ALTNAME(
        r5900, 0x00, pmfhl_lw, pmfhl.lw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From Hi/Lo register

    RAB_DEF_OPCODE_ALTNAME(
        r5900, 0x01, pmfhl_uw, pmfhl.uw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From Hi/Lo register

    RAB_DEF_OPCODE_ALTNAME(
        r5900, 0x02, pmfhl_slw, pmfhl.slw, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From Hi/Lo register

    RAB_DEF_OPCODE_ALTNAME(
        r5900, 0x03, pmfhl_lh, pmfhl.lh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From Hi/Lo register

    RAB_DEF_OPCODE_ALTNAME(
        r5900, 0x04, pmfhl_sh, pmfhl.sh, IsaVersion::EXTENSION, IsaExtension::R5900,
        operands: Operand::arr1(Operand::core_rd),
        modifies_rd: true
    ) // Parallel Move From Hi/Lo register