/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    ALL_INVALID,
    core_j,
    core_jal,
    core_beq,
    core_bne,
    core_beql,
    core_bnel,
    core_blez,
    core_blezl,
    core_bgtz,
    core_bgtzl,
    core_addi,
    core_addiu,
    core_slti,
    core_sltiu,
    core_andi,
    core_ori,
    core_xori,
    core_daddi,
    core_daddiu,
    core_lui,
    core_ldl,
    core_ldr,
    core_lb,
    core_lh,
    core_lwl,
    core_lw,
    core_lbu,
    core_lhu,
    core_lwr,
    core_lwu,
    core_sb,
    core_sh,
    core_swl,
    core_sw,
    core_sdl,
    core_sdr,
    core_swr,
    core_ll,
    core_pref,
    core_lld,
    core_ld,
    core_sc,
    core_scd,
    core_sd,
    core_cache,
    core_lwc1,
    core_ldc1,
    core_swc1,
    core_sdc1,
    core_lwc2,
    core_ldc2,
    core_swc2,
    core_sdc2,
    core_b,
    core_beqz,
    core_bnez,
    core_beqzl,
    core_bnezl,

    core_sll,
    core_srl,
    core_sra,
    core_dsll,
    core_dsrl,
    core_dsra,
    core_dsll32,
    core_dsrl32,
    core_dsra32,
    core_dsllv,
    core_dsrlv,
    core_dsrav,
    core_sllv,
    core_srlv,
    core_srav,
    core_mthi,
    core_mtlo,
    core_jr,
    core_jalr,
    core_mfhi,
    core_mflo,
    core_movz,
    core_movn,
    core_div,
    core_divu,
    core_ddiv,
    core_ddivu,
    core_add,
    core_addu,
    core_sub,
    core_subu,
    core_and,
    core_or,
    core_xor,
    core_nor,
    core_slt,
    core_sltu,
    core_dadd,
    core_daddu,
    core_dsub,
    core_dsubu,
    core_syscall,
    core_break,
    core_sync,
    core_mult,
    core_multu,
    core_dmult,
    core_dmultu,
    core_tge,
    core_tgeu,
    core_tlt,
    core_tltu,
    core_teq,
    core_tne,
    core_nop,
    core_not,
    core_neg,
    core_negu,
    core_bltz,
    core_bgez,
    core_bltzl,
    core_bgezl,
    core_tgei,
    core_tgeiu,
    core_tlti,
    core_tltiu,
    core_teqi,
    core_tnei,
    core_bltzal,
    core_bgezal,
    core_bltzall,
    core_bgezall,
    core_bal,
    core_mfc0,
    core_dmfc0,
    core_cfc0,
    core_mtc0,
    core_dmtc0,
    core_ctc0,

    core_bc0f,
    core_bc0t,
    core_bc0fl,
    core_bc0tl,
    core_tlbr,
    core_tlbwi,
    core_tlbwr,
    core_tlbp,
    core_rfe,
    core_eret,
    core_mfc1,
    core_dmfc1,
    core_mtc1,
    core_dmtc1,
    core_cfc1,
    core_ctc1,

    core_bc1f,
    core_bc1t,
    core_bc1fl,
    core_bc1tl,
    core_add_s,
    core_sub_s,
    core_mul_s,
    core_div_s,
    core_sqrt_s,
    core_abs_s,
    core_mov_s,
    core_neg_s,
    core_round_l_s,
    core_trunc_l_s,
    core_ceil_l_s,
    core_floor_l_s,
    core_round_w_s,
    core_trunc_w_s,
    core_ceil_w_s,
    core_floor_w_s,
    core_cvt_d_s,
    core_cvt_w_s,
    core_cvt_l_s,
    core_c_f_s,
    core_c_un_s,
    core_c_eq_s,
    core_c_ueq_s,
    core_c_olt_s,
    core_c_ult_s,
    core_c_ole_s,
    core_c_ule_s,
    core_c_sf_s,
    core_c_ngle_s,
    core_c_seq_s,
    core_c_ngl_s,
    core_c_lt_s,
    core_c_nge_s,
    core_c_le_s,
    core_c_ngt_s,
    core_add_d,
    core_sub_d,
    core_mul_d,
    core_div_d,
    core_sqrt_d,
    core_abs_d,
    core_mov_d,
    core_neg_d,
    core_round_l_d,
    core_trunc_l_d,
    core_ceil_l_d,
    core_floor_l_d,
    core_round_w_d,
    core_trunc_w_d,
    core_ceil_w_d,
    core_floor_w_d,
    core_cvt_s_d,
    core_cvt_w_d,
    core_cvt_l_d,
    core_c_f_d,
    core_c_un_d,
    core_c_eq_d,
    core_c_ueq_d,
    core_c_olt_d,
    core_c_ult_d,
    core_c_ole_d,
    core_c_ule_d,
    core_c_df_d,
    core_c_ngle_d,
    core_c_seq_d,
    core_c_ngl_d,
    core_c_lt_d,
    core_c_nge_d,
    core_c_le_d,
    core_c_ngt_d,
    core_cvt_s_w,
    core_cvt_d_w,
    core_cvt_s_l,
    core_cvt_d_l,
    core_mfc2,
    core_mtc2,
    core_cfc2,
    core_ctc2,
    #[cfg(feature = "RSP")]
    rsp_mfc2,
    #[cfg(feature = "RSP")]
    rsp_mtc2,
    #[cfg(feature = "RSP")]
    rsp_cfc2,
    #[cfg(feature = "RSP")]
    rsp_ctc2,

    #[cfg(feature = "RSP")]
    rsp_vmulf,
    #[cfg(feature = "RSP")]
    rsp_vmulu,
    #[cfg(feature = "RSP")]
    rsp_vrndp,
    #[cfg(feature = "RSP")]
    rsp_vmulq,
    #[cfg(feature = "RSP")]
    rsp_vmudl,
    #[cfg(feature = "RSP")]
    rsp_vmudm,
    #[cfg(feature = "RSP")]
    rsp_vmudn,
    #[cfg(feature = "RSP")]
    rsp_vmudh,
    #[cfg(feature = "RSP")]
    rsp_vmacf,
    #[cfg(feature = "RSP")]
    rsp_vmacu,
    #[cfg(feature = "RSP")]
    rsp_vrndn,
    #[cfg(feature = "RSP")]
    rsp_vmacq,
    #[cfg(feature = "RSP")]
    rsp_vmadl,
    #[cfg(feature = "RSP")]
    rsp_vmadm,
    #[cfg(feature = "RSP")]
    rsp_vmadn,
    #[cfg(feature = "RSP")]
    rsp_vmadh,
    #[cfg(feature = "RSP")]
    rsp_vadd,
    #[cfg(feature = "RSP")]
    rsp_vsub,
    #[cfg(feature = "RspViceMsp")]
    rsp_vsut,
    #[cfg(feature = "RSP")]
    rsp_vabs,
    #[cfg(feature = "RSP")]
    rsp_vaddc,
    #[cfg(feature = "RSP")]
    rsp_vsubc,
    #[cfg(feature = "RspViceMsp")]
    rsp_vaddb,
    #[cfg(feature = "RspViceMsp")]
    rsp_vsubb,
    #[cfg(feature = "RspViceMsp")]
    rsp_vaccb,
    #[cfg(feature = "RspViceMsp")]
    rsp_vsucb,
    #[cfg(feature = "RspViceMsp")]
    rsp_vsad,
    #[cfg(feature = "RspViceMsp")]
    rsp_vsac,
    #[cfg(feature = "RspViceMsp")]
    rsp_vsum,
    #[cfg(feature = "RSP")]
    rsp_vsar,
    #[cfg(feature = "RspViceMsp")]
    rsp_vacc,
    #[cfg(feature = "RspViceMsp")]
    rsp_vsuc,
    #[cfg(feature = "RSP")]
    rsp_vlt,
    #[cfg(feature = "RSP")]
    rsp_veq,
    #[cfg(feature = "RSP")]
    rsp_vne,
    #[cfg(feature = "RSP")]
    rsp_vge,
    #[cfg(feature = "RSP")]
    rsp_vcl,
    #[cfg(feature = "RSP")]
    rsp_vch,
    #[cfg(feature = "RSP")]
    rsp_vcr,
    #[cfg(feature = "RSP")]
    rsp_vmrg,
    #[cfg(feature = "RSP")]
    rsp_vand,
    #[cfg(feature = "RSP")]
    rsp_vnand,
    #[cfg(feature = "RSP")]
    rsp_vor,
    #[cfg(feature = "RSP")]
    rsp_vnor,
    #[cfg(feature = "RSP")]
    rsp_vxor,
    #[cfg(feature = "RSP")]
    rsp_vnxor,
    #[cfg(feature = "RspViceMsp")]
    rsp_v056,
    #[cfg(feature = "RspViceMsp")]
    rsp_v057,
    #[cfg(feature = "RSP")]
    rsp_vrcp,
    #[cfg(feature = "RSP")]
    rsp_vrcpl,
    #[cfg(feature = "RSP")]
    rsp_vrcph,
    #[cfg(feature = "RSP")]
    rsp_vmov,
    #[cfg(feature = "RSP")]
    rsp_vrsq,
    #[cfg(feature = "RSP")]
    rsp_vrsql,
    #[cfg(feature = "RSP")]
    rsp_vrsqh,
    #[cfg(feature = "RSP")]
    rsp_vnop,
    #[cfg(feature = "RspViceMsp")]
    rsp_vextt,
    #[cfg(feature = "RspViceMsp")]
    rsp_vextq,
    #[cfg(feature = "RspViceMsp")]
    rsp_vextn,
    #[cfg(feature = "RspViceMsp")]
    rsp_v073,
    #[cfg(feature = "RspViceMsp")]
    rsp_vinst,
    #[cfg(feature = "RspViceMsp")]
    rsp_vinsq,
    #[cfg(feature = "RspViceMsp")]
    rsp_vinsn,
    #[cfg(feature = "RspViceMsp")]
    rsp_vnull,
    #[cfg(feature = "RSP")]
    rsp_lbv,
    #[cfg(feature = "RSP")]
    rsp_lsv,
    #[cfg(feature = "RSP")]
    rsp_llv,
    #[cfg(feature = "RSP")]
    rsp_ldv,
    #[cfg(feature = "RSP")]
    rsp_lqv,
    #[cfg(feature = "RSP")]
    rsp_lrv,
    #[cfg(feature = "RSP")]
    rsp_lpv,
    #[cfg(feature = "RSP")]
    rsp_luv,
    #[cfg(feature = "RSP")]
    rsp_lhv,
    #[cfg(feature = "RSP")]
    rsp_lfv,
    #[cfg(feature = "RspViceMsp")]
    rsp_lwv,
    #[cfg(feature = "RSP")]
    rsp_ltv,
    #[cfg(feature = "RSP")]
    rsp_sbv,
    #[cfg(feature = "RSP")]
    rsp_ssv,
    #[cfg(feature = "RSP")]
    rsp_slv,
    #[cfg(feature = "RSP")]
    rsp_sdv,
    #[cfg(feature = "RSP")]
    rsp_sqv,
    #[cfg(feature = "RSP")]
    rsp_srv,
    #[cfg(feature = "RSP")]
    rsp_spv,
    #[cfg(feature = "RSP")]
    rsp_suv,
    #[cfg(feature = "RSP")]
    rsp_shv,
    #[cfg(feature = "RSP")]
    rsp_sfv,
    #[cfg(feature = "RSP")]
    rsp_swv,
    #[cfg(feature = "RSP")]
    rsp_stv,

    #[cfg(feature = "RSP")]
    rsp_mfc0,
    #[cfg(feature = "RSP")]
    rsp_mtc0,

    #[cfg(feature = "R3000GTE")]
    r3000gte_RTPS,
    #[cfg(feature = "R3000GTE")]
    r3000gte_RTPT,
    #[cfg(feature = "R3000GTE")]
    r3000gte_DPCL,
    #[cfg(feature = "R3000GTE")]
    r3000gte_DPCS,
    #[cfg(feature = "R3000GTE")]
    r3000gte_DPCT,
    #[cfg(feature = "R3000GTE")]
    r3000gte_INTPL,
    #[cfg(feature = "R3000GTE")]
    r3000gte_NCS,
    #[cfg(feature = "R3000GTE")]
    r3000gte_NCT,
    #[cfg(feature = "R3000GTE")]
    r3000gte_NCDS,
    #[cfg(feature = "R3000GTE")]
    r3000gte_NCDT,
    #[cfg(feature = "R3000GTE")]
    r3000gte_NCCS,
    #[cfg(feature = "R3000GTE")]
    r3000gte_NCCT,
    #[cfg(feature = "R3000GTE")]
    r3000gte_CDP,
    #[cfg(feature = "R3000GTE")]
    r3000gte_CC,
    #[cfg(feature = "R3000GTE")]
    r3000gte_NCLIP,
    #[cfg(feature = "R3000GTE")]
    r3000gte_AVSZ3,
    #[cfg(feature = "R3000GTE")]
    r3000gte_AVSZ4,
    #[cfg(feature = "R3000GTE")]
    r3000gte_MVMVA,
    #[cfg(feature = "R3000GTE")]
    r3000gte_SQR,
    #[cfg(feature = "R3000GTE")]
    r3000gte_OP,
    #[cfg(feature = "R3000GTE")]
    r3000gte_GPF,
    #[cfg(feature = "R3000GTE")]
    r3000gte_GPL,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_lv_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_sv_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_lv_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_sv_q,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_clz,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_clo,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_madd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_maddu,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_msub,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_msubu,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_movz,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_movn,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_max,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_min,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_srl,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rotr,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_srlv,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_rotrv,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_sleep,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mfie,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mtie,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_ext,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_ins,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wsbh,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_wsbw,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_seb,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_seh,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_bitrev,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_bvf,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_bvt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_bvfl,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_bvtl,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mfv,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mfvc,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsync2,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mtv,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_mtvc,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vadd_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vadd_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vadd_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vadd_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsub_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsub_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsub_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsub_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsbn_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdiv_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdiv_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdiv_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdiv_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmul_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmul_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmul_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmul_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdot_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdot_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdot_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vscl_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vscl_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vscl_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vhdp_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vhdp_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vhdp_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcrs_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vdet_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmp_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmin_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmin_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmin_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmin_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmax_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmax_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmax_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmax_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vscmp_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vscmp_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vscmp_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vscmp_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsge_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsge_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsge_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsge_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vslt_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vslt_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vslt_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vslt_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vwbn_s,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmov_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmov_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmov_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmov_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vabs_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vabs_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vabs_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vabs_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vneg_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vneg_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vneg_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vneg_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vidt_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vidt_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat0_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat0_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat0_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat0_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat1_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat1_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat1_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsat1_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vzero_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vzero_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vzero_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vzero_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vone_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vone_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vone_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vone_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrcp_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrcp_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrcp_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrcp_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrsq_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrsq_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrsq_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrsq_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsin_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsin_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsin_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsin_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcos_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcos_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcos_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcos_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vexp2_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vexp2_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vexp2_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vexp2_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vlog2_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vlog2_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vlog2_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vlog2_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsqrt_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsqrt_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsqrt_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsqrt_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vasin_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vasin_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vasin_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vasin_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnrcp_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnrcp_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnrcp_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnrcp_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnsin_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnsin_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnsin_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnsin_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrexp2_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrexp2_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrexp2_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrexp2_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrnds_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndi_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndi_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndi_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndi_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf1_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf1_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf1_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf1_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf2_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf2_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf2_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrndf2_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2h_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2h_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vh2f_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vh2f_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsbz_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vlgb_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vuc2ifs_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vc2i_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vus2i_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vus2i_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vs2i_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vs2i_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2uc_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2c_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2us_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2us_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2s_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2s_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsrt1_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsrt2_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vbfy1_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vbfy1_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vbfy2_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vocp_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vocp_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vocp_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vocp_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsocp_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsocp_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vfad_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vfad_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vfad_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vavg_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vavg_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vavg_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsrt3_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsrt4_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsgn_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsgn_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsgn_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsgn_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmfvc,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmtvc,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vt4444_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vt5551_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vt5650_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcst_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcst_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcst_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcst_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2in_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2in_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2in_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2in_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iz_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iz_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iz_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iz_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iu_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iu_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iu_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2iu_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2id_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2id_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2id_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vf2id_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2f_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2f_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2f_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vi2f_q,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovt_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovt_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovt_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovt_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovf_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovf_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovf_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcmovf_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vpfxs,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vpfxt,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vpfxd,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_viim_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vfim_s,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmmul_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmmul_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmmul_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vhtfm2_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vtfm2_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vhtfm3_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vtfm3_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vhtfm4_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vtfm4_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmscl_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmscl_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmscl_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vcrsp_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vqmul_q,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrot_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrot_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vrot_q,

    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmmov_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmmov_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmmov_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmidt_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmidt_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmidt_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmzero_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmzero_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmzero_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmone_p,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmone_t,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vmone_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vnop,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vsync,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_vflush,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_svl_q,
    #[cfg(feature = "R4000ALLEGREX")]
    r4000allegrex_svr_q,
    #[cfg(feature = "R5900EE")]
    r5900ee_lq,
    #[cfg(feature = "R5900EE")]
    r5900ee_sq,
    #[cfg(feature = "R5900EE")]
    r5900ee_lqc2,
    #[cfg(feature = "R5900EE")]
    r5900ee_sqc2,

    #[cfg(feature = "R5900EE")]
    r5900ee_sync_p,
    #[cfg(feature = "R5900EE")]
    r5900ee_mult,
    #[cfg(feature = "R5900EE")]
    r5900ee_mfsa,
    #[cfg(feature = "R5900EE")]
    r5900ee_mtsa,

    #[cfg(feature = "R5900EE")]
    r5900ee_mtsab,
    #[cfg(feature = "R5900EE")]
    r5900ee_mtsah,

    #[cfg(feature = "R5900EE")]
    r5900ee_madd,
    #[cfg(feature = "R5900EE")]
    r5900ee_maddu,
    #[cfg(feature = "R5900EE")]
    r5900ee_plzcw,
    #[cfg(feature = "R5900EE")]
    r5900ee_mfhi1,
    #[cfg(feature = "R5900EE")]
    r5900ee_mthi1,
    #[cfg(feature = "R5900EE")]
    r5900ee_mflo1,
    #[cfg(feature = "R5900EE")]
    r5900ee_mtlo1,
    #[cfg(feature = "R5900EE")]
    r5900ee_mult1,
    #[cfg(feature = "R5900EE")]
    r5900ee_multu1,
    #[cfg(feature = "R5900EE")]
    r5900ee_div1,
    #[cfg(feature = "R5900EE")]
    r5900ee_divu1,
    #[cfg(feature = "R5900EE")]
    r5900ee_madd1,
    #[cfg(feature = "R5900EE")]
    r5900ee_maddu1,
    #[cfg(feature = "R5900EE")]
    r5900ee_psllh,
    #[cfg(feature = "R5900EE")]
    r5900ee_psrlh,
    #[cfg(feature = "R5900EE")]
    r5900ee_psrah,
    #[cfg(feature = "R5900EE")]
    r5900ee_psllw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psrlw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psraw,

    #[cfg(feature = "R5900EE")]
    r5900ee_paddw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pcgtw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmaxw,
    #[cfg(feature = "R5900EE")]
    r5900ee_paddh,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pcgth,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmaxh,
    #[cfg(feature = "R5900EE")]
    r5900ee_paddb,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubb,
    #[cfg(feature = "R5900EE")]
    r5900ee_pcgtb,
    #[cfg(feature = "R5900EE")]
    r5900ee_paddsw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubsw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pextlw,
    #[cfg(feature = "R5900EE")]
    r5900ee_ppacw,
    #[cfg(feature = "R5900EE")]
    r5900ee_paddsh,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubsh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pextlh,
    #[cfg(feature = "R5900EE")]
    r5900ee_ppach,
    #[cfg(feature = "R5900EE")]
    r5900ee_paddsb,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubsb,
    #[cfg(feature = "R5900EE")]
    r5900ee_pextlb,
    #[cfg(feature = "R5900EE")]
    r5900ee_ppacb,
    #[cfg(feature = "R5900EE")]
    r5900ee_pext5,
    #[cfg(feature = "R5900EE")]
    r5900ee_ppac5,
    #[cfg(feature = "R5900EE")]
    r5900ee_pabsw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pceqw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pminw,
    #[cfg(feature = "R5900EE")]
    r5900ee_padsbh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pabsh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pceqh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pminh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pceqb,
    #[cfg(feature = "R5900EE")]
    r5900ee_padduw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubuw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pextuw,
    #[cfg(feature = "R5900EE")]
    r5900ee_padduh,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubuh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pextuh,
    #[cfg(feature = "R5900EE")]
    r5900ee_paddub,
    #[cfg(feature = "R5900EE")]
    r5900ee_psubub,
    #[cfg(feature = "R5900EE")]
    r5900ee_pextub,
    #[cfg(feature = "R5900EE")]
    r5900ee_qfsrv,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmaddw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psllvw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psrlvw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmsubw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmfhi,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmflo,
    #[cfg(feature = "R5900EE")]
    r5900ee_pinth,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmultw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pdivw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pcpyld,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmaddh,
    #[cfg(feature = "R5900EE")]
    r5900ee_phmadh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pand,
    #[cfg(feature = "R5900EE")]
    r5900ee_pxor,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmsubh,
    #[cfg(feature = "R5900EE")]
    r5900ee_phmsbh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pexeh,
    #[cfg(feature = "R5900EE")]
    r5900ee_prevh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmulth,
    #[cfg(feature = "R5900EE")]
    r5900ee_pdivbw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pexew,
    #[cfg(feature = "R5900EE")]
    r5900ee_prot3w,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmadduw,
    #[cfg(feature = "R5900EE")]
    r5900ee_psravw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmthi,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmtlo,
    #[cfg(feature = "R5900EE")]
    r5900ee_pinteh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmultuw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pdivuw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pcpyud,
    #[cfg(feature = "R5900EE")]
    r5900ee_por,
    #[cfg(feature = "R5900EE")]
    r5900ee_pnor,
    #[cfg(feature = "R5900EE")]
    r5900ee_pexch,
    #[cfg(feature = "R5900EE")]
    r5900ee_pcpyh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pexcw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmfhl_lw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmfhl_uw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmfhl_slw,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmfhl_lh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmfhl_sh,
    #[cfg(feature = "R5900EE")]
    r5900ee_pmthl_lw,

    #[cfg(feature = "R5900EE")]
    r5900ee_ei,
    #[cfg(feature = "R5900EE")]
    r5900ee_di,

    #[cfg(feature = "R5900EE")]
    r5900ee_c1__sqrt_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_rsqrt_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_adda_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_suba_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_mula_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_madd_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_msub_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_madda_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_msuba_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_max_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_min_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_c_lt_s,
    #[cfg(feature = "R5900EE")]
    r5900ee_c_le_s,

    #[cfg(feature = "R5900EE")]
    r5900ee_qmfc2,
    #[cfg(feature = "R5900EE")]
    r5900ee_cfc2,
    #[cfg(feature = "R5900EE")]
    r5900ee_qmtc2,
    #[cfg(feature = "R5900EE")]
    r5900ee_ctc2,

    #[cfg(feature = "R5900EE")]
    r5900ee_bc2f,
    #[cfg(feature = "R5900EE")]
    r5900ee_bc2t,
    #[cfg(feature = "R5900EE")]
    r5900ee_bc2fl,
    #[cfg(feature = "R5900EE")]
    r5900ee_bc2tl,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddx,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddy,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubx,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsuby,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddx,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddy,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubx,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsuby,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaxx,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaxy,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaxz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaxw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vminix,
    #[cfg(feature = "R5900EE")]
    r5900ee_vminiy,
    #[cfg(feature = "R5900EE")]
    r5900ee_vminiz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vminiw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulx,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmuly,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaxi,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmuli,
    #[cfg(feature = "R5900EE")]
    r5900ee_vminii,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddi,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddi,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubi,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubi,
    #[cfg(feature = "R5900EE")]
    r5900ee_vadd,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmadd,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmul,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmax,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsub,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsub,
    #[cfg(feature = "R5900EE")]
    r5900ee_vopmsub,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmini,
    #[cfg(feature = "R5900EE")]
    r5900ee_viadd,
    #[cfg(feature = "R5900EE")]
    r5900ee_visub,
    #[cfg(feature = "R5900EE")]
    r5900ee_viaddi,
    #[cfg(feature = "R5900EE")]
    r5900ee_viand,
    #[cfg(feature = "R5900EE")]
    r5900ee_vior,
    #[cfg(feature = "R5900EE")]
    r5900ee_vcallms,
    #[cfg(feature = "R5900EE")]
    r5900ee_vcallmsr,

    #[cfg(feature = "R5900EE")]
    r5900ee_vaddax,
    #[cfg(feature = "R5900EE")]
    r5900ee_vadday,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddaz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddaw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubax,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubay,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubaz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubaw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddax,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmadday,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddaz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddaw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubax,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubay,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubaz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubaw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vitof0,
    #[cfg(feature = "R5900EE")]
    r5900ee_vitof4,
    #[cfg(feature = "R5900EE")]
    r5900ee_vitof12,
    #[cfg(feature = "R5900EE")]
    r5900ee_vitof15,
    #[cfg(feature = "R5900EE")]
    r5900ee_vftoi0,
    #[cfg(feature = "R5900EE")]
    r5900ee_vftoi4,
    #[cfg(feature = "R5900EE")]
    r5900ee_vftoi12,
    #[cfg(feature = "R5900EE")]
    r5900ee_vftoi15,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulax,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulay,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulaz,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulaw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulaq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vabs,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmulai,
    #[cfg(feature = "R5900EE")]
    r5900ee_vclipw,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddaq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddaq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vaddai,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmaddai,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubaq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubaq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsubai,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsubai,
    #[cfg(feature = "R5900EE")]
    r5900ee_vadda,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmadda,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmula,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsuba,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmsuba,
    #[cfg(feature = "R5900EE")]
    r5900ee_vopmula,
    #[cfg(feature = "R5900EE")]
    r5900ee_vnop,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmove,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmr32,
    #[cfg(feature = "R5900EE")]
    r5900ee_vlqi,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsqi,
    #[cfg(feature = "R5900EE")]
    r5900ee_vlqd,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsqd,
    #[cfg(feature = "R5900EE")]
    r5900ee_vdiv,
    #[cfg(feature = "R5900EE")]
    r5900ee_vsqrt,
    #[cfg(feature = "R5900EE")]
    r5900ee_vrsqrt,
    #[cfg(feature = "R5900EE")]
    r5900ee_vwaitq,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmtir,
    #[cfg(feature = "R5900EE")]
    r5900ee_vmfir,
    #[cfg(feature = "R5900EE")]
    r5900ee_vrnext,
    #[cfg(feature = "R5900EE")]
    r5900ee_vrget,
    #[cfg(feature = "R5900EE")]
    r5900ee_vrinit,
    #[cfg(feature = "R5900EE")]
    r5900ee_vrxor,

    #[cfg(feature = "R5900EE")]
    r5900ee_vilwr_w,
    #[cfg(feature = "R5900EE")]
    r5900ee_vilwr_z,
    #[cfg(feature = "R5900EE")]
    r5900ee_vilwr_y,
    #[cfg(feature = "R5900EE")]
    r5900ee_vilwr_x,
    #[cfg(feature = "R5900EE")]
    r5900ee_viswr_w,
    #[cfg(feature = "R5900EE")]
    r5900ee_viswr_z,
    #[cfg(feature = "R5900EE")]
    r5900ee_viswr_y,
    #[cfg(feature = "R5900EE")]
    r5900ee_viswr_x,
}
