#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
# SPDX-License-Identifier: MIT

# Automatically generated. DO NOT MODIFY

from __future__ import annotations
from .Enum import Enum
class OperandType:
    ALL_INVALID: Enum
    cpu_rs: Enum
    cpu_rt: Enum
    cpu_rd: Enum
    cpu_sa: Enum
    cpu_zero: Enum
    cpu_cop0d: Enum
    cpu_fs: Enum
    cpu_ft: Enum
    cpu_fd: Enum
    cpu_cop1cs: Enum
    cpu_cop2t: Enum
    cpu_cop2cd: Enum
    cpu_op: Enum
    cpu_hint: Enum
    cpu_code: Enum
    cpu_code_lower: Enum
    cpu_copraw: Enum
    cpu_label: Enum
    cpu_immediate: Enum
    cpu_branch_target_label: Enum
    cpu_immediate_base: Enum
    cpu_maybe_rd_rs: Enum
    rsp_rs: Enum
    rsp_rt: Enum
    rsp_rd: Enum
    rsp_cop0d: Enum
    rsp_cop2t: Enum
    rsp_cop2cd: Enum
    rsp_vs: Enum
    rsp_vt: Enum
    rsp_vd: Enum
    rsp_hint: Enum
    rsp_vt_elementhigh: Enum
    rsp_vt_elementlow: Enum
    rsp_vd_de: Enum
    rsp_vs_index: Enum
    rsp_offset_rs: Enum
    rsp_immediate_base: Enum
    rsp_maybe_rd_rs: Enum
    r3000gte_sf: Enum
    r3000gte_mx: Enum
    r3000gte_v: Enum
    r3000gte_cv: Enum
    r3000gte_lm: Enum
    r4000allegrex_s_vs: Enum
    r4000allegrex_s_vt: Enum
    r4000allegrex_s_vd: Enum
    r4000allegrex_s_vt_imm: Enum
    r4000allegrex_s_vd_imm: Enum
    r4000allegrex_p_vs: Enum
    r4000allegrex_p_vt: Enum
    r4000allegrex_p_vd: Enum
    r4000allegrex_t_vs: Enum
    r4000allegrex_t_vt: Enum
    r4000allegrex_t_vd: Enum
    r4000allegrex_q_vs: Enum
    r4000allegrex_q_vt: Enum
    r4000allegrex_q_vd: Enum
    r4000allegrex_q_vt_imm: Enum
    r4000allegrex_mp_vs: Enum
    r4000allegrex_mp_vt: Enum
    r4000allegrex_mp_vd: Enum
    r4000allegrex_mt_vs: Enum
    r4000allegrex_mt_vt: Enum
    r4000allegrex_mt_vd: Enum
    r4000allegrex_mq_vs: Enum
    r4000allegrex_mq_vt: Enum
    r4000allegrex_mq_vd: Enum
    r4000allegrex_cop2cs: Enum
    r4000allegrex_cop2cd: Enum
    r4000allegrex_pos: Enum
    r4000allegrex_size: Enum
    r4000allegrex_size_plus_pos: Enum
    r4000allegrex_imm3: Enum
    r4000allegrex_offset14_base: Enum
    r4000allegrex_offset14_base_maybe_wb: Enum
    r4000allegrex_vcmp_cond: Enum
    r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt: Enum
    r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt: Enum
    r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt: Enum
    r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt: Enum
    r4000allegrex_vconstant: Enum
    r4000allegrex_power_of_two: Enum
    r4000allegrex_vfpu_cc_bit: Enum
    r5900_I: Enum
    r5900_Q: Enum
    r5900_R: Enum
    r5900_ACC: Enum
    r5900_ACCxyzw: Enum
    r5900_vfs: Enum
    r5900_vft: Enum
    r5900_vfd: Enum
    r5900_vfsxyzw: Enum
    r5900_vftxyzw: Enum
    r5900_vfdxyzw: Enum
    r5900_vfsn: Enum
    r5900_vftn: Enum
    r5900_vfdn: Enum
    r5900_vfsl: Enum
    r5900_vftl: Enum
    r5900_vfdl: Enum
    r5900_vfsm: Enum
    r5900_vftm: Enum
    r5900_vfdm: Enum
    r5900_vis: Enum
    r5900_vit: Enum
    r5900_vid: Enum
    r5900_vis_predecr: Enum
    r5900_vit_predecr: Enum
    r5900_vid_predecr: Enum
    r5900_vis_postincr: Enum
    r5900_vit_postincr: Enum
    r5900_vid_postincr: Enum
    r5900_immediate5: Enum
    r5900_immediate15: Enum
    ALL_MAX: Enum
