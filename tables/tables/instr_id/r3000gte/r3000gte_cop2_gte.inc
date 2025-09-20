/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    /* Indexed by "function" operand */

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x01, rtps,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x30, rtpt,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x29, dpcl,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x10, dpcs,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2A, dpct,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x11, intpl,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1E, ncs,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x20, nct,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x13, ncds,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x16, ncdt,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1B, nccs,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3F, ncct,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x14, cdp,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1C, cc,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x06, nclip,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2D, avsz3,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2E, avsz4,
        .operands={0}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x12, mvmva,
        .operands={RAB_OPERAND_r3000gte_sf, RAB_OPERAND_r3000gte_mx, RAB_OPERAND_r3000gte_v, RAB_OPERAND_r3000gte_cv, RAB_OPERAND_r3000gte_lm}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x28, sqr,
        .operands={RAB_OPERAND_r3000gte_sf}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x0C, op,
        .operands={RAB_OPERAND_r3000gte_sf}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3D, gpf,
        .operands={RAB_OPERAND_r3000gte_sf}
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3E, gpl,
        .operands={RAB_OPERAND_r3000gte_sf}
    )
