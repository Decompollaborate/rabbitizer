/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{IsaExtension, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    word: u32,
    vram: u32, // TODO: maybe make Vram wrapper type?

    isa_version: IsaVersion,
    isa_extension: IsaExtension,

    opcode_decoder: OpcodeDecoder,

    _handwritten_category: bool, // TODO: remove in favour of checking the OpcodeCategory instead
    _in_handwritten_function: bool, // TODO: maybe remove?

                                 // flags: u32,
}

impl Instruction {
    #[must_use]
    pub const fn new(
        word: u32,
        vram: u32,
        isa_version: IsaVersion,
        isa_extension: IsaExtension
    ) -> Self {
        let opcode_decoder = OpcodeDecoder::decode(word, isa_version, isa_extension);

        Self {
            word,
            vram,
            isa_version,
            isa_extension,
            opcode_decoder,
            _handwritten_category: false,
            _in_handwritten_function: false,
        }
    }
}

impl Instruction {
    // getters and setters

    #[must_use]
    pub const fn word(&self) -> u32 {
        self.word
    }

    #[must_use]
    pub const fn vram(&self) -> u32 {
        self.vram
    }

    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        self.isa_version
    }

    #[must_use]
    pub const fn isa_extension(&self) -> IsaExtension {
        self.isa_extension
    }

    #[must_use]
    pub const fn opcode(&self) -> Opcode {
        self.opcode_decoder.opcode()
    }

    #[must_use]
    pub const fn opcode_category(&self) -> OpcodeCategory {
        self.opcode_decoder.opcode_category()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_j() {
        let instr = Instruction::new(
            0x08000004,
            0x80000000,
            IsaVersion::MIPS_III,
            IsaExtension::NONE,
        );
        assert_eq!(instr.opcode_category(), OpcodeCategory::CPU_NORMAL);
        assert_eq!(instr.opcode(), Opcode::cpu_j);
        assert_eq!(
            Instruction::new(
                0x08000000,
                0x80000000,
                IsaVersion::MIPS_III,
                IsaExtension::NONE
            )
            .opcode(),
            Opcode::cpu_j
        );
    }
}
