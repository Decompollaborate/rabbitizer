/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{DecodingFlags, EncodedFieldMask, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};
impl OpcodeDecoder {
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
            0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1A | 0x1B
            | 0x1C | 0x1D => Self::decode_isa_extension_r3000gte_coprocessor2_gte(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
            _ => Self::decode_isa_extension_none_coprocessor2(
                word,
                mandatory_bits,
                flags,
                isa_version,
            ),
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
        let opcode;
        let opcode_category = OpcodeCategory::R3000GTE_COP2_GTE;
        mandatory_bits = mandatory_bits.union(mask.mask_value(word));
        mandatory_bits =
            mandatory_bits.union(EncodedFieldMask::r3000gte_fake_opcode.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_sf.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_mx.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_v.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_cv.mask_value(word));
        mandatory_bits = mandatory_bits.union(EncodedFieldMask::r3000gte_lm.mask_value(word));
        match mask.get_shifted(word) {
            0x01 => opcode = Opcode::r3000gte_RTPS,
            0x30 => opcode = Opcode::r3000gte_RTPT,
            0x29 => opcode = Opcode::r3000gte_DPCL,
            0x10 => opcode = Opcode::r3000gte_DPCS,
            0x2A => opcode = Opcode::r3000gte_DPCT,
            0x11 => opcode = Opcode::r3000gte_INTPL,
            0x1E => opcode = Opcode::r3000gte_NCS,
            0x20 => opcode = Opcode::r3000gte_NCT,
            0x13 => opcode = Opcode::r3000gte_NCDS,
            0x16 => opcode = Opcode::r3000gte_NCDT,
            0x1B => opcode = Opcode::r3000gte_NCCS,
            0x3F => opcode = Opcode::r3000gte_NCCT,
            0x14 => opcode = Opcode::r3000gte_CDP,
            0x1C => opcode = Opcode::r3000gte_CC,
            0x06 => opcode = Opcode::r3000gte_NCLIP,
            0x2D => opcode = Opcode::r3000gte_AVSZ3,
            0x2E => opcode = Opcode::r3000gte_AVSZ4,
            0x12 => opcode = Opcode::r3000gte_MVMVA,
            0x28 => opcode = Opcode::r3000gte_SQR,
            0x0C => opcode = Opcode::r3000gte_OP,
            0x3D => opcode = Opcode::r3000gte_GPF,
            0x3E => opcode = Opcode::r3000gte_GPL,
            _ => opcode = Opcode::ALL_INVALID,
        }
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }
}
