/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    RAB_DEF_OPERAND_EXT(RSP, rsp, cop0d, EncodedFieldMask::cop0d, (RspCop0))
    RAB_DEF_OPERAND_EXT(RSP, rsp, cop2cd, EncodedFieldMask::cop2cd, (RspCop2)) // Coprocessor 2 control rd
    RAB_DEF_OPERAND_EXT(RSP, rsp, vs, EncodedFieldMask::rsp_vs, (RspVector))
    RAB_DEF_OPERAND_EXT(RSP, rsp, vd, EncodedFieldMask::rsp_vd, (RspVector))

    // composed/aliased operands
    RAB_DEF_OPERAND_EXT(RSP, rsp, vt_elementhigh, EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementhigh), (RspVector, u8))
    RAB_DEF_OPERAND_EXT(RSP, rsp, vt_elementlow, EncodedFieldMask::rsp_vt.union(EncodedFieldMask::rsp_elementlow), (RspVector, u8))
    RAB_DEF_OPERAND_EXT(RSP, rsp, vd_de, EncodedFieldMask::rsp_vd.union(EncodedFieldMask::rsp_de), (RspVector, u8))
    RAB_DEF_OPERAND_EXT(RSP, rsp, vs_index, EncodedFieldMask::rsp_vs.union(EncodedFieldMask::rsp_index), (RspVector, u8))
    RAB_DEF_OPERAND_EXT(RSP, rsp, offset7_rs, EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs), (u8, Gpr))
    RAB_DEF_OPERAND_EXT(RSP, rsp, offset8_rs, EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs), (u8, Gpr))
    RAB_DEF_OPERAND_EXT(RSP, rsp, offset9_rs, EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs), (u16, Gpr))
    RAB_DEF_OPERAND_EXT(RSP, rsp, offset10_rs, EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs), (u16, Gpr))
    RAB_DEF_OPERAND_EXT(RSP, rsp, offset11_rs, EncodedFieldMask::rsp_offset.union(EncodedFieldMask::rs), (u16, Gpr))
