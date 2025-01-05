/* SPDX-FileCopyrightText: © 2023-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    /* Indexed by "function" operand */

    RAB_DEF_OPCODE(
        r3000gte, 0x01, RTPS, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x30, RTPT, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x29, DPCL, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x10, DPCS, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x2A, DPCT, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x11, INTPL, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x1E, NCS, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x20, NCT, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x13, NCDS, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x16, NCDT, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x1B, NCCS, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x3F, NCCT, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x14, CDP, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x1C, CC, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x06, NCLIP, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x2D, AVSZ3, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x2E, AVSZ4, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr0()
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x12, MVMVA, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr5(Operand::r3000gte_sf, Operand::r3000gte_mx, Operand::r3000gte_v, Operand::r3000gte_cv, Operand::r3000gte_lm)
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x28, SQR, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x0C, OP, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x3D, GPF, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr1(Operand::r3000gte_sf)
    )

    RAB_DEF_OPCODE(
        r3000gte, 0x3E, GPL, IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE),
        operands: Operand::arr1(Operand::r3000gte_sf)
    )
