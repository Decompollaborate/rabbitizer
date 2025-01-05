/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP vd, vs, vt[elementhigh]
    RAB_DEF_OPCODE(
        rsp, 0x00, vmulf, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x01, vmulu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x02, vrndp, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x03, vmulq, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x04, vmudl, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x05, vmudm, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x06, vmudn, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x07, vmudh, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x08, vmacf, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x09, vmacu, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x0A, vrndn, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x0B, vmacq, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x0C, vmadl, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x0D, vmadm, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x0E, vmadn, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x0F, vmadh, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x10, vadd, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x11, vsub, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x13, vabs, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x14, vaddc, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x15, vsubc, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x1D, vsar, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x28, vand, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x29, vnand, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x2A, vor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x2B, vnor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x2C, vxor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x2D, vnxor, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )

    RAB_DEF_OPCODE(
        rsp, 0x20, vlt, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x21, veq, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x22, vne, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x23, vge, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x24, vcl, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x25, vch, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x26, vcr, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x27, vmrg, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )

    // OP vd[de], vt[elementhigh]
    RAB_DEF_OPCODE(
        rsp, 0x30, vrcp, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x31, vrcpl, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // operands may be wrong
    RAB_DEF_OPCODE(
        rsp, 0x32, vrcph, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x33, vmov, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x34, vrsq, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x35, vrsql, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x36, vrsqh, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )

    // OP
    RAB_DEF_OPCODE(
        rsp, 0x37, vnop, IsaVersion::EXTENSION, IsaExtension::RSP,
        operands: Operand::arr0()
    )
