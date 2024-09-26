/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{
    generated::Gpr, utils, EncodedFieldMask, InstructionFlags, IsaExtension, IsaVersion, Opcode,
    OpcodeCategory, OpcodeDecoder, Operand, ValuedOperandIterator,
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
            InstructionFlags::default()
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

    #[must_use]
    pub const fn flags(&self) -> &InstructionFlags {
        &self.flags
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
        self.opcode_decoder.mandatory_bits() | self.opcode().valid_bits()
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

/// Opcode fields
impl Instruction {
    /// Returns either the [`Gpr`] register embedded on the `rs` field of the
    /// word of this instruction or [`None`] if this instruction does not have
    /// an `rs` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn reg_rs(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::cpu_rs) {
            return None;
        }
        Some(self.reg_rs_unchecked())
    }

    /// Returns either the [`Gpr`] register embedded on the `rt` field of the
    /// word of this instruction or [`None`] if this instruction does not have
    /// an `rt` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn reg_rt(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::cpu_rt) {
            return None;
        }
        Some(self.reg_rt_unchecked())
    }

    /// Returns either the [`Gpr`] register embedded on the `rd` field of the
    /// word of this instruction or [`None`] if this instruction does not have
    /// an `rd` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn reg_rd(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::cpu_rd) {
            return None;
        }
        Some(self.reg_rd_unchecked())
    }

    #[must_use]
    pub fn field_immediate(&self) -> Option<u16> {
        if !self.opcode().has_operand_alias(Operand::cpu_immediate) {
            return None;
        }
        Some(self.field_immediate_unchecked())
    }
}

/// Unchecked opcode fields
impl Instruction {
    // Should be safe to use `.unwrap()` on all the `_unchecked` functions that
    // return an enum because using the correct `EncodedFieldMask` should yield
    // a bit of range that should not escape the range of the given enum.
    // If the unwrap ends up panicking then we have an internal library error
    // somewhere.

    /// Returns the [`Gpr`] register embedded on the `rs` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has an `rs` field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use [`reg_rs`] instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`reg_rs`]: Instruction::reg_rs
    #[must_use]
    pub fn reg_rs_unchecked(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rs.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Gpr`] register embedded on the `rt` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has an `rt` field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use [`reg_rt`] instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`reg_rt`]: Instruction::reg_rt
    #[must_use]
    pub fn reg_rt_unchecked(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rt.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Gpr`] register embedded on the `rd` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has an `rd` field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use [`reg_rd`] instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`reg_rd`]: Instruction::reg_rd
    #[must_use]
    pub fn reg_rd_unchecked(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rd.get_shifted(self.word())).unwrap()
    }

    #[must_use]
    pub fn field_immediate_unchecked(&self) -> u16 {
        EncodedFieldMask::immediate
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }
}

impl Instruction {
    #[must_use]
    pub fn get_processed_immediate(&self) -> Option<i32> {
        if let Some(imm) = self.field_immediate() {
            if self.opcode().is_unsigned() {
                Some(imm as i32)
            } else {
                Some(utils::from_2s_complement(imm as u32, 16))
            }
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_processed_immediate_unchecked(&self) -> i32 {
        let imm = self.field_immediate_unchecked();

        if self.opcode().is_unsigned() {
            imm as i32
        } else {
            utils::from_2s_complement(imm as u32, 16)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_j() {
        let instr =
            Instruction::new_no_extension(0x08000004, 0x80000000, None, IsaVersion::MIPS_III);
        assert!(instr.is_valid());
        assert_eq!(instr.opcode_category(), OpcodeCategory::CPU_NORMAL);
        assert_eq!(instr.opcode(), Opcode::cpu_j);
        assert!(instr.opcode().is_jump());

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
        let flags = InstructionFlags::default();

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
