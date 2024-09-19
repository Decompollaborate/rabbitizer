/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPERAND(rsp, rs, EncodedFieldMask::rs)
    RAB_DEF_OPERAND(rsp, rt, EncodedFieldMask::rt)
    RAB_DEF_OPERAND(rsp, rd, EncodedFieldMask::rd)
    RAB_DEF_OPERAND(rsp, cop0d, EncodedFieldMask::cop0d)
    RAB_DEF_OPERAND(rsp, cop2t, EncodedFieldMask::cop2t)
    RAB_DEF_OPERAND(rsp, cop2cd, EncodedFieldMask::cop2cd) // Coprocessor 2 control rd
    RAB_DEF_OPERAND(rsp, vs, EncodedFieldMask::rsp_vs)
    RAB_DEF_OPERAND(rsp, vt, EncodedFieldMask::rsp_vt)
    RAB_DEF_OPERAND(rsp, vd, EncodedFieldMask::rsp_vd)
    RAB_DEF_OPERAND(rsp, hint, EncodedFieldMask::hint)
    // RAB_DEF_OPERAND(rsp, elementhigh, EncodedFieldMask::rsp_elementhigh)
    // RAB_DEF_OPERAND(rsp, elementlow, EncodedFieldMask::rsp_elementlow)
    // RAB_DEF_OPERAND(rsp, index, EncodedFieldMask::rsp_index)
    // RAB_DEF_OPERAND(rsp, offset, EncodedFieldMask::rsp_offset)

    // composed/aliased operands
    RAB_DEF_OPERAND(rsp, vt_elementhigh, EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementhigh))
    RAB_DEF_OPERAND(rsp, vt_elementlow, EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementlow))
    RAB_DEF_OPERAND(rsp, vd_de, EncodedFieldMask::rsp_vd.union(EncodedFieldMask::rsp_de))
    RAB_DEF_OPERAND(rsp, vs_index, EncodedFieldMask::rsp_vs.union(EncodedFieldMask::rsp_index))
    RAB_DEF_OPERAND(rsp, offset_rs, EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs))
    RAB_DEF_OPERAND(rsp, immediate_base, EncodedFieldMask::immediate.union(EncodedFieldMask::rs))
    RAB_DEF_OPERAND(rsp, maybe_rd_rs, EncodedFieldMask::rd.union(EncodedFieldMask::rs))
