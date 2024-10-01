/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE(
        ALL, , INVALID, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )

    #include "opcodes/opcodes_core.inc"
    #include "opcodes/opcodes_rsp.inc"
    #include "opcodes/opcodes_r3000gte.inc"
    #include "opcodes/opcodes_r4000allegrex.inc"
    #include "opcodes/opcodes_r5900.inc"
