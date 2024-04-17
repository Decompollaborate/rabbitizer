/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-----21--19----16--------------------------------0
    |   VFPU4   |0 0|0 1 0|0 1| fmt |                               |
    ------6-------2----3----2----3-----------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
    | vmfvc | vmtvc | ---   | ---   | ---   | ---   | ---   | ---   |
    |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vmfvc
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|1 0|0 0 0|      imm8     | |      vd     |
    ------6-------2----3----2----3---1-------7---------------7-------
*/

    // TODO: vd, imm8
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x00, vmfvc,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // VFPU Move Word from VFPU Control

/*
vmtvc
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|1 0|0 0 1| |      vs     |      imm8     |
    ------6-------2----3----2----3-----------7-------1-------7-------
*/

    // TODO: imm8, vs
    RABBITIZER_DEF_INSTR_ID(
        r4000allegrex, 0x01, vmtvc,
        .operands={RAB_OPERAND_r4000allegrex_s_vd, RAB_OPERAND_r4000allegrex_s_vs},
        .instrType=RABBITIZER_INSTR_TYPE_R
    ) // VFPU Move Word to VFPU Control
