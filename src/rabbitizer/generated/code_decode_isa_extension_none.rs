/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::opcodes::{Opcode, OpcodeCategory, OpcodeDecoder};
use crate::{DecodingFlags, EncodedFieldMask, IsaVersion};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_none_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let opcode_category = OpcodeCategory::CORE_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_j,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_jal,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_beq,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bne,
            0x14 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_beql,
            0x15 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bnel,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_blez,
            0x16 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_blezl,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bgtz,
            0x17 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bgtzl,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_addi,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_addiu,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_slti,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sltiu,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_andi,
            0x0D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_ori,
            0x0E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_xori,
            0x18 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_daddi,
            0x19 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_daddiu,
            0x0F if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lui,
            0x1A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_ldl,
            0x1B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_ldr,
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lb,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lh,
            0x22 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lwl,
            0x23 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lw,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lbu,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lhu,
            0x26 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lwr,
            0x27 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_lwu,
            0x28 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sb,
            0x29 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sh,
            0x2A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_swl,
            0x2B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sw,
            0x2C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_sdl,
            0x2D if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_sdr,
            0x2E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_swr,
            0x30 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_ll,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_IV as u32 => Opcode::core_pref,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_lld,
            0x37 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_ld,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_sc,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_scd,
            0x3F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_sd,
            0x2F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_cache,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lwc1,
            0x35 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_ldc1,
            0x39 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_swc1,
            0x3D if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_sdc1,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_lwc2,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_ldc2,
            0x3A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_swc2,
            0x3E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_sdc2,

            0x00 => {
                return Self::decode_isa_extension_none_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x01 => {
                return Self::decode_isa_extension_none_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x10 => {
                return Self::decode_isa_extension_none_coprocessor0(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_none_coprocessor1(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x12 => {
                return Self::decode_isa_extension_none_coprocessor2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sll,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_srl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sra,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsll,
            0x3A if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsrl,
            0x3B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsra,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsll32,
            0x3E if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsrl32,
            0x3F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsra32,
            0x14 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsllv,
            0x16 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsrlv,
            0x17 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsrav,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sllv,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_srlv,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_srav,
            0x11 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mthi,
            0x13 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mtlo,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_jr,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_jalr,
            0x10 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mfhi,
            0x12 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mflo,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_IV as u32 => Opcode::core_movz,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_IV as u32 => Opcode::core_movn,
            0x1A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_div,
            0x1B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_divu,
            0x1E if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_ddiv,
            0x1F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_ddivu,
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_add,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_addu,
            0x22 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sub,
            0x23 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_subu,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_and,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_or,
            0x26 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_xor,
            0x27 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_nor,
            0x2A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_slt,
            0x2B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sltu,
            0x2C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dadd,
            0x2D if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_daddu,
            0x2E if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsub,
            0x2F if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dsubu,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_syscall,
            0x0D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_break,
            0x0F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_sync,
            0x18 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mult,
            0x19 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_multu,
            0x1C if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dmult,
            0x1D if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dmultu,
            0x30 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tge,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tgeu,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tlt,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tltu,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_teq,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tne,

            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bltz,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bgez,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bltzl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bgezl,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tgei,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tgeiu,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tlti,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tltiu,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_teqi,
            0x0E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_tnei,
            0x10 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bltzal,
            0x11 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bgezal,
            0x12 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bltzall,
            0x13 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bgezall,

            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mfc0,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dmfc0,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cfc0,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mtc0,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dmtc0,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_ctc0,
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
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP0_BC0;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bc0f,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bc0t,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bc0fl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bc0tl,
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP0_TLB;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_tlbr,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_tlbwi,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_tlbwr,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_tlbp,
            0x10 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_rfe,
            0x18 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_eret,
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mfc1,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dmfc1,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mtc1,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_dmtc1,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cfc1,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_ctc1,
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
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP1_BC1;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bc1f,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_bc1t,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bc1fl,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_bc1tl,
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP1_FPUS;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_add_s,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sub_s,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mul_s,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_div_s,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_sqrt_s,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_abs_s,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mov_s,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_neg_s,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_round_l_s,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_trunc_l_s,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_ceil_l_s,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_floor_l_s,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_round_w_s,
            0x0D if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_trunc_w_s,
            0x0E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_ceil_w_s,
            0x0F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_floor_w_s,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cvt_d_s,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cvt_w_s,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_cvt_l_s,
            0x30 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_f_s,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_un_s,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_eq_s,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ueq_s,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_olt_s,
            0x35 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ult_s,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ole_s,
            0x37 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ule_s,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_sf_s,
            0x39 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ngle_s,
            0x3A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_seq_s,
            0x3B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ngl_s,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_lt_s,
            0x3D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_nge_s,
            0x3E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_le_s,
            0x3F if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ngt_s,
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP1_FPUD;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_add_d,
            0x01 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_sub_d,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mul_d,
            0x03 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_div_d,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_sqrt_d,
            0x05 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_abs_d,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mov_d,
            0x07 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_neg_d,
            0x08 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_round_l_d,
            0x09 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_trunc_l_d,
            0x0A if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_ceil_l_d,
            0x0B if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_floor_l_d,
            0x0C if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_round_w_d,
            0x0D if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_trunc_w_d,
            0x0E if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_ceil_w_d,
            0x0F if isa_version as u32 >= IsaVersion::MIPS_II as u32 => Opcode::core_floor_w_d,
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cvt_s_d,
            0x24 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cvt_w_d,
            0x25 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_cvt_l_d,
            0x30 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_f_d,
            0x31 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_un_d,
            0x32 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_eq_d,
            0x33 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ueq_d,
            0x34 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_olt_d,
            0x35 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ult_d,
            0x36 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ole_d,
            0x37 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ule_d,
            0x38 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_df_d,
            0x39 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ngle_d,
            0x3A if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_seq_d,
            0x3B if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ngl_d,
            0x3C if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_lt_d,
            0x3D if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_nge_d,
            0x3E if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_le_d,
            0x3F if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_c_ngt_d,
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP1_FPUW;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x20 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cvt_s_w,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cvt_d_w,
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP1_FPUL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x20 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_cvt_s_l,
            0x21 if isa_version as u32 >= IsaVersion::MIPS_III as u32 => Opcode::core_cvt_d_l,
            _ => Opcode::ALL_INVALID,
        };
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
        let opcode_category = OpcodeCategory::CORE_COP2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mfc2,
            0x04 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_mtc2,
            0x02 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_cfc2,
            0x06 if isa_version as u32 >= IsaVersion::MIPS_I as u32 => Opcode::core_ctc2,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
