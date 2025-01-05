/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::encoded_field_mask::EncodedFieldMask;
use crate::isa::IsaVersion;
use crate::opcodes::{DecodingFlags, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
    #[must_use]
    pub(crate) const fn decode_isa_extension_r3000gte_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let _opcode_category = OpcodeCategory::CORE_NORMAL;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
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
                return Self::decode_isa_extension_r3000gte_coprocessor2(
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
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r3000gte_coprocessor2(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        flags: &DecodingFlags,
        isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::fmt;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        match mask.get_shifted(word) {
            0x10 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x11 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x12 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x13 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x14 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x15 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x16 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x17 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x18 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x19 => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1A => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1B => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1C => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            0x1D => {
                return Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
            _ => {
                return Self::decode_isa_extension_none_coprocessor2(
                    word,
                    mandatory_bits,
                    flags,
                    isa_version,
                )
            }
        }
    }
    #[must_use]
    pub(crate) const fn decode_isa_extension_r3000gte_coprocessor2_gte(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
        _flags: &DecodingFlags,
        _isa_version: IsaVersion,
    ) -> Self {
        let mask = EncodedFieldMask::function;
        let opcode_category = OpcodeCategory::R3000GTE_COP2_GTE;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        mandatory_bits =
            mandatory_bits.union(EncodedFieldMask::r3000gte_fake_opcode.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_sf.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_mx.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_v.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_cv.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_lm.mask_value(word));
        let opcode = match mask.get_shifted(word) {
            0x01 => Opcode::r3000gte_RTPS,
            0x30 => Opcode::r3000gte_RTPT,
            0x29 => Opcode::r3000gte_DPCL,
            0x10 => Opcode::r3000gte_DPCS,
            0x2A => Opcode::r3000gte_DPCT,
            0x11 => Opcode::r3000gte_INTPL,
            0x1E => Opcode::r3000gte_NCS,
            0x20 => Opcode::r3000gte_NCT,
            0x13 => Opcode::r3000gte_NCDS,
            0x16 => Opcode::r3000gte_NCDT,
            0x1B => Opcode::r3000gte_NCCS,
            0x3F => Opcode::r3000gte_NCCT,
            0x14 => Opcode::r3000gte_CDP,
            0x1C => Opcode::r3000gte_CC,
            0x06 => Opcode::r3000gte_NCLIP,
            0x2D => Opcode::r3000gte_AVSZ3,
            0x2E => Opcode::r3000gte_AVSZ4,
            0x12 => Opcode::r3000gte_MVMVA,
            0x28 => Opcode::r3000gte_SQR,
            0x0C => Opcode::r3000gte_OP,
            0x3D => Opcode::r3000gte_GPF,
            0x3E => Opcode::r3000gte_GPL,
            _ => Opcode::ALL_INVALID,
        };
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
