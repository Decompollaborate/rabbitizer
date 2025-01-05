/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPCODE_REMOVED(0x02, core, bltzl, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x03, core, bgezl, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x08, core, tgei, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x09, core, tgeiu, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x0A, core, tlti, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x0B, core, tltiu, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x0C, core, teqi, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x0E, core, tnei, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x12, core, bltzall, IsaVersion::EXTENSION, IsaExtension::RSP)
    RAB_DEF_OPCODE_REMOVED(0x13, core, bgezall, IsaVersion::EXTENSION, IsaExtension::RSP)

    RAB_DEF_OPCODE_REDIRECT(_, none, regimm)
