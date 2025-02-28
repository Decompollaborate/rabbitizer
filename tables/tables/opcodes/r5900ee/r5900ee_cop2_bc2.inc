/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21-20------16------------------------------0
    |  = COP0   |   BC2   |   fmt   |                               |
    ------6----------5----------5------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | BC2F  | BC2T  | BC2FL | BC2TL | ---   | ---   | ---   | ---   |
 01 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // TODO: operands and properties

    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x00, bc2f, R5900EE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        is_branch: true
    )
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x01, bc2t, R5900EE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        is_branch: true
    )
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x02, bc2fl, R5900EE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        is_branch: true,
        is_branch_likely: true
    )
    RAB_DEF_OPCODE_EXTENSION(
        r5900ee, 0x03, bc2tl, R5900EE,
        operands: Operand::arr1(Operand::core_branch_target_label),
        is_branch: true,
        is_branch_likely: true
    )
