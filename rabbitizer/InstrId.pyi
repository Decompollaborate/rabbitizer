#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

from .Enum import Enum


class InstrId:
    cpu_INVALID: Enum
    cpu_mthi: Enum
    cpu_mtlo: Enum
    cpu_jr: Enum
    cpu_jalr: Enum
    cpu_jalr_rd: Enum
    cpu_mfhi: Enum
    cpu_mflo: Enum
    cpu_mult: Enum
    cpu_multu: Enum
    cpu_dmult: Enum
    cpu_dmultu: Enum
    cpu_tge: Enum
    cpu_tgeu: Enum
    cpu_tlt: Enum
    cpu_tltu: Enum
    cpu_teq: Enum
    cpu_tne: Enum
    cpu_movz: Enum
    cpu_movn: Enum
    cpu_div: Enum
    cpu_divu: Enum
    cpu_sn64_div: Enum
    cpu_sn64_divu: Enum
    cpu_ddiv: Enum
    cpu_ddivu: Enum
    cpu_add: Enum
    cpu_addu: Enum
    cpu_sub: Enum
    cpu_subu: Enum
    cpu_and: Enum
    cpu_or: Enum
    cpu_xor: Enum
    cpu_nor: Enum
    cpu_slt: Enum
    cpu_sltu: Enum
    cpu_dadd: Enum
    cpu_daddu: Enum
    cpu_dsub: Enum
    cpu_dsubu: Enum
    cpu_syscall: Enum
    cpu_break: Enum
    cpu_sync: Enum
    cpu_dsllv: Enum
    cpu_dsrlv: Enum
    cpu_dsrav: Enum
    cpu_sllv: Enum
    cpu_srlv: Enum
    cpu_srav: Enum
    cpu_sll: Enum
    cpu_srl: Enum
    cpu_sra: Enum
    cpu_dsll: Enum
    cpu_dsrl: Enum
    cpu_dsra: Enum
    cpu_dsll32: Enum
    cpu_dsrl32: Enum
    cpu_dsra32: Enum
    cpu_bltz: Enum
    cpu_bgez: Enum
    cpu_bltzl: Enum
    cpu_bgezl: Enum
    cpu_tgei: Enum
    cpu_tgeiu: Enum
    cpu_tlti: Enum
    cpu_tltiu: Enum
    cpu_bltzal: Enum
    cpu_bgezal: Enum
    cpu_bltzall: Enum
    cpu_bgezall: Enum
    cpu_teqi: Enum
    cpu_tnei: Enum
    cpu_j: Enum
    cpu_jal: Enum
    cpu_beq: Enum
    cpu_bne: Enum
    cpu_beql: Enum
    cpu_bnel: Enum
    cpu_blez: Enum
    cpu_bgtz: Enum
    cpu_blezl: Enum
    cpu_bgtzl: Enum
    cpu_lui: Enum
    cpu_andi: Enum
    cpu_ori: Enum
    cpu_xori: Enum
    cpu_addi: Enum
    cpu_addiu: Enum
    cpu_daddi: Enum
    cpu_daddiu: Enum
    cpu_slti: Enum
    cpu_sltiu: Enum
    cpu_ldl: Enum
    cpu_ldr: Enum
    cpu_lb: Enum
    cpu_lh: Enum
    cpu_lwl: Enum
    cpu_lw: Enum
    cpu_lbu: Enum
    cpu_lhu: Enum
    cpu_lwr: Enum
    cpu_lwu: Enum
    cpu_sb: Enum
    cpu_sh: Enum
    cpu_swl: Enum
    cpu_sw: Enum
    cpu_sdl: Enum
    cpu_sdr: Enum
    cpu_swr: Enum
    cpu_ll: Enum
    cpu_pref: Enum
    cpu_lld: Enum
    cpu_ld: Enum
    cpu_sc: Enum
    cpu_scd: Enum
    cpu_sd: Enum
    cpu_lwc1: Enum
    cpu_ldc1: Enum
    cpu_swc1: Enum
    cpu_sdc1: Enum
    cpu_lwc2: Enum
    cpu_ldc2: Enum
    cpu_swc2: Enum
    cpu_sdc2: Enum
    cpu_mfc0: Enum
    cpu_dmfc0: Enum
    cpu_cfc0: Enum
    cpu_mtc0: Enum
    cpu_dmtc0: Enum
    cpu_ctc0: Enum
    cpu_tlbr: Enum
    cpu_tlbwi: Enum
    cpu_tlbwr: Enum
    cpu_tlbp: Enum
    cpu_eret: Enum
    cpu_bc0t: Enum
    cpu_bc0f: Enum
    cpu_bc0tl: Enum
    cpu_bc0fl: Enum
    cpu_mfc1: Enum
    cpu_dmfc1: Enum
    cpu_mtc1: Enum
    cpu_dmtc1: Enum
    cpu_cfc1: Enum
    cpu_ctc1: Enum
    cpu_bc1f: Enum
    cpu_bc1t: Enum
    cpu_bc1fl: Enum
    cpu_bc1tl: Enum
    cpu_add_s: Enum
    cpu_sub_s: Enum
    cpu_mul_s: Enum
    cpu_div_s: Enum
    cpu_add_d: Enum
    cpu_sub_d: Enum
    cpu_mul_d: Enum
    cpu_div_d: Enum
    cpu_sqrt_s: Enum
    cpu_abs_s: Enum
    cpu_mov_s: Enum
    cpu_neg_s: Enum
    cpu_sqrt_d: Enum
    cpu_abs_d: Enum
    cpu_mov_d: Enum
    cpu_neg_d: Enum
    cpu_round_l_s: Enum
    cpu_trunc_l_s: Enum
    cpu_ceil_l_s: Enum
    cpu_floor_l_s: Enum
    cpu_round_l_d: Enum
    cpu_trunc_l_d: Enum
    cpu_ceil_l_d: Enum
    cpu_floor_l_d: Enum
    cpu_round_w_s: Enum
    cpu_trunc_w_s: Enum
    cpu_ceil_w_s: Enum
    cpu_floor_w_s: Enum
    cpu_round_w_d: Enum
    cpu_trunc_w_d: Enum
    cpu_ceil_w_d: Enum
    cpu_floor_w_d: Enum
    cpu_c_f_s: Enum
    cpu_c_un_s: Enum
    cpu_c_eq_s: Enum
    cpu_c_ueq_s: Enum
    cpu_c_olt_s: Enum
    cpu_c_ult_s: Enum
    cpu_c_ole_s: Enum
    cpu_c_ule_s: Enum
    cpu_c_f_d: Enum
    cpu_c_un_d: Enum
    cpu_c_eq_d: Enum
    cpu_c_ueq_d: Enum
    cpu_c_olt_d: Enum
    cpu_c_ult_d: Enum
    cpu_c_ole_d: Enum
    cpu_c_ule_d: Enum
    cpu_c_sf_s: Enum
    cpu_c_ngle_s: Enum
    cpu_c_seq_s: Enum
    cpu_c_ngl_s: Enum
    cpu_c_lt_s: Enum
    cpu_c_nge_s: Enum
    cpu_c_le_s: Enum
    cpu_c_ngt_s: Enum
    cpu_c_sf_d: Enum
    cpu_c_ngle_d: Enum
    cpu_c_seq_d: Enum
    cpu_c_ngl_d: Enum
    cpu_c_lt_d: Enum
    cpu_c_nge_d: Enum
    cpu_c_le_d: Enum
    cpu_c_ngt_d: Enum
    cpu_cvt_s_d: Enum
    cpu_cvt_s_w: Enum
    cpu_cvt_s_l: Enum
    cpu_cvt_d_s: Enum
    cpu_cvt_d_w: Enum
    cpu_cvt_d_l: Enum
    cpu_cvt_w_s: Enum
    cpu_cvt_w_d: Enum
    cpu_cvt_l_s: Enum
    cpu_cvt_l_d: Enum
    cpu_nop: Enum
    cpu_beqz: Enum
    cpu_bnez: Enum
    cpu_b: Enum
    cpu_move: Enum
    cpu_not: Enum
    cpu_negu: Enum
    cpu_MAX: Enum

    rsp_INVALID: Enum
    rsp_vmulf: Enum
    rsp_vmulu: Enum
    rsp_vrndp: Enum
    rsp_vmulq: Enum
    rsp_vmudl: Enum
    rsp_vmudm: Enum
    rsp_vmudn: Enum
    rsp_vmudh: Enum
    rsp_vmacf: Enum
    rsp_vmacu: Enum
    rsp_vrndn: Enum
    rsp_vmacq: Enum
    rsp_vmadl: Enum
    rsp_vmadm: Enum
    rsp_vmadn: Enum
    rsp_vmadh: Enum
    rsp_vadd: Enum
    rsp_vsub: Enum
    rsp_vabs: Enum
    rsp_vaddc: Enum
    rsp_vsubc: Enum
    rsp_vsar: Enum
    rsp_vand: Enum
    rsp_vnand: Enum
    rsp_vor: Enum
    rsp_vnor: Enum
    rsp_vxor: Enum
    rsp_vnxor: Enum
    rsp_vlt: Enum
    rsp_veq: Enum
    rsp_vne: Enum
    rsp_vge: Enum
    rsp_vcl: Enum
    rsp_vch: Enum
    rsp_vcr: Enum
    rsp_vmrg: Enum
    rsp_vrcp: Enum
    rsp_vrcpl: Enum
    rsp_vrcph: Enum
    rsp_vmov: Enum
    rsp_vrsq: Enum
    rsp_vrsql: Enum
    rsp_vrsqh: Enum
    rsp_vnop: Enum
    rsp_mfc2: Enum
    rsp_mtc2: Enum
    rsp_cfc2: Enum
    rsp_ctc2: Enum
    rsp_sbv: Enum
    rsp_ssv: Enum
    rsp_slv: Enum
    rsp_sdv: Enum
    rsp_sqv: Enum
    rsp_srv: Enum
    rsp_spv: Enum
    rsp_suv: Enum
    rsp_swv: Enum
    rsp_shv: Enum
    rsp_sfv: Enum
    rsp_stv: Enum
    rsp_lbv: Enum
    rsp_lsv: Enum
    rsp_llv: Enum
    rsp_ldv: Enum
    rsp_lqv: Enum
    rsp_lrv: Enum
    rsp_lpv: Enum
    rsp_luv: Enum
    rsp_lhv: Enum
    rsp_lfv: Enum
    rsp_ltv: Enum
    rsp_jr: Enum
    rsp_jalr: Enum
    rsp_jalr_rd: Enum
    rsp_movz: Enum
    rsp_movn: Enum
    rsp_add: Enum
    rsp_addu: Enum
    rsp_sub: Enum
    rsp_subu: Enum
    rsp_and: Enum
    rsp_or: Enum
    rsp_xor: Enum
    rsp_nor: Enum
    rsp_slt: Enum
    rsp_sltu: Enum
    rsp_break: Enum
    rsp_sllv: Enum
    rsp_srlv: Enum
    rsp_srav: Enum
    rsp_sll: Enum
    rsp_srl: Enum
    rsp_sra: Enum
    rsp_bltz: Enum
    rsp_bgez: Enum
    rsp_bltzal: Enum
    rsp_bgezal: Enum
    rsp_j: Enum
    rsp_jal: Enum
    rsp_beq: Enum
    rsp_bne: Enum
    rsp_blez: Enum
    rsp_bgtz: Enum
    rsp_lui: Enum
    rsp_andi: Enum
    rsp_ori: Enum
    rsp_xori: Enum
    rsp_addi: Enum
    rsp_addiu: Enum
    rsp_slti: Enum
    rsp_sltiu: Enum
    rsp_lb: Enum
    rsp_lh: Enum
    rsp_lw: Enum
    rsp_lbu: Enum
    rsp_lhu: Enum
    rsp_sb: Enum
    rsp_sh: Enum
    rsp_sw: Enum
    rsp_pref: Enum
    rsp_lwc1: Enum
    rsp_swc1: Enum
    rsp_mfc0: Enum
    rsp_mtc0: Enum
    rsp_nop: Enum
    rsp_beqz: Enum
    rsp_bnez: Enum
    rsp_b: Enum
    rsp_move: Enum
    rsp_not: Enum
    rsp_negu: Enum
    rsp_MAX: Enum

    r5900_INVALID: Enum
    r5900_lq: Enum
    r5900_sq: Enum
    #r5900_lqc2: Enum
    #r5900_sqc2: Enum
    r5900_sync_p: Enum
    r5900_mfsa: Enum
    r5900_mtsa: Enum
    r5900_mtsab: Enum
    r5900_mtsah: Enum
    r5900_madd: Enum
    r5900_maddu: Enum
    r5900_plzcw: Enum
    r5900_mfhi1: Enum
    r5900_mthi1: Enum
    r5900_mflo1: Enum
    r5900_mtlo1: Enum
    r5900_mult1: Enum
    r5900_multu1: Enum
    r5900_div1: Enum
    r5900_divu1: Enum
    r5900_madd1: Enum
    r5900_maddu1: Enum
    r5900_pmfhl: Enum
    r5900_pmthl: Enum
    r5900_psllh: Enum
    r5900_psrlh: Enum
    r5900_psrah: Enum
    r5900_psllw: Enum
    r5900_psrlw: Enum
    r5900_psraw: Enum
    r5900_paddw: Enum
    r5900_psubw: Enum
    r5900_pcgtw: Enum
    r5900_pmaxw: Enum
    r5900_paddh: Enum
    r5900_psubh: Enum
    r5900_pcgth: Enum
    r5900_pmaxh: Enum
    r5900_paddb: Enum
    r5900_psubb: Enum
    r5900_pcgtb: Enum
    r5900_paddsw: Enum
    r5900_psubsw: Enum
    r5900_pextlw: Enum
    r5900_ppacw: Enum
    r5900_paddsh: Enum
    r5900_psubsh: Enum
    r5900_pextlh: Enum
    r5900_ppach: Enum
    r5900_paddsb: Enum
    r5900_psubsb: Enum
    r5900_pextlb: Enum
    r5900_ppacb: Enum
    r5900_pext5: Enum
    r5900_ppac5: Enum
    r5900_pabsw: Enum
    r5900_pceqw: Enum
    r5900_pminw: Enum
    r5900_padsbh: Enum
    r5900_pabsh: Enum
    r5900_pceqh: Enum
    r5900_pminh: Enum
    r5900_pceqb: Enum
    r5900_padduw: Enum
    r5900_psubuw: Enum
    r5900_pextuw: Enum
    r5900_padduh: Enum
    r5900_psubuh: Enum
    r5900_pextuh: Enum
    r5900_paddub: Enum
    r5900_psubub: Enum
    r5900_pextub: Enum
    r5900_qfsrv: Enum
    r5900_pmaddw: Enum
    r5900_psllvw: Enum
    r5900_psrlvw: Enum
    r5900_pmsubw: Enum
    r5900_pmfhi: Enum
    r5900_pmflo: Enum
    r5900_pinth: Enum
    r5900_pmultw: Enum
    r5900_pdivw: Enum
    r5900_pcpyld: Enum
    r5900_pmaddh: Enum
    r5900_phmadh: Enum
    r5900_pand: Enum
    r5900_pxor: Enum
    r5900_pmsubh: Enum
    r5900_phmsbh: Enum
    r5900_pexeh: Enum
    r5900_prevh: Enum
    r5900_pmulth: Enum
    r5900_pdivbw: Enum
    r5900_pexew: Enum
    r5900_prot3w: Enum
    r5900_pmadduw: Enum
    r5900_psravw: Enum
    r5900_pmthi: Enum
    r5900_pmtlo: Enum
    r5900_pinteh: Enum
    r5900_pmultuw: Enum
    r5900_pdivuw: Enum
    r5900_pcpyud: Enum
    r5900_por: Enum
    r5900_pnor: Enum
    r5900_pexch: Enum
    r5900_pcpyh: Enum
    r5900_pexcw: Enum
    r5900_ei: Enum
    r5900_di: Enum
    r5900_rsqrt_s: Enum
    r5900_adda_s: Enum
    r5900_suba_s: Enum
    r5900_mula_s: Enum
    r5900_madd_s: Enum
    r5900_msub_s: Enum
    r5900_madda_s: Enum
    r5900_msuba_s: Enum
    r5900_max_s: Enum
    r5900_min_s: Enum
    #r5900_qmfc2: Enum
    #r5900_cfc2: Enum
    #r5900_qmtc2: Enum
    #r5900_ctc2: Enum
    #r5900_bc2f: Enum
    #r5900_bc2t: Enum
    #r5900_bc2fl: Enum
    #r5900_bc2tl: Enum
    #r5900_vaddx: Enum
    #r5900_vaddy: Enum
    #r5900_vaddz: Enum
    #r5900_vaddw: Enum
    #r5900_vsubx: Enum
    #r5900_vsuby: Enum
    #r5900_vsubz: Enum
    #r5900_vsubw: Enum
    #r5900_vmaddx: Enum
    #r5900_vmaddy: Enum
    #r5900_vmaddz: Enum
    #r5900_vmaddw: Enum
    #r5900_vmsubx: Enum
    #r5900_vmsuby: Enum
    #r5900_vmsubz: Enum
    #r5900_vmsubw: Enum
    #r5900_vmaxx: Enum
    #r5900_vmaxy: Enum
    #r5900_vmaxz: Enum
    #r5900_vmaxw: Enum
    #r5900_vminix: Enum
    #r5900_vminiy: Enum
    #r5900_vminiz: Enum
    #r5900_vminiw: Enum
    #r5900_vmulx: Enum
    #r5900_vmuly: Enum
    #r5900_vmulz: Enum
    #r5900_vmulw: Enum
    #r5900_vmulq: Enum
    #r5900_vmaxi: Enum
    #r5900_vmuli: Enum
    #r5900_vminii: Enum
    #r5900_vaddq: Enum
    #r5900_vmaddq: Enum
    #r5900_vaddi: Enum
    #r5900_vmaddi: Enum
    #r5900_vsubq: Enum
    #r5900_vmsubq: Enum
    #r5900_vsubi: Enum
    #r5900_vmsubi: Enum
    #r5900_vadd: Enum
    #r5900_vmadd: Enum
    #r5900_vmul: Enum
    #r5900_vmax: Enum
    #r5900_vsub: Enum
    #r5900_vmsub: Enum
    #r5900_vopmsub: Enum
    #r5900_vmini: Enum
    #r5900_viadd: Enum
    #r5900_visub: Enum
    #r5900_viaddi: Enum
    #r5900_viand: Enum
    #r5900_vior: Enum
    #r5900_vcallms: Enum
    #r5900_callmsr: Enum
    #r5900_vaddax: Enum
    #r5900_vadday: Enum
    #r5900_vaddaz: Enum
    #r5900_vaddaw: Enum
    #r5900_vsubax: Enum
    #r5900_vsubay: Enum
    #r5900_vsubaz: Enum
    #r5900_vsubaw: Enum
    #r5900_vvmaddx: Enum
    #r5900_vvmaddy: Enum
    #r5900_vvmaddz: Enum
    #r5900_vvmaddw: Enum
    #r5900_vmsubax: Enum
    #r5900_vmsubay: Enum
    #r5900_vmsubaz: Enum
    #r5900_vmsubaw: Enum
    #r5900_vitof0: Enum
    #r5900_vitof4: Enum
    #r5900_vitof12: Enum
    #r5900_vitof15: Enum
    #r5900_vftoi0: Enum
    #r5900_vftoi4: Enum
    #r5900_vftoi12: Enum
    #r5900_vftoi15: Enum
    #r5900_vmulax: Enum
    #r5900_vmulay: Enum
    #r5900_vmulaz: Enum
    #r5900_vmulaw: Enum
    #r5900_vmulaq: Enum
    #r5900_vabs: Enum
    #r5900_vmulai: Enum
    #r5900_vclipw: Enum
    #r5900_vaddaq: Enum
    #r5900_vmaddaq: Enum
    #r5900_vaddai: Enum
    #r5900_vmaddai: Enum
    #r5900_vsubaq: Enum
    #r5900_vmsubaq: Enum
    #r5900_vsubai: Enum
    #r5900_vmsubai: Enum
    #r5900_vadda: Enum
    #r5900_vmadda: Enum
    #r5900_vmula: Enum
    #r5900_vsuba: Enum
    #r5900_vmsuba: Enum
    #r5900_vopmula: Enum
    #r5900_vnop: Enum
    #r5900_vmove: Enum
    #r5900_vmr32: Enum
    #r5900_vlqi: Enum
    #r5900_vsqi: Enum
    #r5900_vlqd: Enum
    #r5900_vsqd: Enum
    #r5900_vdiv: Enum
    #r5900_vsqrt: Enum
    #r5900_vrsqrt: Enum
    #r5900_vwaitq: Enum
    #r5900_vmtir: Enum
    #r5900_vmfir: Enum
    #r5900_vilwr: Enum
    #r5900_viswr: Enum
    #r5900_vrnext: Enum
    #r5900_vrget: Enum
    #r5900_vrinit: Enum
    #r5900_vrxor: Enum
    r5900_MAX: Enum

    ALL_MAX: Enum
