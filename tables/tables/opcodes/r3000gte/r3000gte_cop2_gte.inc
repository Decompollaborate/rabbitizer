/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    /* Indexed by "function" operand */

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x01, RTPS, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x30, RTPT, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x29, DPCL, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x10, DPCS, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x2A, DPCT, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x11, INTPL, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x1E, NCS, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x20, NCT, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x13, NCDS, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x16, NCDT, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x1B, NCCS, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x3F, NCCT, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x14, CDP, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x1C, CC, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x06, NCLIP, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x2D, AVSZ3, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x2E, AVSZ4, R3000GTE,
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x12, MVMVA, R3000GTE,
        operands: Operand::arr5(Operand::r3000gte_sf, Operand::r3000gte_mx, Operand::r3000gte_v, Operand::r3000gte_cv, Operand::r3000gte_lm)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x28, SQR, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x0C, OP, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x3D, GPF, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE_EXT(
        r3000gte, 0x3E, GPL, R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )
