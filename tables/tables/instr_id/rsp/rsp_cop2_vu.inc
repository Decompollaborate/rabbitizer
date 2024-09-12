/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // OP vd, vs, vt[elementhigh]
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x00, vmulf,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x01, vmulu,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x02, vrndp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x03, vmulq,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x04, vmudl,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x05, vmudm,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x06, vmudn,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x07, vmudh,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x08, vmacf,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x09, vmacu,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0A, vrndn,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0B, vmacq,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0C, vmadl,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0D, vmadm,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0E, vmadn,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x0F, vmadh,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x10, vadd,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x11, vsub,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x13, vabs,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x14, vaddc,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x15, vsubc,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x1D, vsar,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x28, vand,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x29, vnand,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x2A, vor,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x2B, vnor,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x2C, vxor,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x2D, vnxor,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )

    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x20, vlt,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x21, veq,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x22, vne,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x23, vge,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x24, vcl,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x25, vch,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x26, vcr,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x27, vmrg,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )

    // OP vd[de], vt[elementhigh]
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x30, vrcp,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x31, vrcpl,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // operands may be wrong
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x32, vrcph,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x33, vmov,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x34, vrsq,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x35, vrsql,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x36, vrsqh,
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    )

    // OP
    RABBITIZER_DEF_INSTR_ID(
        rsp, 0x37, vnop,
        operands: Operand::arr0()
    )
