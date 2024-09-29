/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::registers::{Cop1Control, Gpr};
use crate::utils;
use crate::{DisplayFlags, EncodedFieldMask, InstructionDisplay, InstructionFlags};
use crate::{IsaExtension, IsaVersion};
use crate::{Opcode, OpcodeCategory, OpcodeDecoder};
use crate::{Operand, OperandIterator, ValuedOperandIterator};

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
    pub fn operands_iter(&self) -> OperandIterator {
        self.opcode().operands_iter()
    }

    #[must_use]
    pub fn valued_operands_iter(&self) -> ValuedOperandIterator {
        ValuedOperandIterator::new(self)
    }
}

/// Opcode fields
impl Instruction {
    /// Returns either the [`Gpr`] register embedded on the `rs` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rs` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn reg_rs(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::core_rs) {
            return None;
        }
        Some(self.reg_rs_unchecked())
    }

    /// Returns either the [`Gpr`] register embedded on the `rt` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rt` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn reg_rt(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::core_rt) {
            return None;
        }
        Some(self.reg_rt_unchecked())
    }

    /// Returns either the [`Gpr`] register embedded on the `rd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rd` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn reg_rd(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::core_rd) {
            return None;
        }
        Some(self.reg_rd_unchecked())
    }

    /// Returns either the `instr_index` value embedded on the `instr_index`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `instr_index` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_instr_index(&self) -> Option<u32> {
        if self.opcode().has_operand_alias(Operand::core_label) {
            Some(self.field_instr_index_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `immediate` value embedded on the `immediate` field
    /// of the word of this instruction, or [`None`] if this instruction does
    /// not have an `immediate` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_immediate(&self) -> Option<u16> {
        if !self.opcode().has_operand_alias(Operand::core_immediate) {
            return None;
        }
        Some(self.field_immediate_unchecked())
    }

    /// Returns either the [`Cop1Control`] register embedded on the `cop1cs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `cop1cs` operand.
    ///
    /// [`Cop1Control`]: crate::registers::Cop1Control
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_cop1cs(&self) -> Option<Cop1Control> {
        if self.opcode().has_operand_alias(Operand::core_cop1cs) {
            Some(self.field_cop1cs_unchecked())
        } else {
            None
        }
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
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`reg_rs`] function
    /// instead.
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
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`reg_rt`] function
    /// instead.
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
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`reg_rd`] function
    /// instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`reg_rd`]: Instruction::reg_rd
    #[must_use]
    pub fn reg_rd_unchecked(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rd.get_shifted(self.word())).unwrap()
    }

    /// Returns the `instr_index` value embedded on the `instr_index` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`field_instr_index`] function instead.
    ///
    /// [`field_instr_index`]: Instruction::field_instr_index
    #[must_use]
    pub const fn field_instr_index_unchecked(&self) -> u32 {
        EncodedFieldMask::instr_index.get_shifted(self.word())
    }

    /// Returns the `immediate` value embedded on the `immediate` field of the
    /// word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `immediate` value. It is recommended to use the
    /// [`field_immediate`] function instead.
    ///
    /// [`field_immediate`]: Instruction::field_immediate
    #[must_use]
    pub fn field_immediate_unchecked(&self) -> u16 {
        EncodedFieldMask::immediate
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`Cop1Control`] register embedded on the `cop1cs` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1Control`] register. It is recommended to use the
    /// [`field_cop1cs`] function instead.
    ///
    /// [`Cop1Control`]: crate::registers::Cop1Control
    /// [`field_cop1cs`]: Instruction::field_cop1cs
    #[must_use]
    pub fn field_cop1cs_unchecked(&self) -> Cop1Control {
        Cop1Control::try_from(EncodedFieldMask::cop1cs.get_shifted(self.word())).unwrap()
    }
}

impl Instruction {
    /// Get the (possibly signed) immediate value for this instruction.
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

    /// Get the (possibly signed) immediate value for this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has an `immediate` field, meaning that calling this function
    /// on an instruction that does not have this field will interpret garbage
    /// data as returned immediate. It is recommended to use the
    /// [`get_processed_immediate`] function instead.
    ///
    /// [`get_processed_immediate`]: Instruction::get_processed_immediate
    #[must_use]
    pub fn get_processed_immediate_unchecked(&self) -> i32 {
        let imm = self.field_immediate_unchecked();

        if self.opcode().is_unsigned() {
            imm as i32
        } else {
            utils::from_2s_complement(imm as u32, 16)
        }
    }

    #[must_use]
    #[inline(always)]
    const fn vram_from_instr_index(&self, instr_index: u32) -> u32 {
        let vram = instr_index << 2;

        // Jumps are PC-region branches. The upper bits are filled with the address in the delay slot
        vram | (self.vram + 4) & 0xF0000000
    }

    #[must_use]
    pub fn get_instr_index_as_vram(&self) -> Option<u32> {
        self.field_instr_index()
            .map(|instr_index| self.vram_from_instr_index(instr_index))
    }

    #[must_use]
    pub const fn get_instr_index_as_vram_unchecked(&self) -> u32 {
        self.vram_from_instr_index(self.field_instr_index_unchecked())
    }
}

impl Instruction {
    #[must_use]
    pub const fn display<'ins, 'imm, 'flg>(
        &'ins self,
        imm_override: Option<&'imm str>,
        display_flags: &'flg DisplayFlags,
    ) -> InstructionDisplay<'ins, 'imm, 'flg> {
        InstructionDisplay::new(self, imm_override, display_flags)
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
        assert_eq!(instr.opcode_category(), OpcodeCategory::CORE_NORMAL);
        assert_eq!(instr.opcode(), Opcode::core_j);
        assert!(instr.opcode().is_jump());

        assert_eq!(
            Instruction::new_no_extension(0x08000000, 0x80000000, None, IsaVersion::MIPS_III)
                .opcode(),
            Opcode::core_j
        );
    }

    #[test]
    fn check_jal() {
        let instr =
            Instruction::new_no_extension(0x0C000004, 0x80000000, None, IsaVersion::MIPS_III);
        assert!(instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::core_jal);
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
        assert_eq!(instr.opcode(), Opcode::core_lwu);

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
        assert_eq!(instr.opcode(), Opcode::core_dsub);
    }
}
