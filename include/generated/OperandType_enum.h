/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#ifndef OperandType_enum_h_automatic
#define OperandType_enum_h_automatic

typedef enum RabbitizerOperandType {
    RAB_OPERAND_ALL_INVALID,
    RAB_OPERAND_cpu_rs,
    RAB_OPERAND_cpu_rt,
    RAB_OPERAND_cpu_rd,
    RAB_OPERAND_cpu_sa,
    RAB_OPERAND_cpu_zero,
    RAB_OPERAND_cpu_cop0d,
    RAB_OPERAND_cpu_fs,
    RAB_OPERAND_cpu_ft,
    RAB_OPERAND_cpu_fd,
    RAB_OPERAND_cpu_cop1cs,
    RAB_OPERAND_cpu_cop2t,
    RAB_OPERAND_cpu_cop2cd,
    RAB_OPERAND_cpu_op,
    RAB_OPERAND_cpu_hint,
    RAB_OPERAND_cpu_code,
    RAB_OPERAND_cpu_code_lower,
    RAB_OPERAND_cpu_copraw,
    RAB_OPERAND_cpu_label,
    RAB_OPERAND_cpu_immediate,
    RAB_OPERAND_cpu_branch_target_label,
    RAB_OPERAND_cpu_immediate_base,
    RAB_OPERAND_cpu_maybe_rd_rs,
    RAB_OPERAND_rsp_rs,
    RAB_OPERAND_rsp_rt,
    RAB_OPERAND_rsp_rd,
    RAB_OPERAND_rsp_cop0d,
    RAB_OPERAND_rsp_cop2t,
    RAB_OPERAND_rsp_cop2cd,
    RAB_OPERAND_rsp_vs,
    RAB_OPERAND_rsp_vt,
    RAB_OPERAND_rsp_vd,
    RAB_OPERAND_rsp_hint,
    RAB_OPERAND_rsp_vt_elementhigh,
    RAB_OPERAND_rsp_vt_elementlow,
    RAB_OPERAND_rsp_vd_de,
    RAB_OPERAND_rsp_vs_index,
    RAB_OPERAND_rsp_offset_rs,
    RAB_OPERAND_rsp_immediate_base,
    RAB_OPERAND_rsp_maybe_rd_rs,
    RAB_OPERAND_r3000gte_sf,
    RAB_OPERAND_r3000gte_mx,
    RAB_OPERAND_r3000gte_v,
    RAB_OPERAND_r3000gte_cv,
    RAB_OPERAND_r3000gte_lm,
    RAB_OPERAND_r4000allegrex_s_vs,
    RAB_OPERAND_r4000allegrex_s_vt,
    RAB_OPERAND_r4000allegrex_s_vd,
    RAB_OPERAND_r4000allegrex_s_vt_imm,
    RAB_OPERAND_r4000allegrex_s_vd_imm,
    RAB_OPERAND_r4000allegrex_q_vs,
    RAB_OPERAND_r4000allegrex_q_vt,
    RAB_OPERAND_r4000allegrex_q_vd,
    RAB_OPERAND_r4000allegrex_q_vt_imm,
    RAB_OPERAND_r4000allegrex_pos,
    RAB_OPERAND_r4000allegrex_size,
    RAB_OPERAND_r4000allegrex_size_plus_pos,
    RAB_OPERAND_r4000allegrex_imm3,
    RAB_OPERAND_r4000allegrex_offset14_base,
    RAB_OPERAND_r4000allegrex_offset14_base_maybe_wb,
    RAB_OPERAND_r5900_I,
    RAB_OPERAND_r5900_Q,
    RAB_OPERAND_r5900_R,
    RAB_OPERAND_r5900_ACC,
    RAB_OPERAND_r5900_ACCxyzw,
    RAB_OPERAND_r5900_vfs,
    RAB_OPERAND_r5900_vft,
    RAB_OPERAND_r5900_vfd,
    RAB_OPERAND_r5900_vfsxyzw,
    RAB_OPERAND_r5900_vftxyzw,
    RAB_OPERAND_r5900_vfdxyzw,
    RAB_OPERAND_r5900_vfsn,
    RAB_OPERAND_r5900_vftn,
    RAB_OPERAND_r5900_vfdn,
    RAB_OPERAND_r5900_vfsl,
    RAB_OPERAND_r5900_vftl,
    RAB_OPERAND_r5900_vfdl,
    RAB_OPERAND_r5900_vfsm,
    RAB_OPERAND_r5900_vftm,
    RAB_OPERAND_r5900_vfdm,
    RAB_OPERAND_r5900_vis,
    RAB_OPERAND_r5900_vit,
    RAB_OPERAND_r5900_vid,
    RAB_OPERAND_r5900_vis_predecr,
    RAB_OPERAND_r5900_vit_predecr,
    RAB_OPERAND_r5900_vid_predecr,
    RAB_OPERAND_r5900_vis_postincr,
    RAB_OPERAND_r5900_vit_postincr,
    RAB_OPERAND_r5900_vid_postincr,
    RAB_OPERAND_r5900_immediate5,
    RAB_OPERAND_r5900_immediate15,
    RAB_OPERAND_ALL_MAX,
} RabbitizerOperandType;

#endif
