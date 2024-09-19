/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // scalar
    RAB_DEF_OPERAND(r4000allegrex, s_vs, EncodedFieldMask::r4000allegrex_vs)
    RAB_DEF_OPERAND(r4000allegrex, s_vt, EncodedFieldMask::r4000allegrex_vt)
    RAB_DEF_OPERAND(r4000allegrex, s_vd, EncodedFieldMask::r4000allegrex_vd)
    RAB_DEF_OPERAND(r4000allegrex, s_vt_imm, EncodedFieldMask::r4000allegrex_vt_imm)
    RAB_DEF_OPERAND(r4000allegrex, s_vd_imm, EncodedFieldMask::r4000allegrex_vd_imm)

    // pair
    RAB_DEF_OPERAND(r4000allegrex, p_vs, EncodedFieldMask::r4000allegrex_vs)
    RAB_DEF_OPERAND(r4000allegrex, p_vt, EncodedFieldMask::r4000allegrex_vt)
    RAB_DEF_OPERAND(r4000allegrex, p_vd, EncodedFieldMask::r4000allegrex_vd)

    // triple
    RAB_DEF_OPERAND(r4000allegrex, t_vs, EncodedFieldMask::r4000allegrex_vs)
    RAB_DEF_OPERAND(r4000allegrex, t_vt, EncodedFieldMask::r4000allegrex_vt)
    RAB_DEF_OPERAND(r4000allegrex, t_vd, EncodedFieldMask::r4000allegrex_vd)

    // quad
    RAB_DEF_OPERAND(r4000allegrex, q_vs, EncodedFieldMask::r4000allegrex_vs)
    RAB_DEF_OPERAND(r4000allegrex, q_vt, EncodedFieldMask::r4000allegrex_vt)
    RAB_DEF_OPERAND(r4000allegrex, q_vd, EncodedFieldMask::r4000allegrex_vd)
    RAB_DEF_OPERAND(r4000allegrex, q_vt_imm, EncodedFieldMask::r4000allegrex_vt_imm)

    // matrix pair
    RAB_DEF_OPERAND(r4000allegrex, mp_vs, EncodedFieldMask::r4000allegrex_vs)
    RAB_DEF_OPERAND(r4000allegrex, mp_vt, EncodedFieldMask::r4000allegrex_vt)
    RAB_DEF_OPERAND(r4000allegrex, mp_vd, EncodedFieldMask::r4000allegrex_vd)
    RAB_DEF_OPERAND(r4000allegrex, mp_vs_transpose, EncodedFieldMask::r4000allegrex_vs)

    // matrix triple
    RAB_DEF_OPERAND(r4000allegrex, mt_vs, EncodedFieldMask::r4000allegrex_vs)
    RAB_DEF_OPERAND(r4000allegrex, mt_vt, EncodedFieldMask::r4000allegrex_vt)
    RAB_DEF_OPERAND(r4000allegrex, mt_vd, EncodedFieldMask::r4000allegrex_vd)
    RAB_DEF_OPERAND(r4000allegrex, mt_vs_transpose, EncodedFieldMask::r4000allegrex_vs)

    // matrix quad
    RAB_DEF_OPERAND(r4000allegrex, mq_vs, EncodedFieldMask::r4000allegrex_vs)
    RAB_DEF_OPERAND(r4000allegrex, mq_vt, EncodedFieldMask::r4000allegrex_vt)
    RAB_DEF_OPERAND(r4000allegrex, mq_vd, EncodedFieldMask::r4000allegrex_vd)
    RAB_DEF_OPERAND(r4000allegrex, mq_vs_transpose, EncodedFieldMask::r4000allegrex_vs)

    RAB_DEF_OPERAND(r4000allegrex, cop2cs, EncodedFieldMask::r4000allegrex_cop2cs)
    RAB_DEF_OPERAND(r4000allegrex, cop2cd, EncodedFieldMask::r4000allegrex_cop2cd)

    RAB_DEF_OPERAND(r4000allegrex, pos, EncodedFieldMask::r4000allegrex_pos)
    RAB_DEF_OPERAND(r4000allegrex, size, EncodedFieldMask::r4000allegrex_size)
    RAB_DEF_OPERAND(r4000allegrex, size_plus_pos, EncodedFieldMask::r4000allegrex_size_plus_pos)

    RAB_DEF_OPERAND(r4000allegrex, imm3, EncodedFieldMask::r4000allegrex_imm3)

    // RAB_DEF_OPERAND(r4000allegrex, offset14, EncodedFieldMask::r4000allegrex_offset14)
    RAB_DEF_OPERAND(r4000allegrex, offset14_base, EncodedFieldMask::r4000allegrex_offset14.union(EncodedFieldMask::rs))
    RAB_DEF_OPERAND(r4000allegrex, offset14_base_maybe_wb, EncodedFieldMask::r4000allegrex_offset14.union(EncodedFieldMask::rs).union(EncodedFieldMask::r4000allegrex_wb))

    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond, EncodedFieldMask::r4000allegrex_vcmp_cond)
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_s_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt))
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_p_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt))
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_t_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt))
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_q_maybe_vs_maybe_vt, EncodedFieldMask::r4000allegrex_vcmp_cond.union(EncodedFieldMask::r4000allegrex_vs).union(EncodedFieldMask::r4000allegrex_vt))

    RAB_DEF_OPERAND(r4000allegrex, vconstant, EncodedFieldMask::r4000allegrex_vconstant)

    RAB_DEF_OPERAND(r4000allegrex, power_of_two, EncodedFieldMask::r4000allegrex_power_of_two)
    RAB_DEF_OPERAND(r4000allegrex, vfpu_cc_bit, EncodedFieldMask::r4000allegrex_vfpu_cc_bit)
    RAB_DEF_OPERAND(r4000allegrex, bn, EncodedFieldMask::r4000allegrex_bn)

    RAB_DEF_OPERAND(r4000allegrex, int16, EncodedFieldMask::r4000allegrex_intfloat16)
    RAB_DEF_OPERAND(r4000allegrex, float16, EncodedFieldMask::r4000allegrex_intfloat16)

    RAB_DEF_OPERAND(r4000allegrex, p_vrot_code, EncodedFieldMask::r4000allegrex_vrot_code)
    RAB_DEF_OPERAND(r4000allegrex, t_vrot_code, EncodedFieldMask::r4000allegrex_vrot_code)
    RAB_DEF_OPERAND(r4000allegrex, q_vrot_code, EncodedFieldMask::r4000allegrex_vrot_code)

    RAB_DEF_OPERAND(r4000allegrex, rpx, EncodedFieldMask::r4000allegrex_rpx)
    RAB_DEF_OPERAND(r4000allegrex, rpy, EncodedFieldMask::r4000allegrex_rpy)
    RAB_DEF_OPERAND(r4000allegrex, rpz, EncodedFieldMask::r4000allegrex_rpz)
    RAB_DEF_OPERAND(r4000allegrex, rpw, EncodedFieldMask::r4000allegrex_rpw)

    RAB_DEF_OPERAND(r4000allegrex, wpx, EncodedFieldMask::r4000allegrex_wpx)
    RAB_DEF_OPERAND(r4000allegrex, wpy, EncodedFieldMask::r4000allegrex_wpy)
    RAB_DEF_OPERAND(r4000allegrex, wpz, EncodedFieldMask::r4000allegrex_wpz)
    RAB_DEF_OPERAND(r4000allegrex, wpw, EncodedFieldMask::r4000allegrex_wpw)
