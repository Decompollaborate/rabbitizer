/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRID_H
#define RABBITIZER_INSTRID_H
#pragma once

typedef enum RabbitizerInstrCpuId {
    RABBITIZER_INSTR_CPU_ID_INVALID,

    RABBITIZER_INSTR_CPU_ID_sll,       // Shift word Left Logical

    RABBITIZER_INSTR_CPU_ID_srl,       // Shift word Right Logical
    RABBITIZER_INSTR_CPU_ID_sra,       // Shift word Right Arithmetic
    RABBITIZER_INSTR_CPU_ID_sllv,      // Shift word Left Logical Variable

    RABBITIZER_INSTR_CPU_ID_srlv,      // Shift word Right Logical Variable
    RABBITIZER_INSTR_CPU_ID_srav,      // Shift word Right Arithmetic Variable

    RABBITIZER_INSTR_CPU_ID_jr,        // Jump Register
    RABBITIZER_INSTR_CPU_ID_jalr,      // Jump And Link Register
    RABBITIZER_INSTR_CPU_ID_jalr_rd,   // Jump And Link Register // Special case for rd != 31
    RABBITIZER_INSTR_CPU_ID_movz,      // MOVe conditional on Zero
    RABBITIZER_INSTR_CPU_ID_movn,      // MOVe conditional on Not zero
    RABBITIZER_INSTR_CPU_ID_syscall,   // SYStem CALL
    RABBITIZER_INSTR_CPU_ID_break,     // Break

    RABBITIZER_INSTR_CPU_ID_sync,      // Sync

    RABBITIZER_INSTR_CPU_ID_mfhi,      // Move From HI register
    RABBITIZER_INSTR_CPU_ID_mthi,      // Move To HI register
    RABBITIZER_INSTR_CPU_ID_mflo,      // Move From LO register
    RABBITIZER_INSTR_CPU_ID_mtlo,      // Move To LO register
    RABBITIZER_INSTR_CPU_ID_dsllv,     // Doubleword Shift Left Logical Variable

    RABBITIZER_INSTR_CPU_ID_dsrlv,     // Doubleword Shift Right Logical Variable
    RABBITIZER_INSTR_CPU_ID_dsrav,     // Doubleword Shift Right Arithmetic Variable

    RABBITIZER_INSTR_CPU_ID_mult,      // MULTtiply word
    RABBITIZER_INSTR_CPU_ID_multu,     // MULTtiply Unsigned word
    RABBITIZER_INSTR_CPU_ID_div,       // DIVide word
    RABBITIZER_INSTR_CPU_ID_divu,      // DIVide Unsigned word
    RABBITIZER_INSTR_CPU_ID_dmult,     // Doubleword MULTiply
    RABBITIZER_INSTR_CPU_ID_dmultu,    // Doubleword MULTiply Unsigned
    RABBITIZER_INSTR_CPU_ID_ddiv,      // Doubleword DIVide
    RABBITIZER_INSTR_CPU_ID_ddivu,     // Doubleword DIVide Unsigned

    RABBITIZER_INSTR_CPU_ID_sn64_div,  // DIVide word
    RABBITIZER_INSTR_CPU_ID_sn64_divu, // DIVide Unsigned word

    RABBITIZER_INSTR_CPU_ID_add,       // ADD word
    RABBITIZER_INSTR_CPU_ID_addu,      // ADD Unsigned word
    RABBITIZER_INSTR_CPU_ID_sub,       // Subtract word
    RABBITIZER_INSTR_CPU_ID_subu,      // SUBtract Unsigned word
    RABBITIZER_INSTR_CPU_ID_and,       // AND
    RABBITIZER_INSTR_CPU_ID_or,        // OR
    RABBITIZER_INSTR_CPU_ID_xor,       // eXclusive OR
    RABBITIZER_INSTR_CPU_ID_nor,       // Not OR

    RABBITIZER_INSTR_CPU_ID_slt,       // Set on Less Than
    RABBITIZER_INSTR_CPU_ID_sltu,      // Set on Less Than Unsigned
    RABBITIZER_INSTR_CPU_ID_dadd,      // Doubleword Add
    RABBITIZER_INSTR_CPU_ID_daddu,     // Doubleword Add Unsigned
    RABBITIZER_INSTR_CPU_ID_dsub,      // Doubleword SUBtract
    RABBITIZER_INSTR_CPU_ID_dsubu,     // Doubleword SUBtract Unsigned

    RABBITIZER_INSTR_CPU_ID_tge,       // Trap if Greater or Equal
    RABBITIZER_INSTR_CPU_ID_tgeu,      // Trap if Greater or Equal Unsigned
    RABBITIZER_INSTR_CPU_ID_tlt,       // Trap if Less Than
    RABBITIZER_INSTR_CPU_ID_tltu,      // Trap if Less Than Unsigned
    RABBITIZER_INSTR_CPU_ID_teq,       // Trap if EQual

    RABBITIZER_INSTR_CPU_ID_tne,       // Trap if Not Equal

    RABBITIZER_INSTR_CPU_ID_dsll,      // Doubleword Shift Left Logical

    RABBITIZER_INSTR_CPU_ID_dsrl,      // Doubleword Shift Right Logical
    RABBITIZER_INSTR_CPU_ID_dsra,      // Doubleword Shift Right Arithmetic
    RABBITIZER_INSTR_CPU_ID_dsll32,    // Doubleword Shift Left Logical plus 32

    RABBITIZER_INSTR_CPU_ID_dsrl32,    // Doubleword Shift Right Logical plus 32
    RABBITIZER_INSTR_CPU_ID_dsra32,    // Doubleword Shift Right Arithmetic plus 32

    RABBITIZER_INSTR_CPU_ID_bltz,      // Branch on Less Than Zero
    RABBITIZER_INSTR_CPU_ID_bgez,      // Branch on Greater than or Equal to Zero
    RABBITIZER_INSTR_CPU_ID_bltzl,     // Branch on Less Than Zero Likely
    RABBITIZER_INSTR_CPU_ID_bgezl,     // Branch on Greater than or Equal to Zero Likely

    RABBITIZER_INSTR_CPU_ID_tgei,
    RABBITIZER_INSTR_CPU_ID_tgeiu,
    RABBITIZER_INSTR_CPU_ID_tlti,
    RABBITIZER_INSTR_CPU_ID_tltiu,

    RABBITIZER_INSTR_CPU_ID_bltzal,
    RABBITIZER_INSTR_CPU_ID_bgezal,
    RABBITIZER_INSTR_CPU_ID_bltzall,
    RABBITIZER_INSTR_CPU_ID_bgezall,

    RABBITIZER_INSTR_CPU_ID_teqi,
    RABBITIZER_INSTR_CPU_ID_tnei,

    RABBITIZER_INSTR_CPU_ID_j,         // Jump
    RABBITIZER_INSTR_CPU_ID_jal,       // Jump And Link
    RABBITIZER_INSTR_CPU_ID_beq,       // Branch on EQual
    RABBITIZER_INSTR_CPU_ID_bne,       // Branch on Not Equal
    RABBITIZER_INSTR_CPU_ID_blez,      // Branch on Less than or Equal to Zero
    RABBITIZER_INSTR_CPU_ID_bgtz,      // Branch on Greater Than Zero

    RABBITIZER_INSTR_CPU_ID_addi,      // Add Immediate
    RABBITIZER_INSTR_CPU_ID_addiu,     // Add Immediate Unsigned Word
    RABBITIZER_INSTR_CPU_ID_slti,      // Set on Less Than Immediate
    RABBITIZER_INSTR_CPU_ID_sltiu,     // Set on Less Than Immediate Unsigned
    RABBITIZER_INSTR_CPU_ID_andi,      // And Immediate
    RABBITIZER_INSTR_CPU_ID_ori,       // Or Immediate
    RABBITIZER_INSTR_CPU_ID_xori,      // eXclusive OR Immediate
    RABBITIZER_INSTR_CPU_ID_lui,       // Load Upper Immediate

    RABBITIZER_INSTR_CPU_ID_mfc0,      // Move word From CP0
    RABBITIZER_INSTR_CPU_ID_dmfc0,     // Doubleword Move From CP0
    RABBITIZER_INSTR_CPU_ID_cfc0,      // Move control word From CP0

    RABBITIZER_INSTR_CPU_ID_mtc0,      // Move word to CP0
    RABBITIZER_INSTR_CPU_ID_dmtc0,     // Doubleword Move To CP0
    RABBITIZER_INSTR_CPU_ID_ctc0,      // Move control word To CP0

    RABBITIZER_INSTR_CPU_ID_tlbr,      // Read Indexed TLB Entry
    RABBITIZER_INSTR_CPU_ID_tlbwi,     // Write Indexed TLB Entry
    RABBITIZER_INSTR_CPU_ID_tlbwr,     // Write Random TLB Entry
    RABBITIZER_INSTR_CPU_ID_tlbp,      // Probe TLB for Matching Entry
    RABBITIZER_INSTR_CPU_ID_eret,      // Return from Exception

    RABBITIZER_INSTR_CPU_ID_bc0t,      // Branch on FP True
    RABBITIZER_INSTR_CPU_ID_bc0f,      // Branch on FP False
    RABBITIZER_INSTR_CPU_ID_bc0tl,     // Branch on FP True Likely
    RABBITIZER_INSTR_CPU_ID_bc0fl,     // Branch on FP False Likely

    RABBITIZER_INSTR_CPU_ID_mfc1,      // Move Word From Floating-Point
    RABBITIZER_INSTR_CPU_ID_dmfc1,     // Doubleword Move From Floating-Point
    RABBITIZER_INSTR_CPU_ID_cfc1,      // Move Control Word from Floating-Point

    RABBITIZER_INSTR_CPU_ID_mtc1,      // Move Word to Floating-Point
    RABBITIZER_INSTR_CPU_ID_dmtc1,     // Doubleword Move To Floating-Point
    RABBITIZER_INSTR_CPU_ID_ctc1,      // Move Control Word to Floating-Point

    RABBITIZER_INSTR_CPU_ID_bc1f,
    RABBITIZER_INSTR_CPU_ID_bc1t,
    RABBITIZER_INSTR_CPU_ID_bc1fl,
    RABBITIZER_INSTR_CPU_ID_bc1tl,
    RABBITIZER_INSTR_CPU_ID_add_s,     // Floating-Point Add
    RABBITIZER_INSTR_CPU_ID_sub_s,     // Floating-Point Sub
    RABBITIZER_INSTR_CPU_ID_mul_s,     // Floating-Point Multiply
    RABBITIZER_INSTR_CPU_ID_div_s,     // Floating-Point Divide
    RABBITIZER_INSTR_CPU_ID_sqrt_s,    // Floating-Point Square Root
    RABBITIZER_INSTR_CPU_ID_abs_s,     // Floating-Point Absolute Value
    RABBITIZER_INSTR_CPU_ID_mov_s,     // Floating-Point Move
    RABBITIZER_INSTR_CPU_ID_neg_s,     // Floating-Point Negate
    RABBITIZER_INSTR_CPU_ID_round_l_s, // Floating-Point Round to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_trunc_l_s, // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_ceil_l_s,  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_floor_l_s, // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_round_w_s, // Floating-Point Round to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_trunc_w_s, // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_ceil_w_s,  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_floor_w_s, // Floating-Point Floor Convert to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_cvt_d_s,
    RABBITIZER_INSTR_CPU_ID_cvt_w_s,
    RABBITIZER_INSTR_CPU_ID_cvt_l_s,
    RABBITIZER_INSTR_CPU_ID_c_f_s,
    RABBITIZER_INSTR_CPU_ID_c_un_s,
    RABBITIZER_INSTR_CPU_ID_c_eq_s,
    RABBITIZER_INSTR_CPU_ID_c_ueq_s,
    RABBITIZER_INSTR_CPU_ID_c_olt_s,
    RABBITIZER_INSTR_CPU_ID_c_ult_s,
    RABBITIZER_INSTR_CPU_ID_c_ole_s,
    RABBITIZER_INSTR_CPU_ID_c_ule_s,
    RABBITIZER_INSTR_CPU_ID_c_sf_s,
    RABBITIZER_INSTR_CPU_ID_c_ngle_s,
    RABBITIZER_INSTR_CPU_ID_c_seq_s,
    RABBITIZER_INSTR_CPU_ID_c_ngl_s,
    RABBITIZER_INSTR_CPU_ID_c_lt_s,
    RABBITIZER_INSTR_CPU_ID_c_nge_s,
    RABBITIZER_INSTR_CPU_ID_c_le_s,
    RABBITIZER_INSTR_CPU_ID_c_ngt_s,
    RABBITIZER_INSTR_CPU_ID_add_d,     // Floating-Point Add
    RABBITIZER_INSTR_CPU_ID_sub_d,     // Floating-Point Sub
    RABBITIZER_INSTR_CPU_ID_mul_d,     // Floating-Point Multiply
    RABBITIZER_INSTR_CPU_ID_div_d,     // Floating-Point Divide
    RABBITIZER_INSTR_CPU_ID_sqrt_d,    // Floating-Point Square Root
    RABBITIZER_INSTR_CPU_ID_abs_d,     // Floating-Point Absolute Value
    RABBITIZER_INSTR_CPU_ID_mov_d,     // Floating-Point Move
    RABBITIZER_INSTR_CPU_ID_neg_d,     // Floating-Point Negate
    RABBITIZER_INSTR_CPU_ID_round_l_d, // Floating-Point Round to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_trunc_l_d, // Floating-Point Truncate to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_ceil_l_d,  // Floating-Point Ceiling Convert to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_floor_l_d, // Floating-Point Floor Convert to Long Fixed-Point
    RABBITIZER_INSTR_CPU_ID_round_w_d, // Floating-Point Round to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_trunc_w_d, // Floating-Point Truncate to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_ceil_w_d,  // Floating-Point Ceiling Convert to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_floor_w_d, // Floating-Point Floor Convert to Word Fixed-Point
    RABBITIZER_INSTR_CPU_ID_cvt_s_d,
    RABBITIZER_INSTR_CPU_ID_cvt_w_d,
    RABBITIZER_INSTR_CPU_ID_cvt_l_d,
    RABBITIZER_INSTR_CPU_ID_c_f_d,
    RABBITIZER_INSTR_CPU_ID_c_un_d,
    RABBITIZER_INSTR_CPU_ID_c_eq_d,
    RABBITIZER_INSTR_CPU_ID_c_ueq_d,
    RABBITIZER_INSTR_CPU_ID_c_olt_d,
    RABBITIZER_INSTR_CPU_ID_c_ult_d,
    RABBITIZER_INSTR_CPU_ID_c_ole_d,
    RABBITIZER_INSTR_CPU_ID_c_ule_d,
    RABBITIZER_INSTR_CPU_ID_c_sf_d,
    RABBITIZER_INSTR_CPU_ID_c_ngle_d,
    RABBITIZER_INSTR_CPU_ID_c_seq_d,
    RABBITIZER_INSTR_CPU_ID_c_ngl_d,
    RABBITIZER_INSTR_CPU_ID_c_lt_d,
    RABBITIZER_INSTR_CPU_ID_c_nge_d,
    RABBITIZER_INSTR_CPU_ID_c_le_d,
    RABBITIZER_INSTR_CPU_ID_c_ngt_d,
    RABBITIZER_INSTR_CPU_ID_cvt_s_w,
    RABBITIZER_INSTR_CPU_ID_cvt_d_w,
    RABBITIZER_INSTR_CPU_ID_cvt_s_l,
    RABBITIZER_INSTR_CPU_ID_cvt_d_l,

    RABBITIZER_INSTR_CPU_ID_beql,      // Branch on EQual Likely
    RABBITIZER_INSTR_CPU_ID_bnel,      // Branch on Not Equal Likely
    RABBITIZER_INSTR_CPU_ID_blezl,     // Branch on Less than or Equal to Zero Likely
    RABBITIZER_INSTR_CPU_ID_bgtzl,     // Branch on Greater Than Zero Likely

    RABBITIZER_INSTR_CPU_ID_daddi,     // Doubleword add Immediate
    RABBITIZER_INSTR_CPU_ID_daddiu,    // Doubleword add Immediate Unsigned
    RABBITIZER_INSTR_CPU_ID_ldl,       // Load Doubleword Left
    RABBITIZER_INSTR_CPU_ID_ldr,       // Load Doubleword Right

    RABBITIZER_INSTR_CPU_ID_lb,        // Load Byte
    RABBITIZER_INSTR_CPU_ID_lh,        // Load Halfword
    RABBITIZER_INSTR_CPU_ID_lwl,       // Load Word Left
    RABBITIZER_INSTR_CPU_ID_lw,        // Load Word
    RABBITIZER_INSTR_CPU_ID_lbu,       // Load Byte Insigned
    RABBITIZER_INSTR_CPU_ID_lhu,       // Load Halfword Unsigned
    RABBITIZER_INSTR_CPU_ID_lwr,       // Load Word Right
    RABBITIZER_INSTR_CPU_ID_lwu,       // Load Word Unsigned

    RABBITIZER_INSTR_CPU_ID_sb,        // Store Byte
    RABBITIZER_INSTR_CPU_ID_sh,        // Store Halfword
    RABBITIZER_INSTR_CPU_ID_swl,       // Store Word Left
    RABBITIZER_INSTR_CPU_ID_sw,        // Store Word
    RABBITIZER_INSTR_CPU_ID_sdl,       // Store Doubleword Left
    RABBITIZER_INSTR_CPU_ID_sdr,       // Store Doubleword Right
    RABBITIZER_INSTR_CPU_ID_swr,       // Store Word Right
    RABBITIZER_INSTR_CPU_ID_cache,     // Cache

    RABBITIZER_INSTR_CPU_ID_ll,        // Load Linked word
    RABBITIZER_INSTR_CPU_ID_lwc1,      // Load Word to Coprocessor z
    RABBITIZER_INSTR_CPU_ID_lwc2,      // Load Word to Coprocessor z
    RABBITIZER_INSTR_CPU_ID_pref,      // Prefetch
    RABBITIZER_INSTR_CPU_ID_lld,       // Load Linked Doubleword
    RABBITIZER_INSTR_CPU_ID_ldc1,      // Load Doubleword to Coprocessor z
    RABBITIZER_INSTR_CPU_ID_ldc2,      // Load Doubleword to Coprocessor z
    RABBITIZER_INSTR_CPU_ID_ld,        // Load Doubleword

    RABBITIZER_INSTR_CPU_ID_sc,        // Store Conditional word
    RABBITIZER_INSTR_CPU_ID_swc1,      // Store Word from Coprocessor z
    RABBITIZER_INSTR_CPU_ID_swc2,      // Store Word from Coprocessor z
    // 
    RABBITIZER_INSTR_CPU_ID_scd,       // Store Conditional Doubleword
    RABBITIZER_INSTR_CPU_ID_sdc1,      // Store Doubleword from Coprocessor z
    RABBITIZER_INSTR_CPU_ID_sdc2,      // Store Doubleword from Coprocessor z
    RABBITIZER_INSTR_CPU_ID_sd,        // Store Doubleword

    // Pseudo-Instruction Unique IDs
    RABBITIZER_INSTR_CPU_ID_beqz,      // Branch on EQual Zero
    RABBITIZER_INSTR_CPU_ID_bnez,      // Branch on Not Equal Zero
    RABBITIZER_INSTR_CPU_ID_b,         // Branch (unconditional)
    RABBITIZER_INSTR_CPU_ID_nop,       // No OPeration
    RABBITIZER_INSTR_CPU_ID_move,      // Move
    RABBITIZER_INSTR_CPU_ID_negu,
    RABBITIZER_INSTR_CPU_ID_not,       // Not

    RABBITIZER_INSTR_CPU_ID_MAX,
} RabbitizerInstrCpuId;

typedef enum RabbitizerInstrRspId {
    RABBITIZER_INSTR_RSP_ID_INVALID = RABBITIZER_INSTR_CPU_ID_MAX+1,

    RABBITIZER_INSTR_RSP_ID_vmulf,
    RABBITIZER_INSTR_RSP_ID_vmulu,
    RABBITIZER_INSTR_RSP_ID_vrndp,
    RABBITIZER_INSTR_RSP_ID_vmulq,
    RABBITIZER_INSTR_RSP_ID_vmudl,
    RABBITIZER_INSTR_RSP_ID_vmudm,
    RABBITIZER_INSTR_RSP_ID_vmudn,
    RABBITIZER_INSTR_RSP_ID_vmudh,
    RABBITIZER_INSTR_RSP_ID_vmacf,
    RABBITIZER_INSTR_RSP_ID_vmacu,
    RABBITIZER_INSTR_RSP_ID_vrndn,
    RABBITIZER_INSTR_RSP_ID_vmacq,
    RABBITIZER_INSTR_RSP_ID_vmadl,
    RABBITIZER_INSTR_RSP_ID_vmadm,
    RABBITIZER_INSTR_RSP_ID_vmadn,
    RABBITIZER_INSTR_RSP_ID_vmadh,
    RABBITIZER_INSTR_RSP_ID_vadd,
    RABBITIZER_INSTR_RSP_ID_vsub,
    RABBITIZER_INSTR_RSP_ID_vabs,
    RABBITIZER_INSTR_RSP_ID_vaddc,
    RABBITIZER_INSTR_RSP_ID_vsubc,
    RABBITIZER_INSTR_RSP_ID_vsar,
    RABBITIZER_INSTR_RSP_ID_vand,
    RABBITIZER_INSTR_RSP_ID_vnand,
    RABBITIZER_INSTR_RSP_ID_vor,
    RABBITIZER_INSTR_RSP_ID_vnor,
    RABBITIZER_INSTR_RSP_ID_vxor,
    RABBITIZER_INSTR_RSP_ID_vnxor,

    RABBITIZER_INSTR_RSP_ID_vlt,
    RABBITIZER_INSTR_RSP_ID_veq,
    RABBITIZER_INSTR_RSP_ID_vne,
    RABBITIZER_INSTR_RSP_ID_vge,
    RABBITIZER_INSTR_RSP_ID_vcl,
    RABBITIZER_INSTR_RSP_ID_vch,
    RABBITIZER_INSTR_RSP_ID_vcr,
    RABBITIZER_INSTR_RSP_ID_vmrg,

    RABBITIZER_INSTR_RSP_ID_vrcp,
    RABBITIZER_INSTR_RSP_ID_vrcpl,
    RABBITIZER_INSTR_RSP_ID_vrcph,
    RABBITIZER_INSTR_RSP_ID_vmov,
    RABBITIZER_INSTR_RSP_ID_vrsq,
    RABBITIZER_INSTR_RSP_ID_vrsql,
    RABBITIZER_INSTR_RSP_ID_vrsqh,
    RABBITIZER_INSTR_RSP_ID_vnop,

    RABBITIZER_INSTR_RSP_ID_mfc2,
    RABBITIZER_INSTR_RSP_ID_mtc2,
    RABBITIZER_INSTR_RSP_ID_cfc2,
    RABBITIZER_INSTR_RSP_ID_ctc2,

    RABBITIZER_INSTR_RSP_ID_sbv,
    RABBITIZER_INSTR_RSP_ID_ssv,
    RABBITIZER_INSTR_RSP_ID_slv,
    RABBITIZER_INSTR_RSP_ID_sdv,

    RABBITIZER_INSTR_RSP_ID_sqv,
    RABBITIZER_INSTR_RSP_ID_srv,

    RABBITIZER_INSTR_RSP_ID_spv,

    RABBITIZER_INSTR_RSP_ID_suv,
    RABBITIZER_INSTR_RSP_ID_swv,

    RABBITIZER_INSTR_RSP_ID_shv,

    RABBITIZER_INSTR_RSP_ID_sfv,
    RABBITIZER_INSTR_RSP_ID_stv,

    RABBITIZER_INSTR_RSP_ID_lbv,
    RABBITIZER_INSTR_RSP_ID_lsv,
    RABBITIZER_INSTR_RSP_ID_llv,
    RABBITIZER_INSTR_RSP_ID_ldv,

    RABBITIZER_INSTR_RSP_ID_lqv,
    RABBITIZER_INSTR_RSP_ID_lrv,

    RABBITIZER_INSTR_RSP_ID_lpv,

    RABBITIZER_INSTR_RSP_ID_luv,

    RABBITIZER_INSTR_RSP_ID_lhv,

    RABBITIZER_INSTR_RSP_ID_lfv,
    RABBITIZER_INSTR_RSP_ID_ltv,

    RABBITIZER_INSTR_RSP_ID_MAX,
} RabbitizerInstrRspId;


typedef union RabbitizerInstrId {
    RabbitizerInstrCpuId cpuId;
    RabbitizerInstrRspId rspId;
} RabbitizerInstrId;


extern const char *RabbitizerInstrId_Names[];

#endif
