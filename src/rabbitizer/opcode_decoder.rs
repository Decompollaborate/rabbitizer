/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{IsaExtension, IsaVersion, Opcode, OpcodeCategory};

pub struct OpcodeDecoder {
    opcode: Opcode,
    opcode_category: OpcodeCategory,

    mandatory_bits: u32,
}

impl OpcodeDecoder {
    pub const fn decode(
        _word: u32,
        _isa_version: IsaVersion,
        _isa_extension: IsaExtension) -> Self {
            // TODO
            Self {
                opcode: Opcode::cpu_INVALID,
                opcode_category: OpcodeCategory::ALL_INVALID,
                mandatory_bits: 0,
            }
    }
}

impl OpcodeDecoder {
    // getters and setters

    pub const fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub const fn opcode_category(&self) -> OpcodeCategory {
        self.opcode_category
    }

    pub const fn mandatory_bits(&self) -> u32 {
        self.mandatory_bits
    }
}
