/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{DecodingFlags, EncodedFieldMask, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mut opcode = Opcode::ALL_INVALID;
        let opcode_category = OpcodeCategory::RSP_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x02 => opcode = Opcode::rsp_j,
            0x03 => opcode = Opcode::rsp_jal,
            0x04 => opcode = Opcode::rsp_beq,
            0x05 => opcode = Opcode::rsp_bne,
            0x06 => opcode = Opcode::rsp_blez,
            0x07 => opcode = Opcode::rsp_bgtz,
            0x08 => opcode = Opcode::rsp_addi,
            0x09 => opcode = Opcode::rsp_addiu,
            0x0A => opcode = Opcode::rsp_slti,
            0x0B => opcode = Opcode::rsp_sltiu,
            0x0C => opcode = Opcode::rsp_andi,
            0x0D => opcode = Opcode::rsp_ori,
            0x0E => opcode = Opcode::rsp_xori,
            0x0F => opcode = Opcode::rsp_lui,
            0x20 => opcode = Opcode::rsp_lb,
            0x21 => opcode = Opcode::rsp_lh,
            0x23 => opcode = Opcode::rsp_lw,
            0x24 => opcode = Opcode::rsp_lbu,
            0x25 => opcode = Opcode::rsp_lhu,
            0x28 => opcode = Opcode::rsp_sb,
            0x29 => opcode = Opcode::rsp_sh,
            0x2B => opcode = Opcode::rsp_sw,
            0x33 => opcode = Opcode::rsp_pref,
            0xFFFFFF03 => opcode = Opcode::rsp_b,
            0xFFFFFF04 => opcode = Opcode::rsp_beqz,
            0xFFFFFF05 => opcode = Opcode::rsp_bnez,
            0x32 => {
                return Self::decode_isa_extension_rsp_lwc2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x3A => {
                return Self::decode_isa_extension_rsp_swc2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {}
        }
        Self {
            opcode: Self::fixups_decode_isa_extension_rsp_normal(word, opcode, flags, isa_version),
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_lwc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rd;
        let opcode;
        let opcode_category = OpcodeCategory::RSP_NORMAL_LWC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_lbv,
            0x01 => opcode = Opcode::rsp_lsv,
            0x02 => opcode = Opcode::rsp_llv,
            0x03 => opcode = Opcode::rsp_ldv,
            0x04 => opcode = Opcode::rsp_lqv,
            0x05 => opcode = Opcode::rsp_lrv,
            0x06 => opcode = Opcode::rsp_lpv,
            0x07 => opcode = Opcode::rsp_luv,
            0x08 => opcode = Opcode::rsp_lhv,
            0x09 => opcode = Opcode::rsp_lfv,
            0x0B => opcode = Opcode::rsp_ltv,
            _ => opcode = Opcode::ALL_INVALID,
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_rsp_swc2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rd;
        let opcode;
        let opcode_category = OpcodeCategory::RSP_NORMAL_SWC2;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x00 => opcode = Opcode::rsp_sbv,
            0x01 => opcode = Opcode::rsp_ssv,
            0x02 => opcode = Opcode::rsp_slv,
            0x03 => opcode = Opcode::rsp_sdv,
            0x04 => opcode = Opcode::rsp_sqv,
            0x05 => opcode = Opcode::rsp_srv,
            0x06 => opcode = Opcode::rsp_spv,
            0x07 => opcode = Opcode::rsp_suv,
            0x08 => opcode = Opcode::rsp_shv,
            0x09 => opcode = Opcode::rsp_sfv,
            0x0B => opcode = Opcode::rsp_stv,
            0xFFFFFF07 => opcode = Opcode::rsp_swv,
            _ => opcode = Opcode::ALL_INVALID,
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
        .fixups_decode_isa_extension_rsp_swc2(word, flags, isa_version)
    }
}
