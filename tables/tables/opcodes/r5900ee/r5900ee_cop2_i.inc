/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25-24---21 --------------------------------------1-0
    |  = COP2   |0|  fmt  |                                       |1|
    ------6------1----4--------------------------------------------1-
    |-------00------|-------01------|-------10------|-------11------| lo
 00 | ---           | QMFC2.I       | CFC2.I        | ---           |
 01 | ---           | QMTC2.I       | CTC2.I        | ---           |
 10 | *1            | ---           | ---           | ---           |
 11 | ---           | ---           | ---           | ---           |
 hi |---------------|---------------|---------------|---------------|

     *1 = BC instructions, see BC2 list
*/

    // TODO: properties
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP2_I, 0x01, qmfc2_i, qmfc2.i, R5900EE,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
        modifies_rt: true
    )
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP2_I, 0x02, cfc2_i, cfc2.i, R5900EE,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
        modifies_rt: true
    )
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP2_I, 0x05, qmtc2_i, qmtc2.i, R5900EE,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
        reads_rt: true
    )
    RAB_DEF_OPCODE_EXT_ALTNAME(
        r5900ee, R5900EE_COP2_I, 0x06, ctc2_i, ctc2.i, R5900EE,
        operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
        reads_rt: true
    )

    RAB_DEF_OPCODE_REDIRECT(0x08, r5900ee, coprocessor2_bc2)
