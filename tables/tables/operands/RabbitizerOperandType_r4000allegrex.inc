/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

    // scalar
    RAB_DEF_OPERAND(r4000allegrex, s_vs)
    RAB_DEF_OPERAND(r4000allegrex, s_vt)
    RAB_DEF_OPERAND(r4000allegrex, s_vd)
    RAB_DEF_OPERAND(r4000allegrex, s_vt_imm)
    RAB_DEF_OPERAND(r4000allegrex, s_vd_imm)

    // pair
    RAB_DEF_OPERAND(r4000allegrex, p_vs)
    RAB_DEF_OPERAND(r4000allegrex, p_vt)
    RAB_DEF_OPERAND(r4000allegrex, p_vd)

    // triple
    RAB_DEF_OPERAND(r4000allegrex, t_vs)
    RAB_DEF_OPERAND(r4000allegrex, t_vt)
    RAB_DEF_OPERAND(r4000allegrex, t_vd)

    // quad
    RAB_DEF_OPERAND(r4000allegrex, q_vs)
    RAB_DEF_OPERAND(r4000allegrex, q_vt)
    RAB_DEF_OPERAND(r4000allegrex, q_vd)
    RAB_DEF_OPERAND(r4000allegrex, q_vt_imm)

    // matrix pair
    RAB_DEF_OPERAND(r4000allegrex, mp_vs)
    RAB_DEF_OPERAND(r4000allegrex, mp_vt)
    RAB_DEF_OPERAND(r4000allegrex, mp_vd)
    RAB_DEF_OPERAND(r4000allegrex, mp_vs_transpose)

    // matrix triple
    RAB_DEF_OPERAND(r4000allegrex, mt_vs)
    RAB_DEF_OPERAND(r4000allegrex, mt_vt)
    RAB_DEF_OPERAND(r4000allegrex, mt_vd)
    RAB_DEF_OPERAND(r4000allegrex, mt_vs_transpose)

    // matrix quad
    RAB_DEF_OPERAND(r4000allegrex, mq_vs)
    RAB_DEF_OPERAND(r4000allegrex, mq_vt)
    RAB_DEF_OPERAND(r4000allegrex, mq_vd)
    RAB_DEF_OPERAND(r4000allegrex, mq_vs_transpose)

    RAB_DEF_OPERAND(r4000allegrex, cop2cs)
    RAB_DEF_OPERAND(r4000allegrex, cop2cd)

    RAB_DEF_OPERAND(r4000allegrex, pos)
    RAB_DEF_OPERAND(r4000allegrex, size)
    RAB_DEF_OPERAND(r4000allegrex, size_plus_pos)

    RAB_DEF_OPERAND(r4000allegrex, imm3)

    // RAB_DEF_OPERAND(r4000allegrex, offset14)
    RAB_DEF_OPERAND(r4000allegrex, offset14_base)
    RAB_DEF_OPERAND(r4000allegrex, offset14_base_maybe_wb)

    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond)
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_s_maybe_vs_maybe_vt)
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_p_maybe_vs_maybe_vt)
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_t_maybe_vs_maybe_vt)
    RAB_DEF_OPERAND(r4000allegrex, vcmp_cond_q_maybe_vs_maybe_vt)

    RAB_DEF_OPERAND(r4000allegrex, vconstant)

    RAB_DEF_OPERAND(r4000allegrex, power_of_two)
    RAB_DEF_OPERAND(r4000allegrex, vfpu_cc_bit)
    RAB_DEF_OPERAND(r4000allegrex, bn)

    RAB_DEF_OPERAND(r4000allegrex, int16)
    RAB_DEF_OPERAND(r4000allegrex, float16)

    RAB_DEF_OPERAND(r4000allegrex, p_vrot_code)
    RAB_DEF_OPERAND(r4000allegrex, t_vrot_code)
    RAB_DEF_OPERAND(r4000allegrex, q_vrot_code)

    RAB_DEF_OPERAND(r4000allegrex, rpx)
    RAB_DEF_OPERAND(r4000allegrex, rpy)
    RAB_DEF_OPERAND(r4000allegrex, rpz)
    RAB_DEF_OPERAND(r4000allegrex, rpw)

    RAB_DEF_OPERAND(r4000allegrex, wpx)
    RAB_DEF_OPERAND(r4000allegrex, wpy)
    RAB_DEF_OPERAND(r4000allegrex, wpz)
    RAB_DEF_OPERAND(r4000allegrex, wpw)
