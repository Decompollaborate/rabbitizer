/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // scalar
    RAB_DEF_OPERAND(r4000allegrex, s_vs, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexS))
    RAB_DEF_OPERAND(r4000allegrex, s_vt, EncodedFieldMask::r4000allegrex_vt, (R4000AllegrexS))
    RAB_DEF_OPERAND(r4000allegrex, s_vd, EncodedFieldMask::r4000allegrex_vd, (R4000AllegrexS))
    RAB_DEF_OPERAND(r4000allegrex, s_vt_imm, EncodedFieldMask::r4000allegrex_vt_imm, (R4000AllegrexS))
    RAB_DEF_OPERAND(r4000allegrex, s_vd_imm, EncodedFieldMask::r4000allegrex_vd_imm, (R4000AllegrexS))

    // pair
    RAB_DEF_OPERAND(r4000allegrex, p_vs, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexV2D))
    RAB_DEF_OPERAND(r4000allegrex, p_vt, EncodedFieldMask::r4000allegrex_vt, (R4000AllegrexV2D))
    RAB_DEF_OPERAND(r4000allegrex, p_vd, EncodedFieldMask::r4000allegrex_vd, (R4000AllegrexV2D))

    // triple
    RAB_DEF_OPERAND(r4000allegrex, t_vs, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexV3D))
    RAB_DEF_OPERAND(r4000allegrex, t_vt, EncodedFieldMask::r4000allegrex_vt, (R4000AllegrexV3D))
    RAB_DEF_OPERAND(r4000allegrex, t_vd, EncodedFieldMask::r4000allegrex_vd, (R4000AllegrexV3D))

    // quad
    RAB_DEF_OPERAND(r4000allegrex, q_vs, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexV4D))
    RAB_DEF_OPERAND(r4000allegrex, q_vt, EncodedFieldMask::r4000allegrex_vt, (R4000AllegrexV4D))
    RAB_DEF_OPERAND(r4000allegrex, q_vd, EncodedFieldMask::r4000allegrex_vd, (R4000AllegrexV4D))
    RAB_DEF_OPERAND(r4000allegrex, q_vt_imm, EncodedFieldMask::r4000allegrex_vt_6_imm, (R4000AllegrexV4D))

    // matrix pair
    RAB_DEF_OPERAND(r4000allegrex, mp_vs, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexM2x2))
    RAB_DEF_OPERAND(r4000allegrex, mp_vt, EncodedFieldMask::r4000allegrex_vt, (R4000AllegrexM2x2))
    RAB_DEF_OPERAND(r4000allegrex, mp_vd, EncodedFieldMask::r4000allegrex_vd, (R4000AllegrexM2x2))
    RAB_DEF_OPERAND(r4000allegrex, mp_vs_transpose, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexM2x2))

    // matrix triple
    RAB_DEF_OPERAND(r4000allegrex, mt_vs, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexM3x3))
    RAB_DEF_OPERAND(r4000allegrex, mt_vt, EncodedFieldMask::r4000allegrex_vt, (R4000AllegrexM3x3))
    RAB_DEF_OPERAND(r4000allegrex, mt_vd, EncodedFieldMask::r4000allegrex_vd, (R4000AllegrexM3x3))
    RAB_DEF_OPERAND(r4000allegrex, mt_vs_transpose, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexM3x3))

    // matrix quad
    RAB_DEF_OPERAND(r4000allegrex, mq_vs, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexM4x4))
    RAB_DEF_OPERAND(r4000allegrex, mq_vt, EncodedFieldMask::r4000allegrex_vt, (R4000AllegrexM4x4))
    RAB_DEF_OPERAND(r4000allegrex, mq_vd, EncodedFieldMask::r4000allegrex_vd, (R4000AllegrexM4x4))
    RAB_DEF_OPERAND(r4000allegrex, mq_vs_transpose, EncodedFieldMask::r4000allegrex_vs, (R4000AllegrexM4x4))

    RAB_DEF_OPERAND(r4000allegrex, cop2cs, EncodedFieldMask::r4000allegrex_cop2cs, (R4000AllegrexVfpuControl))
    RAB_DEF_OPERAND(r4000allegrex, cop2cd, EncodedFieldMask::r4000allegrex_cop2cd, (R4000AllegrexVfpuControl))

    RAB_DEF_OPERAND(r4000allegrex, pos, EncodedFieldMask::r4000allegrex_pos, (u8))
    RAB_DEF_OPERAND(r4000allegrex, size, EncodedFieldMask::r4000allegrex_size, (u8))
    RAB_DEF_OPERAND(r4000allegrex, size_plus_pos, EncodedFieldMask::r4000allegrex_size_plus_pos, (i8))

    RAB_DEF_OPERAND(r4000allegrex, imm3, EncodedFieldMask::r4000allegrex_imm3, (u8))

    // RAB_DEF_OPERAND(r4000allegrex, offset14, EncodedFieldMask::r4000allegrex_offset14, ())
    RAB_DEF_OPERAND(r4000allegrex, offset14_base, EncodedFieldMask::r4000allegrex_offset14.union(EncodedFieldMask::rs), (u16, Gpr))
    RAB_DEF_OPERAND(r4000allegrex, offset14_base_maybe_wb, EncodedFieldMask::r4000allegrex_offset14.union(EncodedFieldMask::rs).union(EncodedFieldMask::r4000allegrex_wb), (u16, Gpr, bool))

    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_s_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt), (R4000AllegrexVCond, Option<R4000AllegrexS>, Option<R4000AllegrexS>))
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_p_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt), (R4000AllegrexVCond, Option<R4000AllegrexV2D>, Option<R4000AllegrexV2D>))
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_t_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt), (R4000AllegrexVCond, Option<R4000AllegrexV3D>, Option<R4000AllegrexV3D>))
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_q_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt), (R4000AllegrexVCond, Option<R4000AllegrexV4D>, Option<R4000AllegrexV4D>))

    RAB_DEF_OPERAND(r4000allegrex, vconstant, EncodedFieldMask::r4000allegrex_vconstant, (R4000AllegrexVConstant))

    RAB_DEF_OPERAND(r4000allegrex, power_of_two, EncodedFieldMask::r4000allegrex_power_of_two, (u8))
    RAB_DEF_OPERAND(r4000allegrex, vfpu_cc_bit, EncodedFieldMask::r4000allegrex_vfpu_cc_bit, (u8))
    RAB_DEF_OPERAND(r4000allegrex, bn, EncodedFieldMask::r4000allegrex_bn, (u8))

    RAB_DEF_OPERAND(r4000allegrex, int16, EncodedFieldMask::r4000allegrex_intfloat16, (i16))
    RAB_DEF_OPERAND(r4000allegrex, float16, EncodedFieldMask::r4000allegrex_intfloat16, (f32))

    RAB_DEF_OPERAND(r4000allegrex, p_vrot_code, EncodedFieldMask::r4000allegrex_vrot_code, (u8))
    RAB_DEF_OPERAND(r4000allegrex, t_vrot_code, EncodedFieldMask::r4000allegrex_vrot_code, (u8))
    RAB_DEF_OPERAND(r4000allegrex, q_vrot_code, EncodedFieldMask::r4000allegrex_vrot_code, (u8))

    RAB_DEF_OPERAND(r4000allegrex, wpx, EncodedFieldMask::r4000allegrex_wpx, (R4000AllegrexPrefixDst))
    RAB_DEF_OPERAND(r4000allegrex, wpy, EncodedFieldMask::r4000allegrex_wpy, (R4000AllegrexPrefixDst))
    RAB_DEF_OPERAND(r4000allegrex, wpz, EncodedFieldMask::r4000allegrex_wpz, (R4000AllegrexPrefixDst))
    RAB_DEF_OPERAND(r4000allegrex, wpw, EncodedFieldMask::r4000allegrex_wpw, (R4000AllegrexPrefixDst))

    RAB_DEF_OPERAND(r4000allegrex, rpx, EncodedFieldMask::r4000allegrex_rpx, (R4000AllegrexPrefixSrc))
    RAB_DEF_OPERAND(r4000allegrex, rpy, EncodedFieldMask::r4000allegrex_rpy, (R4000AllegrexPrefixSrc))
    RAB_DEF_OPERAND(r4000allegrex, rpz, EncodedFieldMask::r4000allegrex_rpz, (R4000AllegrexPrefixSrc))
    RAB_DEF_OPERAND(r4000allegrex, rpw, EncodedFieldMask::r4000allegrex_rpw, (R4000AllegrexPrefixSrc))
