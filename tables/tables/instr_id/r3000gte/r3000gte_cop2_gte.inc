/* SPDX-FileCopyrightText: © 2023-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    /* Indexed by "function" operand */

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x01, RTPS, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x30, RTPT, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x29, DPCL, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x10, DPCS, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2A, DPCT, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x11, INTPL, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1E, NCS, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x20, NCT, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x13, NCDS, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x16, NCDT, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1B, NCCS, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3F, NCCT, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x14, CDP, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x1C, CC, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x06, NCLIP, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2D, AVSZ3, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x2E, AVSZ4, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr0()
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x12, MVMVA, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr5(Operand::r3000gte_sf, Operand::r3000gte_mx, Operand::r3000gte_v, Operand::r3000gte_cv, Operand::r3000gte_lm)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x28, SQR, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x0C, OP, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3D, GPF, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RABBITIZER_DEF_INSTR_ID(
        r3000gte, 0x3E, GPL, IsaVersion::EXTENSION, IsaExtension::R3000GTE,
        operands: Operand::arr1(Operand::r3000gte_sf)
    )
