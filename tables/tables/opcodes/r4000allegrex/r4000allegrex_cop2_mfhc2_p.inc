/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------7-6---4--------0
    |  = COP2   |  MFHC2  |                         |1| fmt |       |
    ------6----------5-------------------------------1---3-----------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
    | mfvc  | ---   | ---   | ---   | ---   | ---   | ---   | *1    |
    |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 See S list
*/

/*
mfvc
    31--------26-25-----21-20-----16-----------------7-6------------0
    |    COP2   |0 0 0 1 1|    rt   |0 0 0 0 0 0 0 0|1|    cop2cd   |
    ------6----------5---------5---------------------1-------7-------
*/

    RAB_DEF_OPCODE_EXT(
        r4000allegrex, R4000ALLEGREX_COP2_MFHC2_P, 0x0, mfvc, R4000ALLEGREX,
        operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_cop2cd),
        instr_type: InstrType::R,
        modifies_rt: true
    ) // Move word From Vfpu Control

    RAB_DEF_OPCODE_REDIRECT(0x07, r4000allegrex, coprocessor2_mfhc2_p_s)
