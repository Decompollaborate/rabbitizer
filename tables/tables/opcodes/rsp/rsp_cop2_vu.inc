/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--------------------------------------5----------0
    |  = COP2   |1|                                     | function  |
    ------6------1--------------------------------------------6------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
000 | vmulf | vmulu | vrndp | vmulq | vmudl | vmudm | vmudn | vmudh |
001 | vmacf | vmacu | vrndn | vmacq | vmadl | vmadm | vmadn | vmadh |
010 | vadd  | vsub  |!vsut  | vabs  | vaddc | vsubc |!vaddb |!vsubb |
011 |!vaccb |!vsucb |!vsad  |!vsac  |!vsum  | vsar  |!vacc  |!vsuc  |
100 | vlt   | veq   | vne   | vge   | vcl   | vch   | vcr   | vmrg  |
101 | vand  | vnand | vor   | vnor  | vxor  | vnxor |!v056  |!v057  |
110 | vrcp  | vrcpl | vrcph | vmov  | vrsq  | vrsql | vrsqh | vnop  |
111 |!vextt |!vextq |!vextn |!v073  |!vinst |!vinsq |!vinsn |!vnull |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
*/

    // OP vd, vs, vt[elementhigh]
    RAB_DEF_OPCODE(
        rsp, 0x00, vmulf, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (frac) multiply
    RAB_DEF_OPCODE(
        rsp, 0x01, vmulu, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (uns. frac) multiply
    RAB_DEF_OPCODE(
        rsp, 0x02, vrndp, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector DCT round (+)
    RAB_DEF_OPCODE(
        rsp, 0x03, vmulq, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (int) multiply
    RAB_DEF_OPCODE(
        rsp, 0x04, vmudl, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (low) multiply
    RAB_DEF_OPCODE(
        rsp, 0x05, vmudm, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (mid-m) multiply
    RAB_DEF_OPCODE(
        rsp, 0x06, vmudn, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (mid-n) multiply
    RAB_DEF_OPCODE(
        rsp, 0x07, vmudh, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (high) multiply
    RAB_DEF_OPCODE(
        rsp, 0x08, vmacf, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (frac) multiply accumulate
    RAB_DEF_OPCODE(
        rsp, 0x09, vmacu, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (uns. frac) multiply accumulate
    RAB_DEF_OPCODE(
        rsp, 0x0A, vrndn, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector DCT round (-)
    RAB_DEF_OPCODE(
        rsp, 0x0B, vmacq, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (int) multiply accumulate
    RAB_DEF_OPCODE(
        rsp, 0x0C, vmadl, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (low) multiply accumulate
    RAB_DEF_OPCODE(
        rsp, 0x0D, vmadm, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (mid-m) multiply accumulate
    RAB_DEF_OPCODE(
        rsp, 0x0E, vmadn, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (mid-n) multiply accumulate
    RAB_DEF_OPCODE(
        rsp, 0x0F, vmadh, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector (high) multiply accumulate
    RAB_DEF_OPCODE(
        rsp, 0x10, vadd, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector add
    RAB_DEF_OPCODE(
        rsp, 0x11, vsub, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector sub
    RAB_DEF_OPCODE_GATED(
        rsp, 0x12, vsut, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector sut (vt - vs)
    RAB_DEF_OPCODE(
        rsp, 0x13, vabs, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector abs
    RAB_DEF_OPCODE(
        rsp, 0x14, vaddc, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector addc
    RAB_DEF_OPCODE(
        rsp, 0x15, vsubc, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector subc
    RAB_DEF_OPCODE_GATED(
        rsp, 0x16, vaddb, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector add byte
    RAB_DEF_OPCODE_GATED(
        rsp, 0x17, vsubb, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector sub byte
    RAB_DEF_OPCODE_GATED(
        rsp, 0x18, vaccb, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector add byte/add acc
    RAB_DEF_OPCODE_GATED(
        rsp, 0x19, vsucb, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector sub byte/add acc
    RAB_DEF_OPCODE_GATED(
        rsp, 0x1A, vsad, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector sad
    RAB_DEF_OPCODE_GATED(
        rsp, 0x1B, vsac, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector sac
    RAB_DEF_OPCODE_GATED(
        rsp, 0x1C, vsum, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector sum
    RAB_DEF_OPCODE(
        rsp, 0x1D, vsar, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE_GATED(
        rsp, 0x1E, vacc, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE_GATED(
        rsp, 0x1F, vsuc, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE(
        rsp, 0x20, vlt, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, <
    RAB_DEF_OPCODE(
        rsp, 0x21, veq, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, ==
    RAB_DEF_OPCODE(
        rsp, 0x22, vne, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, !=
    RAB_DEF_OPCODE(
        rsp, 0x23, vge, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, >=
    RAB_DEF_OPCODE(
        rsp, 0x24, vcl, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, clip low
    RAB_DEF_OPCODE(
        rsp, 0x25, vch, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, clip high
    RAB_DEF_OPCODE(
        rsp, 0x26, vcr, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, 1's comp clip
    RAB_DEF_OPCODE(
        rsp, 0x27, vmrg, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector select, merge
    RAB_DEF_OPCODE(
        rsp, 0x28, vand, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector logical op, and
    RAB_DEF_OPCODE(
        rsp, 0x29, vnand, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector logical op, nand
    RAB_DEF_OPCODE(
        rsp, 0x2A, vor, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector logical op, or
    RAB_DEF_OPCODE(
        rsp, 0x2B, vnor, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector logical op, nor
    RAB_DEF_OPCODE(
        rsp, 0x2C, vxor, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector logical op, xor
    RAB_DEF_OPCODE(
        rsp, 0x2D, vnxor, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector logical op, nxor
    RAB_DEF_OPCODE_GATED(
        rsp, 0x2E, v056, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE_GATED(
        rsp, 0x2F, v057, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )

    // OP vd[de], vt[elementhigh]
    RAB_DEF_OPCODE(
        rsp, 0x30, vrcp, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // single precision, lookup source, write result
    RAB_DEF_OPCODE(
        rsp, 0x31, vrcpl, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // lookup source and previous, write result
    RAB_DEF_OPCODE(
        rsp, 0x32, vrcph, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // set source, write previous result
    RAB_DEF_OPCODE(
        rsp, 0x33, vmov, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // vector scalar-element move
    RAB_DEF_OPCODE(
        rsp, 0x34, vrsq, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // single precision, lookup source, write result
    RAB_DEF_OPCODE(
        rsp, 0x35, vrsql, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // lookup source and previous, write result
    RAB_DEF_OPCODE(
        rsp, 0x36, vrsqh, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh)
    ) // set source, write previous result

    // OP
    RAB_DEF_OPCODE(
        rsp, 0x37, vnop, IsaVersion::EXTENSION, Some(IsaExtension::RSP),
        operands: Operand::arr0()
    ) // vector no-op

    // OP vd, vs, vt[elementhigh]
    RAB_DEF_OPCODE_GATED(
        rsp, 0x38, vextt, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector extract triple (5/5/5/1)
    RAB_DEF_OPCODE_GATED(
        rsp, 0x39, vextq, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector extract quad (4/4/4/4)
    RAB_DEF_OPCODE_GATED(
        rsp, 0x3A, vextn, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector extract nibble (4/4/4/4) sign-extend
    RAB_DEF_OPCODE_GATED(
        rsp, 0x3B, v073, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    )
    RAB_DEF_OPCODE_GATED(
        rsp, 0x3C, vinst, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector insert triple (5/5/5/1)
    RAB_DEF_OPCODE_GATED(
        rsp, 0x3D, vinsq, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector insert quad (4/4/4/4)
    RAB_DEF_OPCODE_GATED(
        rsp, 0x3E, vinsn, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr3(Operand::rsp_vd, Operand::rsp_vs, Operand::rsp_vt_elementhigh)
    ) // vector insert nibble (4/4/4/4) sign-extend

    // OP
    RAB_DEF_OPCODE_GATED(
        rsp, 0x3F, vnull, IsaVersion::EXTENSION, Some(IsaExtension::RSP), RspViceMsp,
        operands: Operand::arr0()
    )
