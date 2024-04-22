/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#ifndef OperandType_function_declarations_h_automatic
#define OperandType_function_declarations_h_automatic

    size_t RabbitizerOperandType_process_cpu_rs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_rt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_rd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_sa (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_zero (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_cop0d (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_fs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_ft (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_fd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_cop1cs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_cop2t (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_cop2cd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_op (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_hint (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_code (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_code_lower (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_copraw (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_label (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_immediate (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_branch_target_label (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_immediate_base (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_cpu_maybe_rd_rs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_rs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_rt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_rd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_cop0d (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_cop2t (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_cop2cd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_hint (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_vt_elementhigh (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_vt_elementlow (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_vd_de (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_vs_index (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_offset_rs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_immediate_base (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_rsp_maybe_rd_rs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r3000gte_sf (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r3000gte_mx (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r3000gte_v (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r3000gte_cv (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r3000gte_lm (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_s_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_s_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_s_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_s_vt_imm (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_s_vd_imm (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_p_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_p_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_p_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_t_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_t_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_t_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_q_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_q_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_q_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_q_vt_imm (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mp_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mp_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mp_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mp_vs_transpose (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mt_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mt_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mt_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mt_vs_transpose (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mq_vs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mq_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mq_vd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_mq_vs_transpose (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_cop2cs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_cop2cd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_pos (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_size (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_size_plus_pos (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_imm3 (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_offset14_base (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_offset14_base_maybe_wb (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_vconstant (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_power_of_two (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_vfpu_cc_bit (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_bn (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_int16 (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_float16 (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_p_vrot_code (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_t_vrot_code (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_q_vrot_code (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_rpx (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_rpy (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_rpz (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_rpw (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_wpx (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_wpy (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_wpz (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r4000allegrex_wpw (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_I (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_Q (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_R (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_ACC (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_ACCxyzw (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfs (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vft (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfd (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfsxyzw (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vftxyzw (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfdxyzw (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfsn (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vftn (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfdn (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfsl (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vftl (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfdl (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfsm (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vftm (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vfdm (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vis (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vit (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vid (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vis_predecr (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vit_predecr (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vid_predecr (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vis_postincr (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vit_postincr (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_vid_postincr (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_immediate5 (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);
    size_t RabbitizerOperandType_process_r5900_immediate15 (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);

#endif
