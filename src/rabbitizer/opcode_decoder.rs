/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{EncodedFieldMask, IsaExtension, IsaVersion, Opcode, OpcodeCategory};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OpcodeDecoder {
    opcode: Opcode,
    opcode_category: OpcodeCategory,

    mandatory_bits: EncodedFieldMask,
}

impl OpcodeDecoder {
    #[must_use]
    pub const fn decode(word: u32, _isa_version: IsaVersion, isa_extension: IsaExtension) -> Self {
        // TODO
        match isa_extension {
            IsaExtension::NONE => Self::decode_isa_extension_none(word),
            _ => Self {
                opcode: Opcode::cpu_INVALID,
                opcode_category: OpcodeCategory::ALL_INVALID,
                mandatory_bits: EncodedFieldMask::empty(),
            },
        }
    }
}

impl OpcodeDecoder {
    // getters and setters

    #[must_use]
    pub const fn opcode(&self) -> Opcode {
        self.opcode
    }

    #[must_use]
    pub const fn opcode_category(&self) -> OpcodeCategory {
        self.opcode_category
    }

    #[must_use]
    pub const fn mandatory_bits(&self) -> EncodedFieldMask {
        self.mandatory_bits
    }
}

// IsaExtension::NONE
impl OpcodeDecoder {
    #[must_use]
    const fn decode_isa_extension_none(word: u32) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mandatory_bits = mask.mask_value(word);

        match mask.get_shifted(word) {
            0x00 | 0x01 | 0x10 | 0x11 | 0x12 => Self {
                opcode: Opcode::cpu_INVALID,
                opcode_category: OpcodeCategory::CPU_INVALID,
                mandatory_bits,
            },
            _ => Self::decode_isa_extension_none_normal(word, mandatory_bits),
        }
    }

    #[must_use]
    const fn decode_isa_extension_none_normal(
        word: u32,
        mut mandatory_bits: EncodedFieldMask,
    ) -> Self {
        let mask = EncodedFieldMask::opcode;
        let mut opcode = Opcode::cpu_INVALID;
        let opcode_category = OpcodeCategory::CPU_NORMAL;

        mandatory_bits = mandatory_bits.union(EncodedFieldMask::opcode.mask_value(word));

        match mask.get_shifted(word) {
            0x02 => {
                opcode = Opcode::cpu_j;
            }
            _ => {}
        }
        opcode = Self::pseudos_decode_isa_extension_none_normal(word, opcode);
        Self {
            opcode,
            opcode_category,
            mandatory_bits,
        }
    }

    #[must_use]
    const fn pseudos_decode_isa_extension_none_normal(_word: u32, opcode: Opcode) -> Opcode {
        opcode
    }
}
