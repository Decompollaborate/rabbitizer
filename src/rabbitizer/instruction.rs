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
        flags: InstructionFlags,
        isa_version: IsaVersion,
        isa_extension: IsaExtension,
    ) -> Self {
        let opcode_decoder =
            OpcodeDecoder::decode(word, flags.decoding_flags(), isa_version, isa_extension);

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

    #[must_use]
    pub const fn new_no_extension(
        word: u32,
        vram: u32,
        flags: InstructionFlags,
        isa_version: IsaVersion,
    ) -> Self {
        Self::new(word, vram, flags, isa_version, IsaExtension::NONE)
    }

    #[must_use]
    pub const fn new_rsp(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        Self::new(word, vram, flags, IsaVersion::MIPS_III, IsaExtension::RSP)
    }

    #[must_use]
    pub const fn new_r3000gte(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        Self::new(
            word,
            vram,
            flags,
            IsaVersion::MIPS_I,
            IsaExtension::R3000GTE,
        )
    }

    #[must_use]
    pub const fn new_r4000allegrex(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        Self::new(
            word,
            vram,
            flags,
            IsaVersion::MIPS_III,
            IsaExtension::R4000ALLEGREX,
        )
    }

    #[must_use]
    pub const fn new_r5900(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        Self::new(word, vram, flags, IsaVersion::MIPS_IV, IsaExtension::R5900)
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
        let instr = Instruction::new_no_extension(
            0x08000004,
            0x80000000,
            InstructionFlags::default(),
            IsaVersion::MIPS_III,
        );
        assert_eq!(instr.opcode_category(), OpcodeCategory::CPU_NORMAL);
        assert_eq!(instr.opcode(), Opcode::cpu_j);
        assert_eq!(
            Instruction::new_no_extension(
                0x08000000,
                0x80000000,
                InstructionFlags::default(),
                IsaVersion::MIPS_III,
            )
            .opcode(),
            Opcode::cpu_j
        );
    }

    #[test]
    fn check_jal() {
        let instr = Instruction::new_no_extension(
            0x0C000004,
            0x80000000,
            InstructionFlags::default(),
            IsaVersion::MIPS_III,
        );
        assert_eq!(instr.opcode(), Opcode::cpu_jal);
    }

    #[test]
    fn check_lwu() {
        // lwu was introduced in MIPS III

        let instr = Instruction::new_no_extension(
            0x9C000000,
            0x80000000,
            InstructionFlags::default(),
            IsaVersion::MIPS_III,
        );
        assert_eq!(instr.opcode(), Opcode::cpu_lwu);

        let instr = Instruction::new_no_extension(
            0x9C000000,
            0x80000000,
            InstructionFlags::default(),
            IsaVersion::MIPS_II,
        );
        assert_eq!(instr.opcode(), Opcode::cpu_INVALID);
    }
}
