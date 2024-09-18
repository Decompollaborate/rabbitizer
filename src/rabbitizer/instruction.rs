/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{InstructionFlags, IsaExtension, IsaVersion, Opcode, OpcodeCategory, OpcodeDecoder};

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
        isa_extension: IsaExtension,
        flags: InstructionFlags,
    ) -> Self {
        let opcode_decoder =
            OpcodeDecoder::decode(word, isa_version, isa_extension, flags.decoding_flags());

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

impl Instruction {
    #[must_use]
    pub const fn is_nop(&self) -> bool {
        OpcodeDecoder::is_nop(self.word)
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
            InstructionFlags::default(),
        );
        assert_eq!(instr.opcode_category(), OpcodeCategory::CPU_NORMAL);
        assert_eq!(instr.opcode(), Opcode::cpu_j);
        assert_eq!(
            Instruction::new(
                0x08000000,
                0x80000000,
                IsaVersion::MIPS_III,
                IsaExtension::NONE,
                InstructionFlags::default(),
            )
            .opcode(),
            Opcode::cpu_j
        );
    }

    #[test]
    fn check_jal() {
        let instr = Instruction::new(
            0x0C000004,
            0x80000000,
            IsaVersion::MIPS_III,
            IsaExtension::NONE,
            InstructionFlags::default(),
        );
        assert_eq!(instr.opcode(), Opcode::cpu_jal);
    }

    #[test]
    fn check_lwu() {
        // lwu was introduced in MIPS III

        let instr = Instruction::new(
            0x9C000000,
            0x80000000,
            IsaVersion::MIPS_III,
            IsaExtension::NONE,
            InstructionFlags::default(),
        );
        assert_eq!(instr.opcode(), Opcode::cpu_lwu);

        let instr = Instruction::new(
            0x9C000000,
            0x80000000,
            IsaVersion::MIPS_II,
            IsaExtension::NONE,
            InstructionFlags::default(),
        );
        assert_eq!(instr.opcode(), Opcode::cpu_INVALID);
    }
}
