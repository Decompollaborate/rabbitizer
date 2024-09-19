/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{DecodingFlags, EncodedFieldMask, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_j,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_jal,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_beq,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bne,
            0x14 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_beql,
            0x15 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bnel,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_blez,
            0x16 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_blezl,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bgtz,
            0x17 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bgtzl,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_addi,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_addiu,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_slti,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sltiu,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_andi,
            0x0D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_ori,
            0x0E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_xori,
            0x18 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_daddi,
            0x19 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_daddiu
            }
            0x0F if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lui,
            0x1A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_ldl,
            0x1B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_ldr,
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lb,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lh,
            0x22 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lwl,
            0x23 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lw,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lbu,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lhu,
            0x26 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lwr,
            0x27 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_lwu,
            0x28 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sb,
            0x29 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sh,
            0x2A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_swl,
            0x2B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sw,
            0x2C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_sdl,
            0x2D if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_sdr,
            0x2E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_swr,
            0x30 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_ll,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_IV as u32 => opcode = Opcode::cpu_pref,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_lld,
            0x37 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_ld,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_sc,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_scd,
            0x3F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_sd,
            0x2F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_cache,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lwc1,
            0x35 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_ldc1,
            0x39 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_swc1,
            0x3D if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_sdc1,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_lwc2,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_ldc2,
            0x3A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_swc2,
            0x3E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_sdc2,
            0xFFFFFF03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_b,
            0xFFFFFF04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_beqz
            }
            0xFFFFFF05 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_bnez
            }
            _ => {}
        }
        Self {
            opcode: Self::fixups_decode_isa_extension_none_normal(word, opcode, flags, isa_version),
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_special(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sll,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_srl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sra,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsll,
            0x3A if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsrl,
            0x3B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsra,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_dsll32
            }
            0x3E if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_dsrl32
            }
            0x3F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_dsra32
            }
            0x14 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsllv,
            0x16 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsrlv,
            0x17 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsrav,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sllv,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_srlv,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_srav,
            0x11 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mthi,
            0x13 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mtlo,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_jr,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_jalr,
            0x10 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mfhi,
            0x12 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mflo,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_IV as u32 => opcode = Opcode::cpu_movz,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_IV as u32 => opcode = Opcode::cpu_movn,
            0x1A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_div,
            0x1B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_divu,
            0xFFFFFF1A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_sn64_div
            }
            0xFFFFFF1B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_sn64_divu
            }
            0x1E if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_ddiv,
            0x1F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_ddivu,
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_add,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_addu,
            0x22 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sub,
            0x23 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_subu,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_and,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_or,
            0x26 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_xor,
            0x27 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_nor,
            0x2A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_slt,
            0x2B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sltu,
            0x2C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dadd,
            0x2D if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_daddu,
            0x2E if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsub,
            0x2F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dsubu,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_syscall,
            0x0D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_break,
            0x0F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_sync,
            0x18 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mult,
            0x19 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_multu,
            0x1C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dmult,
            0x1D if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_dmultu
            }
            0x30 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tge,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tgeu,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tlt,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tltu,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_teq,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tne,
            0xFFFFFF00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_nop
            }
            0xFFFFFF25 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_move
            }
            0xFFFFFF27 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_not
            }
            0xFFFFFF22 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_neg
            }
            0xFFFFFF23 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_negu
            }
            _ => {}
        }
        Self {
            opcode: Self::fixups_decode_isa_extension_none_special(
                word,
                opcode,
                flags,
                isa_version,
            ),
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bltz,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bgez,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bltzl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bgezl,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tgei,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tgeiu,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tlti,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tltiu,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_teqi,
            0x0E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_tnei,
            0x10 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bltzal,
            0x11 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bgezal,
            0x12 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_bltzall
            }
            0x13 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_bgezall
            }
            0xFFFFFF11 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_bal
            }
            _ => {}
        }
        Self {
            opcode: Self::fixups_decode_isa_extension_none_regimm(word, opcode, flags, isa_version),
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mfc0,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dmfc0,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cfc0,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mtc0,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dmtc0,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_ctc0,
            0x08 => {
                return Self::decode_isa_extension_none_coprocessor0_bc0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_none_coprocessor0_tlb(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor0_bc0(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::bc_fmt;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP0_BC0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bc0f,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bc0t,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bc0fl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bc0tl,
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor0_tlb(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP0_TLB;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_tlbr,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_tlbwi,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_tlbwr,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_tlbp,
            0x10 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_rfe,
            0x18 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_eret,
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mfc1,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dmfc1,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mtc1,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => opcode = Opcode::cpu_dmtc1,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cfc1,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_ctc1,
            0x08 => {
                return Self::decode_isa_extension_none_coprocessor1_bc1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_s(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_d(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x14 => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_w(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x15 => {
                return Self::decode_isa_extension_none_coprocessor1_fpu_l(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor1_bc1(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::bc_fmt;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP1_BC1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bc1f,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_bc1t,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bc1fl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_bc1tl,
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor1_fpu_s(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP1_FPUS;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_add_s,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sub_s,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mul_s,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_div_s,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_sqrt_s,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_abs_s,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mov_s,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_neg_s,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_round_l_s
            }
            0x09 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_trunc_l_s
            }
            0x0A if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_ceil_l_s
            }
            0x0B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_floor_l_s
            }
            0x0C if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_round_w_s
            }
            0x0D if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_trunc_w_s
            }
            0x0E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_ceil_w_s
            }
            0x0F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_floor_w_s
            }
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cvt_d_s,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cvt_w_s,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_cvt_l_s
            }
            0x30 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_f_s,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_un_s,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_eq_s,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ueq_s,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_olt_s,
            0x35 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ult_s,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ole_s,
            0x37 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ule_s,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_sf_s,
            0x39 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_c_ngle_s
            }
            0x3A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_seq_s,
            0x3B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ngl_s,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_lt_s,
            0x3D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_nge_s,
            0x3E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_le_s,
            0x3F if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ngt_s,
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor1_fpu_d(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP1_FPUD;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_add_d,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_sub_d,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mul_d,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_div_d,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => opcode = Opcode::cpu_sqrt_d,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_abs_d,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mov_d,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_neg_d,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_round_l_d
            }
            0x09 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_trunc_l_d
            }
            0x0A if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_ceil_l_d
            }
            0x0B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_floor_l_d
            }
            0x0C if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_round_w_d
            }
            0x0D if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_trunc_w_d
            }
            0x0E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_ceil_w_d
            }
            0x0F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => {
                opcode = Opcode::cpu_floor_w_d
            }
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cvt_s_d,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cvt_w_d,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_cvt_l_d
            }
            0x30 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_f_d,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_un_d,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_eq_d,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ueq_d,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_olt_d,
            0x35 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ult_d,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ole_d,
            0x37 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ule_d,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_df_d,
            0x39 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => {
                opcode = Opcode::cpu_c_ngle_d
            }
            0x3A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_seq_d,
            0x3B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ngl_d,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_lt_d,
            0x3D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_nge_d,
            0x3E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_le_d,
            0x3F if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_c_ngt_d,
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor1_fpu_w(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP1_FPUW;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cvt_s_w,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cvt_d_w,
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor1_fpu_l(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP1_FPUL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x20 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_cvt_s_l
            }
            0x21 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => {
                opcode = Opcode::cpu_cvt_d_l
            }
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_coprocessor2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::CPU_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mfc2,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_mtc2,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_cfc2,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => opcode = Opcode::cpu_ctc2,
            _ => {}
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
