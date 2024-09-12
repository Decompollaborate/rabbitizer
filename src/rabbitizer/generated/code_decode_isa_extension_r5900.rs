/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{DecodingFlags, EncodedFieldMask, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        isa_version: IsaVersion,
        flags: &DecodingFlags,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let opcode;
        let opcode_category = OpcodeCategory::R5900_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x1E => opcode = Opcode::r5900_lq,
            0x1F => opcode = Opcode::r5900_sq,
            0x36 => opcode = Opcode::r5900_lqc2,
            0x3E => opcode = Opcode::r5900_sqc2,
            _ => {
                return Self::decode_isa_extension_none_normal(
                    word,
                    mandatory_bits,
                    isa_version,
                    flags,
                )
            }
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_special(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        isa_version: IsaVersion,
        flags: &DecodingFlags,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode;
        let opcode_category = OpcodeCategory::R5900_SPECIAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x0F => opcode = Opcode::r5900_sync_p,
            0x18 => opcode = Opcode::r5900_mult,
            0x28 => opcode = Opcode::r5900_mfsa,
            0x29 => opcode = Opcode::r5900_mtsa,
            _ => {
                return Self::decode_isa_extension_none_special(
                    word,
                    mandatory_bits,
                    isa_version,
                    flags,
                )
            }
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
        .fixups_decode_isa_extension_r5900_special(word, isa_version, flags)
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r5900_regimm(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        isa_version: IsaVersion,
        flags: &DecodingFlags,
    ) -> Self {
        let mask = EncodedFieldMask::rt;
        let opcode;
        let opcode_category = OpcodeCategory::R5900_REGIMM;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x18 => opcode = Opcode::r5900_mtsab,
            0x19 => opcode = Opcode::r5900_mtsah,
            _ => {
                return Self::decode_isa_extension_none_regimm(
                    word,
                    mandatory_bits,
                    isa_version,
                    flags,
                )
            }
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
