/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
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
    core_move,
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
    rsp_mfc2,
    rsp_mtc2,
    rsp_cfc2,
    rsp_ctc2,

    rsp_vmulf,
    rsp_vmulu,
    rsp_vrndp,
    rsp_vmulq,
    rsp_vmudl,
    rsp_vmudm,
    rsp_vmudn,
    rsp_vmudh,
    rsp_vmacf,
    rsp_vmacu,
    rsp_vrndn,
    rsp_vmacq,
    rsp_vmadl,
    rsp_vmadm,
    rsp_vmadn,
    rsp_vmadh,
    rsp_vadd,
    rsp_vsub,
    rsp_vabs,
    rsp_vaddc,
    rsp_vsubc,
    rsp_vsar,
    rsp_vand,
    rsp_vnand,
    rsp_vor,
    rsp_vnor,
    rsp_vxor,
    rsp_vnxor,
    rsp_vlt,
    rsp_veq,
    rsp_vne,
    rsp_vge,
    rsp_vcl,
    rsp_vch,
    rsp_vcr,
    rsp_vmrg,
    rsp_vrcp,
    rsp_vrcpl,
    rsp_vrcph,
    rsp_vmov,
    rsp_vrsq,
    rsp_vrsql,
    rsp_vrsqh,
    rsp_vnop,
    rsp_lbv,
    rsp_lsv,
    rsp_llv,
    rsp_ldv,
    rsp_lqv,
    rsp_lrv,
    rsp_lpv,
    rsp_luv,
    rsp_lhv,
    rsp_lfv,
    rsp_ltv,
    rsp_sbv,
    rsp_ssv,
    rsp_slv,
    rsp_sdv,
    rsp_sqv,
    rsp_srv,
    rsp_spv,
    rsp_suv,
    rsp_shv,
    rsp_sfv,
    rsp_stv,
    rsp_swv,

    rsp_mfc0,
    rsp_mtc0,

    r3000gte_RTPS,
    r3000gte_RTPT,
    r3000gte_DPCL,
    r3000gte_DPCS,
    r3000gte_DPCT,
    r3000gte_INTPL,
    r3000gte_NCS,
    r3000gte_NCT,
    r3000gte_NCDS,
    r3000gte_NCDT,
    r3000gte_NCCS,
    r3000gte_NCCT,
    r3000gte_CDP,
    r3000gte_CC,
    r3000gte_NCLIP,
    r3000gte_AVSZ3,
    r3000gte_AVSZ4,
    r3000gte_MVMVA,
    r3000gte_SQR,
    r3000gte_OP,
    r3000gte_GPF,
    r3000gte_GPL,
    r4000allegrex_lv_s,
    r4000allegrex_sv_s,
    r4000allegrex_lv_q,
    r4000allegrex_sv_q,
    r4000allegrex_clz,
    r4000allegrex_clo,
    r4000allegrex_madd,
    r4000allegrex_maddu,
    r4000allegrex_msub,
    r4000allegrex_msubu,
    r4000allegrex_max,
    r4000allegrex_min,
    r4000allegrex_srl,
    r4000allegrex_rotr,
    r4000allegrex_srlv,
    r4000allegrex_rotrv,
    r4000allegrex_sleep,
    r4000allegrex_mfie,
    r4000allegrex_mtie,
    r4000allegrex_ext,
    r4000allegrex_ins,
    r4000allegrex_wsbh,
    r4000allegrex_wsbw,
    r4000allegrex_seb,
    r4000allegrex_seh,
    r4000allegrex_bitrev,
    r4000allegrex_bvf,
    r4000allegrex_bvt,
    r4000allegrex_bvfl,
    r4000allegrex_bvtl,
    r4000allegrex_mfv,
    r4000allegrex_mfvc,
    r4000allegrex_vsync2,
    r4000allegrex_mtv,
    r4000allegrex_mtvc,
    r4000allegrex_vadd_s,
    r4000allegrex_vadd_p,
    r4000allegrex_vadd_t,
    r4000allegrex_vadd_q,
    r4000allegrex_vsub_s,
    r4000allegrex_vsub_p,
    r4000allegrex_vsub_t,
    r4000allegrex_vsub_q,
    r4000allegrex_vsbn_s,
    r4000allegrex_vdiv_s,
    r4000allegrex_vdiv_p,
    r4000allegrex_vdiv_t,
    r4000allegrex_vdiv_q,
    r4000allegrex_vmul_s,
    r4000allegrex_vmul_p,
    r4000allegrex_vmul_t,
    r4000allegrex_vmul_q,
    r4000allegrex_vdot_p,
    r4000allegrex_vdot_t,
    r4000allegrex_vdot_q,
    r4000allegrex_vscl_p,
    r4000allegrex_vscl_t,
    r4000allegrex_vscl_q,
    r4000allegrex_vhdp_p,
    r4000allegrex_vhdp_t,
    r4000allegrex_vhdp_q,
    r4000allegrex_vcrs_t,
    r4000allegrex_vdet_p,
    r4000allegrex_vcmp_s,
    r4000allegrex_vcmp_p,
    r4000allegrex_vcmp_t,
    r4000allegrex_vcmp_q,
    r4000allegrex_vmin_s,
    r4000allegrex_vmin_p,
    r4000allegrex_vmin_t,
    r4000allegrex_vmin_q,
    r4000allegrex_vmax_s,
    r4000allegrex_vmax_p,
    r4000allegrex_vmax_t,
    r4000allegrex_vmax_q,
    r4000allegrex_vscmp_s,
    r4000allegrex_vscmp_p,
    r4000allegrex_vscmp_t,
    r4000allegrex_vscmp_q,
    r4000allegrex_vsge_s,
    r4000allegrex_vsge_p,
    r4000allegrex_vsge_t,
    r4000allegrex_vsge_q,
    r4000allegrex_vslt_s,
    r4000allegrex_vslt_p,
    r4000allegrex_vslt_t,
    r4000allegrex_vslt_q,
    r4000allegrex_vwbn_s,
    r4000allegrex_vmov_s,
    r4000allegrex_vmov_p,
    r4000allegrex_vmov_t,
    r4000allegrex_vmov_q,
    r4000allegrex_vabs_s,
    r4000allegrex_vabs_p,
    r4000allegrex_vabs_t,
    r4000allegrex_vabs_q,
    r4000allegrex_vneg_s,
    r4000allegrex_vneg_p,
    r4000allegrex_vneg_t,
    r4000allegrex_vneg_q,
    r4000allegrex_vidt_p,
    r4000allegrex_vidt_q,
    r4000allegrex_vsat0_s,
    r4000allegrex_vsat0_p,
    r4000allegrex_vsat0_t,
    r4000allegrex_vsat0_q,
    r4000allegrex_vsat1_s,
    r4000allegrex_vsat1_p,
    r4000allegrex_vsat1_t,
    r4000allegrex_vsat1_q,
    r4000allegrex_vzero_s,
    r4000allegrex_vzero_p,
    r4000allegrex_vzero_t,
    r4000allegrex_vzero_q,
    r4000allegrex_vone_s,
    r4000allegrex_vone_p,
    r4000allegrex_vone_t,
    r4000allegrex_vone_q,
    r4000allegrex_vrcp_s,
    r4000allegrex_vrcp_p,
    r4000allegrex_vrcp_t,
    r4000allegrex_vrcp_q,
    r4000allegrex_vrsq_s,
    r4000allegrex_vrsq_p,
    r4000allegrex_vrsq_t,
    r4000allegrex_vrsq_q,
    r4000allegrex_vsin_s,
    r4000allegrex_vsin_p,
    r4000allegrex_vsin_t,
    r4000allegrex_vsin_q,
    r4000allegrex_vcos_s,
    r4000allegrex_vcos_p,
    r4000allegrex_vcos_t,
    r4000allegrex_vcos_q,
    r4000allegrex_vexp2_s,
    r4000allegrex_vexp2_p,
    r4000allegrex_vexp2_t,
    r4000allegrex_vexp2_q,
    r4000allegrex_vlog2_s,
    r4000allegrex_vlog2_p,
    r4000allegrex_vlog2_t,
    r4000allegrex_vlog2_q,
    r4000allegrex_vsqrt_s,
    r4000allegrex_vsqrt_p,
    r4000allegrex_vsqrt_t,
    r4000allegrex_vsqrt_q,
    r4000allegrex_vasin_s,
    r4000allegrex_vasin_p,
    r4000allegrex_vasin_t,
    r4000allegrex_vasin_q,
    r4000allegrex_vnrcp_s,
    r4000allegrex_vnrcp_p,
    r4000allegrex_vnrcp_t,
    r4000allegrex_vnrcp_q,
    r4000allegrex_vnsin_s,
    r4000allegrex_vnsin_p,
    r4000allegrex_vnsin_t,
    r4000allegrex_vnsin_q,
    r4000allegrex_vrexp2_s,
    r4000allegrex_vrexp2_p,
    r4000allegrex_vrexp2_t,
    r4000allegrex_vrexp2_q,
    r4000allegrex_vrnds_s,
    r4000allegrex_vrndi_s,
    r4000allegrex_vrndi_p,
    r4000allegrex_vrndi_t,
    r4000allegrex_vrndi_q,
    r4000allegrex_vrndf1_s,
    r4000allegrex_vrndf1_p,
    r4000allegrex_vrndf1_t,
    r4000allegrex_vrndf1_q,
    r4000allegrex_vrndf2_s,
    r4000allegrex_vrndf2_p,
    r4000allegrex_vrndf2_t,
    r4000allegrex_vrndf2_q,
    r4000allegrex_vf2h_p,
    r4000allegrex_vf2h_q,
    r4000allegrex_vh2f_s,
    r4000allegrex_vh2f_p,
    r4000allegrex_vsbz_s,
    r4000allegrex_vlgb_s,
    r4000allegrex_vuc2ifs_s,
    r4000allegrex_vc2i_s,
    r4000allegrex_vus2i_s,
    r4000allegrex_vus2i_p,
    r4000allegrex_vs2i_s,
    r4000allegrex_vs2i_p,
    r4000allegrex_vi2uc_q,
    r4000allegrex_vi2c_q,
    r4000allegrex_vi2us_p,
    r4000allegrex_vi2us_q,
    r4000allegrex_vi2s_p,
    r4000allegrex_vi2s_q,
    r4000allegrex_vsrt1_q,
    r4000allegrex_vsrt2_q,
    r4000allegrex_vbfy1_p,
    r4000allegrex_vbfy1_q,
    r4000allegrex_vbfy2_q,
    r4000allegrex_vocp_s,
    r4000allegrex_vocp_p,
    r4000allegrex_vocp_t,
    r4000allegrex_vocp_q,
    r4000allegrex_vsocp_s,
    r4000allegrex_vsocp_p,
    r4000allegrex_vfad_p,
    r4000allegrex_vfad_t,
    r4000allegrex_vfad_q,
    r4000allegrex_vavg_p,
    r4000allegrex_vavg_t,
    r4000allegrex_vavg_q,
    r4000allegrex_vsrt3_q,
    r4000allegrex_vsrt4_q,
    r4000allegrex_vsgn_s,
    r4000allegrex_vsgn_p,
    r4000allegrex_vsgn_t,
    r4000allegrex_vsgn_q,
    r4000allegrex_vmfvc,
    r4000allegrex_vmtvc,
    r4000allegrex_vt4444_q,
    r4000allegrex_vt5551_q,
    r4000allegrex_vt5650_q,
    r4000allegrex_vcst_s,
    r4000allegrex_vcst_p,
    r4000allegrex_vcst_t,
    r4000allegrex_vcst_q,
    r4000allegrex_vf2in_s,
    r4000allegrex_vf2in_p,
    r4000allegrex_vf2in_t,
    r4000allegrex_vf2in_q,
    r4000allegrex_vf2iz_s,
    r4000allegrex_vf2iz_p,
    r4000allegrex_vf2iz_t,
    r4000allegrex_vf2iz_q,
    r4000allegrex_vf2iu_s,
    r4000allegrex_vf2iu_p,
    r4000allegrex_vf2iu_t,
    r4000allegrex_vf2iu_q,
    r4000allegrex_vf2id_s,
    r4000allegrex_vf2id_p,
    r4000allegrex_vf2id_t,
    r4000allegrex_vf2id_q,
    r4000allegrex_vi2f_s,
    r4000allegrex_vi2f_p,
    r4000allegrex_vi2f_t,
    r4000allegrex_vi2f_q,
    r4000allegrex_vcmovt_s,
    r4000allegrex_vcmovt_p,
    r4000allegrex_vcmovt_t,
    r4000allegrex_vcmovt_q,
    r4000allegrex_vcmovf_s,
    r4000allegrex_vcmovf_p,
    r4000allegrex_vcmovf_t,
    r4000allegrex_vcmovf_q,
    r4000allegrex_vpfxs,
    r4000allegrex_vpfxt,
    r4000allegrex_vpfxd,
    r4000allegrex_viim_s,
    r4000allegrex_vfim_s,
    r4000allegrex_vmmul_p,
    r4000allegrex_vmmul_t,
    r4000allegrex_vmmul_q,
    r4000allegrex_vhtfm2_p,
    r4000allegrex_vtfm2_p,
    r4000allegrex_vhtfm3_t,
    r4000allegrex_vtfm3_t,
    r4000allegrex_vhtfm4_q,
    r4000allegrex_vtfm4_q,
    r4000allegrex_vmscl_p,
    r4000allegrex_vmscl_t,
    r4000allegrex_vmscl_q,
    r4000allegrex_vcrsp_t,
    r4000allegrex_vqmul_q,
    r4000allegrex_vrot_p,
    r4000allegrex_vrot_t,
    r4000allegrex_vrot_q,
    r4000allegrex_vmmov_p,
    r4000allegrex_vmmov_t,
    r4000allegrex_vmmov_q,
    r4000allegrex_vmidt_p,
    r4000allegrex_vmidt_t,
    r4000allegrex_vmidt_q,
    r4000allegrex_vmzero_p,
    r4000allegrex_vmzero_t,
    r4000allegrex_vmzero_q,
    r4000allegrex_vmone_p,
    r4000allegrex_vmone_t,
    r4000allegrex_vmone_q,
    r4000allegrex_vnop,
    r4000allegrex_vsync,
    r4000allegrex_vflush,
    r4000allegrex_svl_q,
    r4000allegrex_svr_q,
    r5900_lq,
    r5900_sq,
    r5900_lqc2,
    r5900_sqc2,

    r5900_sync_p,
    r5900_mult,
    r5900_mfsa,
    r5900_mtsa,

    r5900_mtsab,
    r5900_mtsah,

    r5900_madd,
    r5900_maddu,
    r5900_plzcw,
    r5900_mfhi1,
    r5900_mthi1,
    r5900_mflo1,
    r5900_mtlo1,
    r5900_mult1,
    r5900_multu1,
    r5900_div1,
    r5900_divu1,
    r5900_madd1,
    r5900_maddu1,
    r5900_psllh,
    r5900_psrlh,
    r5900_psrah,
    r5900_psllw,
    r5900_psrlw,
    r5900_psraw,

    r5900_paddw,
    r5900_psubw,
    r5900_pcgtw,
    r5900_pmaxw,
    r5900_paddh,
    r5900_psubh,
    r5900_pcgth,
    r5900_pmaxh,
    r5900_paddb,
    r5900_psubb,
    r5900_pcgtb,
    r5900_paddsw,
    r5900_psubsw,
    r5900_pextlw,
    r5900_ppacw,
    r5900_paddsh,
    r5900_psubsh,
    r5900_pextlh,
    r5900_ppach,
    r5900_paddsb,
    r5900_psubsb,
    r5900_pextlb,
    r5900_ppacb,
    r5900_pext5,
    r5900_ppac5,
    r5900_pabsw,
    r5900_pceqw,
    r5900_pminw,
    r5900_padsbh,
    r5900_pabsh,
    r5900_pceqh,
    r5900_pminh,
    r5900_pceqb,
    r5900_padduw,
    r5900_psubuw,
    r5900_pextuw,
    r5900_padduh,
    r5900_psubuh,
    r5900_pextuh,
    r5900_paddub,
    r5900_psubub,
    r5900_pextub,
    r5900_qfsrv,
    r5900_pmaddw,
    r5900_psllvw,
    r5900_psrlvw,
    r5900_pmsubw,
    r5900_pmfhi,
    r5900_pmflo,
    r5900_pinth,
    r5900_pmultw,
    r5900_pdivw,
    r5900_pcpyld,
    r5900_pmaddh,
    r5900_phmadh,
    r5900_pand,
    r5900_pxor,
    r5900_pmsubh,
    r5900_phmsbh,
    r5900_pexeh,
    r5900_prevh,
    r5900_pmulth,
    r5900_pdivbw,
    r5900_pexew,
    r5900_prot3w,
    r5900_pmadduw,
    r5900_psravw,
    r5900_pmthi,
    r5900_pmtlo,
    r5900_pinteh,
    r5900_pmultuw,
    r5900_pdivuw,
    r5900_pcpyud,
    r5900_por,
    r5900_pnor,
    r5900_pexch,
    r5900_pcpyh,
    r5900_pexcw,
    r5900_pmfhl_lw,
    r5900_pmfhl_uw,
    r5900_pmfhl_slw,
    r5900_pmfhl_lh,
    r5900_pmfhl_sh,
    r5900_pmthl_lw,

    r5900_ei,
    r5900_di,

    r5900_c1__sqrt_s,
    r5900_rsqrt_s,
    r5900_adda_s,
    r5900_suba_s,
    r5900_mula_s,
    r5900_madd_s,
    r5900_msub_s,
    r5900_madda_s,
    r5900_msuba_s,
    r5900_max_s,
    r5900_min_s,
    r5900_c_lt_s,
    r5900_c_le_s,

    r5900_qmfc2,
    r5900_cfc2,
    r5900_qmtc2,
    r5900_ctc2,

    r5900_bc2f,
    r5900_bc2t,
    r5900_bc2fl,
    r5900_bc2tl,
    r5900_vaddx,
    r5900_vaddy,
    r5900_vaddz,
    r5900_vaddw,
    r5900_vsubx,
    r5900_vsuby,
    r5900_vsubz,
    r5900_vsubw,
    r5900_vmaddx,
    r5900_vmaddy,
    r5900_vmaddz,
    r5900_vmaddw,
    r5900_vmsubx,
    r5900_vmsuby,
    r5900_vmsubz,
    r5900_vmsubw,
    r5900_vmaxx,
    r5900_vmaxy,
    r5900_vmaxz,
    r5900_vmaxw,
    r5900_vminix,
    r5900_vminiy,
    r5900_vminiz,
    r5900_vminiw,
    r5900_vmulx,
    r5900_vmuly,
    r5900_vmulz,
    r5900_vmulw,
    r5900_vmulq,
    r5900_vmaxi,
    r5900_vmuli,
    r5900_vminii,
    r5900_vaddq,
    r5900_vmaddq,
    r5900_vaddi,
    r5900_vmaddi,
    r5900_vsubq,
    r5900_vmsubq,
    r5900_vsubi,
    r5900_vmsubi,
    r5900_vadd,
    r5900_vmadd,
    r5900_vmul,
    r5900_vmax,
    r5900_vsub,
    r5900_vmsub,
    r5900_vopmsub,
    r5900_vmini,
    r5900_viadd,
    r5900_visub,
    r5900_viaddi,
    r5900_viand,
    r5900_vior,
    r5900_vcallms,
    r5900_vcallmsr,

    r5900_vaddax,
    r5900_vadday,
    r5900_vaddaz,
    r5900_vaddaw,
    r5900_vsubax,
    r5900_vsubay,
    r5900_vsubaz,
    r5900_vsubaw,
    r5900_vmaddax,
    r5900_vmadday,
    r5900_vmaddaz,
    r5900_vmaddaw,
    r5900_vmsubax,
    r5900_vmsubay,
    r5900_vmsubaz,
    r5900_vmsubaw,
    r5900_vitof0,
    r5900_vitof4,
    r5900_vitof12,
    r5900_vitof15,
    r5900_vftoi0,
    r5900_vftoi4,
    r5900_vftoi12,
    r5900_vftoi15,
    r5900_vmulax,
    r5900_vmulay,
    r5900_vmulaz,
    r5900_vmulaw,
    r5900_vmulaq,
    r5900_vabs,
    r5900_vmulai,
    r5900_vclipw,
    r5900_vaddaq,
    r5900_vmaddaq,
    r5900_vaddai,
    r5900_vmaddai,
    r5900_vsubaq,
    r5900_vmsubaq,
    r5900_vsubai,
    r5900_vmsubai,
    r5900_vadda,
    r5900_vmadda,
    r5900_vmula,
    r5900_vsuba,
    r5900_vmsuba,
    r5900_vopmula,
    r5900_vnop,
    r5900_vmove,
    r5900_vmr32,
    r5900_vlqi,
    r5900_vsqi,
    r5900_vlqd,
    r5900_vsqd,
    r5900_vdiv,
    r5900_vsqrt,
    r5900_vrsqrt,
    r5900_vwaitq,
    r5900_vmtir,
    r5900_vmfir,
    r5900_vrnext,
    r5900_vrget,
    r5900_vrinit,
    r5900_vrxor,

    r5900_vilwr_w,
    r5900_vilwr_z,
    r5900_vilwr_y,
    r5900_vilwr_x,
    r5900_viswr_w,
    r5900_viswr_z,
    r5900_viswr_y,
    r5900_viswr_x,
}
