/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    /* Indexed by "function" operand */

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x01, RTPS,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x30, RTPT,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x29, DPCL,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x10, DPCS,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2A, DPCT,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x11, INTPL,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1E, NCS,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x20, NCT,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x13, NCDS,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x16, NCDT,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1B, NCCS,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3F, NCCT,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x14, CDP,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1C, CC,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x06, NCLIP,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2D, AVSZ3,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2E, AVSZ4,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x12, MVMVA,
        operands: Operand::arr5(Operand::r3000gte_sf, Operand::r3000gte_mx, Operand::r3000gte_v, Operand::r3000gte_cv, Operand::r3000gte_lm)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x28, SQR,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x0C, OP,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3D, GPF,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3E, GPL,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )
