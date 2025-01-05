/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19----16-----------------7--------------0
    |   VFPU4   |0 0|0 1 0 0 1| fmt |t|             |p|             |
    ------6-------2------5-------3---1---------------1---------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
 00 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 01 | ---   | vmtvc | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | vmfvc | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 tp |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
vmfvc
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|1 0|0 0 0|1|    cop2cs   |0|      vd     |
    ------6-------2----3----2----3---1-------7---------------7-------
*/

    RAB_DEF_OPCODE(
        r4000allegrex, 0x0100, vmfvc, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_cop2cs),
        instr_type: InstrType::R
    ) // VFPU Move Word from VFPU Control

/*
vmtvc
    31--------26-25-24--21--19----16---14----------8---6------------0
    |   VFPU4   |0 0|0 1 0|1 0|0 0 1|0|      vs     |1|    cop2cd   |
    ------6-------2----3----2----3-----------7-------1-------7-------
*/

    RAB_DEF_OPCODE(
        r4000allegrex, 0x0201, vmtvc, IsaVersion::EXTENSION, Some(IsaExtension::R4000ALLEGREX),
        operands: Operand::arr2(Operand::r4000allegrex_cop2cd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R
    ) // VFPU Move Word to VFPU Control
