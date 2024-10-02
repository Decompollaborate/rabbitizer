/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPERAND(rsp, cop0d, EncodedFieldMask::cop0d, (RspCop0))
    RAB_DEF_OPERAND(rsp, cop2t, EncodedFieldMask::cop2t, (RspCop2))
    RAB_DEF_OPERAND(rsp, cop2cd, EncodedFieldMask::cop2cd, (RspCop2)) // Coprocessor 2 control rd
    RAB_DEF_OPERAND(rsp, vs, EncodedFieldMask::rsp_vs, (RspVector))
    RAB_DEF_OPERAND(rsp, vd, EncodedFieldMask::rsp_vd, (RspVector))

    // composed/aliased operands
    RAB_DEF_OPERAND(rsp, vt_elementhigh, EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementhigh), (RspVector, u8))
    RAB_DEF_OPERAND(rsp, vt_elementlow, EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementlow), (RspVector, u8))
    RAB_DEF_OPERAND(rsp, vd_de, EncodedFieldMask::rsp_vd.union(EncodedFieldMask::rsp_de), (RspVector, u8))
    RAB_DEF_OPERAND(rsp, vs_index, EncodedFieldMask::rsp_vs.union(EncodedFieldMask::rsp_index), (RspVector, u8))
    RAB_DEF_OPERAND(rsp, offset_rs, EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs), (u16, Gpr))
