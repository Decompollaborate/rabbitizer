/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{
    Abi, DecodingFlags, EncodedFieldMask, InstructionFlags, IsaExtension, IsaVersion, Opcode,
    OpcodeCategory, OpcodeDecoder, ValuedOperandIterator,
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    word: u32,
    vram: u32, // TODO: maybe make Vram wrapper type?

    isa_version: IsaVersion,
    isa_extension: IsaExtension,

    opcode_decoder: OpcodeDecoder,

    flags: InstructionFlags,

    _handwritten_category: bool, // TODO: remove in favour of checking the OpcodeCategory instead
}

/// Constructors
impl Instruction {
    #[must_use]
    pub const fn new(
        word: u32,
        vram: u32,
        flags: Option<InstructionFlags>,
        isa_version: IsaVersion,
        isa_extension: IsaExtension,
    ) -> Self {
        let f = if let Some(f) = flags {
            f
        } else {
            InstructionFlags::default()
        };

        let opcode_decoder =
            OpcodeDecoder::decode(word, f.decoding_flags(), isa_version, isa_extension);

        Self {
            word,
            vram,
            isa_version,
            isa_extension,
            opcode_decoder,
            flags: f,
            _handwritten_category: false,
        }
    }

    #[must_use]
    pub const fn new_no_extension(
        word: u32,
        vram: u32,
        flags: Option<InstructionFlags>,
        isa_version: IsaVersion,
    ) -> Self {
        let f = if let Some(f) = flags {
            f
        } else {
            InstructionFlags::default()
        };

        Self::new(word, vram, Some(f), isa_version, IsaExtension::NONE)
    }

    #[must_use]
    pub const fn new_rsp(word: u32, vram: u32, flags: Option<InstructionFlags>) -> Self {
        let f = if let Some(f) = flags {
            f
        } else {
            InstructionFlags {
                abi_gpr: Abi::NUMERIC,
                abi_fpr: Abi::NUMERIC,
                decoding_flags: DecodingFlags::default(),
            }
        };
        let isa_extension = IsaExtension::RSP;

        Self::new(
            word,
            vram,
            Some(f),
            isa_extension.isa_version(),
            isa_extension,
        )
    }

    #[must_use]
    pub const fn new_r3000gte(word: u32, vram: u32, flags: Option<InstructionFlags>) -> Self {
        let f = if let Some(f) = flags {
            f
        } else {
            InstructionFlags::default()
        };
        let isa_extension = IsaExtension::R3000GTE;

        Self::new(
            word,
            vram,
            Some(f),
            isa_extension.isa_version(),
            isa_extension,
        )
    }

    #[must_use]
    pub const fn new_r4000allegrex(word: u32, vram: u32, flags: Option<InstructionFlags>) -> Self {
        let f = if let Some(f) = flags {
            f
        } else {
            InstructionFlags::default()
        };
        let isa_extension = IsaExtension::R4000ALLEGREX;

        Self::new(
            word,
            vram,
            Some(f),
            isa_extension.isa_version(),
            isa_extension,
        )
    }

    #[must_use]
    pub const fn new_r5900(word: u32, vram: u32, flags: Option<InstructionFlags>) -> Self {
        let f = if let Some(f) = flags {
            f
        } else {
            InstructionFlags::default()
        };
        let isa_extension = IsaExtension::R5900;

        Self::new(
            word,
            vram,
            Some(f),
            isa_extension.isa_version(),
            isa_extension,
        )
    }
}

/// Getters and setters
impl Instruction {
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

/// Instruction examination
impl Instruction {
    #[must_use]
    pub const fn is_nop(&self) -> bool {
        OpcodeDecoder::is_nop(self.word)
    }

    #[must_use]
    pub fn valid_bits(&self) -> EncodedFieldMask {
        self.opcode_decoder.mandatory_bits() | self.opcode().get_descriptor().valid_bits()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        if !self.opcode().is_valid() {
            return false;
        }

        let valid_bits = self.valid_bits().bits();

        ((!valid_bits) & self.word()) == 0
    }
}

/// Opcode examination
impl Instruction {
    #[must_use]
    pub fn valued_operands_iter(&self) -> ValuedOperandIterator {
        ValuedOperandIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Abi;

    use super::*;

    #[test]
    fn check_j() {
        let instr =
            Instruction::new_no_extension(0x08000004, 0x80000000, None, IsaVersion::MIPS_III);
        assert!(instr.is_valid());
        assert_eq!(instr.opcode_category(), OpcodeCategory::CPU_NORMAL);
        assert_eq!(instr.opcode(), Opcode::cpu_j);
        assert!(instr.opcode().get_descriptor().is_jump());

        assert_eq!(
            Instruction::new_no_extension(0x08000000, 0x80000000, None, IsaVersion::MIPS_III)
                .opcode(),
            Opcode::cpu_j
        );
    }

    #[test]
    fn check_jal() {
        let instr =
            Instruction::new_no_extension(0x0C000004, 0x80000000, None, IsaVersion::MIPS_III);
        assert!(instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::cpu_jal);
    }

    #[test]
    fn check_lwu() {
        // lwu was introduced in MIPS III
        let mut flags = InstructionFlags::default();

        *flags.abi_gpr_mut() = Abi::NUMERIC;

        let instr = Instruction::new_no_extension(
            0x9C000000,
            0x80000000,
            Some(flags),
            IsaVersion::MIPS_III,
        );
        assert!(instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::cpu_lwu);

        let instr =
            Instruction::new_no_extension(0x9C000000, 0x80000000, Some(flags), IsaVersion::MIPS_II);
        assert!(!instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::ALL_INVALID);
    }

    #[test]
    fn check_invalid() {
        let instr =
            Instruction::new_no_extension(0x0000072E, 0x80000000, None, IsaVersion::MIPS_III);
        assert!(!instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::cpu_dsub);
    }
}
