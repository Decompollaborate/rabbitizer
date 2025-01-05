/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP vt[elementlow], offset(vs)
    RAB_DEF_OPCODE(
        rsp, 0x00, sbv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x01, ssv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x02, slv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x03, sdv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x04, sqv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x05, srv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x06, spv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x07, suv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x08, shv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x09, sfv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
    RAB_DEF_OPCODE(
        rsp, 0x0B, stv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )

    RAB_DEF_OPCODE(
        rsp, 0xFFFFFF07, swv, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true
    )
