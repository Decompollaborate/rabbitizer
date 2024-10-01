/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-24---------------15--------------7--------------0
    |   VFPU4   |fmt|               |t|             |p|             |
    ------6-------2------------------1---------------1---------------
    |-------00------|-------01------|-------10------|-------11------| fmt
 00 | *1            | ---           | *2            | vwbn.s        |
 01 | *1            | ---           | *2            | ---           |
 10 | *1            | ---           | *2            | ---           |
 11 | *1            | ---           | *2            | ---           |
 tp |---------------|---------------|---------------|---------------|
      *1 See fmt0 table
      *2 See fmt2 table
*/

/*
vwbn.s
    31--------26-25-24------------16---14----------8---6------------0
    |   VFPU4   |1 1|      imm8     |0|      vs     |0|      vd     |
    ------6-------2---------8--------1-------7-------1-------7-------
*/

    RAB_DEF_OPCODE_ALTNAME(
        r4000allegrex, 0x3 << 2 | 0x0, vwbn_s, vwbn.s, IsaVersion::EXTENSION, IsaExtension::R4000ALLEGREX,
        operands: Operand::arr3(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs, Operand::r4000allegrex_bn),
        instr_type: InstrType::R
    ) // WrapBN Single word
