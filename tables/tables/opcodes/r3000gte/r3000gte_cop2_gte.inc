/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    /* Indexed by "function" operand */

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x01, rtps, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x30, rtpt, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x29, dpcl, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x10, dpcs, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x2A, dpct, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_gbg)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x11, intpl, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x1E, ncs, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x20, nct, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x13, ncds, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x16, ncdt, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x1B, nccs, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x3F, ncct, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x14, cdp, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x1C, cc, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x06, nclip, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x2D, avsz3, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x2E, avsz4, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x12, mvmva, R3000GTE,
        operands: Operand::arr5(Operand::r3000gte_sf, Operand::r3000gte_mx, Operand::r3000gte_v, Operand::r3000gte_cv, Operand::r3000gte_lm)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x28, sqr, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x0C, op, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x3D, gpf, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, R3000GTE_COP2_GTE, 0x3E, gpl, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )
