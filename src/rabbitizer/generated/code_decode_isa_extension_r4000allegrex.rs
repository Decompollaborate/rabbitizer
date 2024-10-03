/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{DecodingFlags, EncodedFieldMask, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x32 => Opcode::r4000allegrex_lv_s,
            0x3A => Opcode::r4000allegrex_sv_s,
            0x36 => Opcode::r4000allegrex_lv_q,
            0x3E => Opcode::r4000allegrex_sv_q,
            0x00 => {
                return Self::decode_isa_extension_r4000allegrex_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x01 => {
                return Self::decode_isa_extension_r4000allegrex_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_normal(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x16 => Opcode::r4000allegrex_clz,
            0x17 => Opcode::r4000allegrex_clo,
            0x1C => Opcode::r4000allegrex_madd,
            0x1D => Opcode::r4000allegrex_maddu,
            0x2E => Opcode::r4000allegrex_msub,
            0x2F => Opcode::r4000allegrex_msubu,
            0x2C => Opcode::r4000allegrex_max,
            0x2D => Opcode::r4000allegrex_min,
            0x02 => {
                return Self::decode_isa_extension_r4000allegrex_special_rs(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x06 => {
                return Self::decode_isa_extension_r4000allegrex_special_sa(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_special(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special_rs(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rs;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL_RS;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_srl,
            0x01 => Opcode::r4000allegrex_rotr,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_special_sa(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::sa;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_SPECIAL_SA;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x00 => Opcode::r4000allegrex_srlv,
            0x01 => Opcode::r4000allegrex_rotrv,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r4000allegrex_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let opcode_category = OpcodeCategory::R4000ALLEGREX_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            _ => {
                return Self::decode_isa_extension_none_regimm(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
