/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RABBITIZER_DEF_INSTR_ID(
        ALL, , INVALID, IsaVersion::MIPS_I, IsaExtension::NONE,
        operands: Operand::arr0()
    )

    #include "instr_id/RabbitizerInstrId_core.inc"
    #include "instr_id/RabbitizerInstrId_rsp.inc"
    #include "instr_id/RabbitizerInstrId_r3000gte.inc"
    #include "instr_id/RabbitizerInstrId_r4000allegrex.inc"
    #include "instr_id/RabbitizerInstrId_r5900.inc"
