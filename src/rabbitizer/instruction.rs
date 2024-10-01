/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::registers::*;
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
            flags,
            _handwritten_category: false,
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
        let isa_extension = IsaExtension::RSP;

        Self::new(
            word,
            vram,
            flags,
            isa_extension.isa_version(),
            isa_extension,
        )
    }

    #[must_use]
    pub const fn new_r3000gte(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        let isa_extension = IsaExtension::R3000GTE;

        Self::new(
            word,
            vram,
            flags,
            isa_extension.isa_version(),
            isa_extension,
        )
    }

    #[must_use]
    pub const fn new_r4000allegrex(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        let isa_extension = IsaExtension::R4000ALLEGREX;

        Self::new(
            word,
            vram,
            flags,
            isa_extension.isa_version(),
            isa_extension,
        )
    }

    #[must_use]
    pub const fn new_r5900(word: u32, vram: u32, flags: InstructionFlags) -> Self {
        let isa_extension = IsaExtension::R5900;

        Self::new(
            word,
            vram,
            flags,
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
    pub fn field_rs(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::core_rs) {
            return None;
        }
        Some(self.field_rs_unchecked())
    }

    /// Returns either the [`Gpr`] register embedded on the `rt` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rt` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_rt(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::core_rt) {
            return None;
        }
        Some(self.field_rt_unchecked())
    }

    /// Returns either the [`Gpr`] register embedded on the `rd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// an `rd` operand.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_rd(&self) -> Option<Gpr> {
        if !self.opcode().has_operand_alias(Operand::core_rd) {
            return None;
        }
        Some(self.field_rd_unchecked())
    }

    /// Returns either the `sa` value embedded on the `sa`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_sa(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::core_sa) {
            Some(self.field_sa_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop0`] register embedded on the `cop0d` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop0`]: crate::registers::Cop0
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_cop0d(&self) -> Option<Cop0> {
        if self.opcode().has_operand_alias(Operand::core_cop0d) {
            Some(self.field_cop0d_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop0Control`] register embedded on the `cop0cd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop0Control`]: crate::registers::Cop0Control
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_cop0cd(&self) -> Option<Cop0Control> {
        if self.opcode().has_operand_alias(Operand::core_cop0cd) {
            Some(self.field_cop0cd_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1`] register embedded on the `fs` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_fs(&self) -> Option<Cop1> {
        if self.opcode().has_operand_alias(Operand::core_fs) {
            Some(self.field_fs_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1`] register embedded on the `ft` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_ft(&self) -> Option<Cop1> {
        if self.opcode().has_operand_alias(Operand::core_ft) {
            Some(self.field_ft_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1`] register embedded on the `fd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_fd(&self) -> Option<Cop1> {
        if self.opcode().has_operand_alias(Operand::core_fd) {
            Some(self.field_fd_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop1Control`] register embedded on the `cop1cs`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have this field.
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

    /// Returns either the [`Cop2`] register embedded on the `cop2t` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_cop2t(&self) -> Option<Cop2> {
        if self.opcode().has_operand_alias(Operand::core_cop2t) {
            Some(self.field_cop2d_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop2`] register embedded on the `cop2d` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_cop2d(&self) -> Option<Cop2> {
        if self.opcode().has_operand_alias(Operand::core_cop2d) {
            Some(self.field_cop2d_unchecked())
        } else {
            None
        }
    }

    /// Returns either the [`Cop2Control`] register embedded on the `cop2cd` field of the
    /// word of this instruction, or [`None`] if this instruction does not have
    /// this field.
    ///
    /// [`Cop2Control`]: crate::registers::Cop2Control
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_cop2cd(&self) -> Option<Cop2Control> {
        if self.opcode().has_operand_alias(Operand::core_cop2cd) {
            Some(self.field_cop2cd_unchecked())
        } else {
            None
        }
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
    /// Note this method returns the immediate as unsigned, but some
    /// instructions use this value as signed and others as unsigned. To get
    /// an immediate with proper signedness use [`get_processed_immediate`]
    /// instead.
    ///
    /// [`None`]: Option::None
    /// [`get_processed_immediate`]: Instruction::get_processed_immediate
    #[must_use]
    pub fn field_immediate(&self) -> Option<u16> {
        if !self.opcode().has_operand_alias(Operand::core_immediate) {
            return None;
        }
        Some(self.field_immediate_unchecked())
    }

    /// Returns either the `op` value embedded on the `op`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `op` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_op(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::core_label) {
            Some(self.field_op_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `hint` value embedded on the `hint`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `hint` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_hint(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::core_label) {
            Some(self.field_hint_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `code_upper` value embedded on the `code_upper`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `code_upper` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_code_upper(&self) -> Option<u16> {
        if self.opcode().has_operand_alias(Operand::core_label) {
            Some(self.field_code_upper_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `code_lower` value embedded on the `code_lower`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not have an `code_lower` operand.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_code_lower(&self) -> Option<u16> {
        if self.opcode().has_operand_alias(Operand::core_label) {
            Some(self.field_code_lower_unchecked())
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
    /// as a [`Gpr`] register. It is recommended to use the [`field_rs`] function
    /// instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`field_rs`]: Instruction::field_rs
    #[must_use]
    pub fn field_rs_unchecked(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rs.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Gpr`] register embedded on the `rt` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`field_rt`] function
    /// instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`field_rt`]: Instruction::field_rt
    #[must_use]
    pub fn field_rt_unchecked(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rt.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Gpr`] register embedded on the `rd` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Gpr`] register. It is recommended to use the [`field_rd`] function
    /// instead.
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`field_rd`]: Instruction::field_rd
    #[must_use]
    pub fn field_rd_unchecked(&self) -> Gpr {
        Gpr::try_from(EncodedFieldMask::rd.get_shifted(self.word())).unwrap()
    }

    /// Returns the `sa` value embedded on the `sa` field of the word of this
    /// instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_sa`] function
    /// instead.
    ///
    /// [`field_sa`]: Instruction::field_sa
    #[must_use]
    pub fn field_sa_unchecked(&self) -> u8 {
        EncodedFieldMask::sa
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`Cop0`] register embedded on the `cop0d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop0`] register. It is recommended to use the [`field_cop0d`]
    /// function instead.
    ///
    /// [`Cop0`]: crate::registers::Cop0
    /// [`field_cop0d`]: Instruction::field_cop0d
    #[must_use]
    pub fn field_cop0d_unchecked(&self) -> Cop0 {
        Cop0::try_from(EncodedFieldMask::cop0d.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Cop0Control`] register embedded on the `cop0cd` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop0Control`] register. It is recommended to use the
    /// [`field_cop0cd`] function instead.
    ///
    /// [`Cop0Control`]: crate::registers::Cop0Control
    /// [`field_cop0cd`]: Instruction::field_cop0cd
    #[must_use]
    pub fn field_cop0cd_unchecked(&self) -> Cop0Control {
        Cop0Control::try_from(EncodedFieldMask::cop0cd.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Cop1`] register embedded on the `fs` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`field_fs`] function
    /// instead.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`field_fs`]: Instruction::field_fs
    #[must_use]
    pub fn field_fs_unchecked(&self) -> Cop1 {
        Cop1::try_from(EncodedFieldMask::fs.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Cop1`] register embedded on the `ft` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`field_ft`] function
    /// instead.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`field_ft`]: Instruction::field_ft
    #[must_use]
    pub fn field_ft_unchecked(&self) -> Cop1 {
        Cop1::try_from(EncodedFieldMask::ft.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Cop1`] register embedded on the `fd` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop1`] register. It is recommended to use the [`field_fd`] function
    /// instead.
    ///
    /// [`Cop1`]: crate::registers::Cop1
    /// [`field_fd`]: Instruction::field_fd
    #[must_use]
    pub fn field_fd_unchecked(&self) -> Cop1 {
        Cop1::try_from(EncodedFieldMask::fd.get_shifted(self.word())).unwrap()
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

    /// Returns the [`Cop2`] register embedded on the `cop2t` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2`] register. It is recommended to use the [`field_cop2t`]
    /// function instead.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`field_cop2t`]: Instruction::field_cop2t
    #[must_use]
    pub fn field_cop2t_unchecked(&self) -> Cop2 {
        Cop2::try_from(EncodedFieldMask::cop2t.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Cop2`] register embedded on the `cop2d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2`] register. It is recommended to use the [`field_cop2d`]
    /// function instead.
    ///
    /// [`Cop2`]: crate::registers::Cop2
    /// [`field_cop2d`]: Instruction::field_cop2d
    #[must_use]
    pub fn field_cop2d_unchecked(&self) -> Cop2 {
        Cop2::try_from(EncodedFieldMask::cop2d.get_shifted(self.word())).unwrap()
    }

    /// Returns the [`Cop2Control`] register embedded on the `cop2cd` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as a [`Cop2Control`] register. It is recommended to use the
    /// [`field_cop2cd`] function instead.
    ///
    /// [`Cop2Control`]: crate::registers::Cop2Control
    /// [`field_cop2cd`]: Instruction::field_cop2cd
    #[must_use]
    pub fn field_cop2cd_unchecked(&self) -> Cop2Control {
        Cop2Control::try_from(EncodedFieldMask::cop2cd.get_shifted(self.word())).unwrap()
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

    /// Returns the `op` value embedded on the `op` field of the word of this
    /// instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `op` value. It is recommended to use the
    /// [`field_op`] function instead.
    ///
    /// [`field_op`]: Instruction::field_op
    #[must_use]
    pub fn field_op_unchecked(&self) -> u8 {
        EncodedFieldMask::op
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `hint` value embedded on the `hint` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `hint` value. It is recommended to use the
    /// [`field_hint`] function instead.
    ///
    /// [`field_hint`]: Instruction::field_hint
    #[must_use]
    pub fn field_hint_unchecked(&self) -> u8 {
        EncodedFieldMask::hint
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `code_upper` value embedded on the `code_upper` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `code_upper` value. It is recommended to use the
    /// [`field_code_upper`] function instead.
    ///
    /// [`field_code_upper`]: Instruction::field_code_upper
    #[must_use]
    pub fn field_code_upper_unchecked(&self) -> u16 {
        EncodedFieldMask::code_upper
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `code_lower` value embedded on the `code_lower` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `code_lower` value. It is recommended to use the
    /// [`field_code_lower`] function instead.
    ///
    /// [`field_code_lower`]: Instruction::field_code_lower
    #[must_use]
    pub fn field_code_lower_unchecked(&self) -> u16 {
        EncodedFieldMask::code_lower
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }
}

/// R3000GTE opcode fields
impl Instruction {
    /// Returns either the `r3000gte_sf` value embedded on the `r3000gte_sf`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_r3000gte_sf(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::r3000gte_sf) {
            Some(self.field_r3000gte_sf_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_mx` value embedded on the `r3000gte_mx`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_r3000gte_mx(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::r3000gte_mx) {
            Some(self.field_r3000gte_mx_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_v` value embedded on the `r3000gte_v`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_r3000gte_v(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::r3000gte_v) {
            Some(self.field_r3000gte_v_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_cv` value embedded on the `r3000gte_cv`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_r3000gte_cv(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::r3000gte_cv) {
            Some(self.field_r3000gte_cv_unchecked())
        } else {
            None
        }
    }

    /// Returns either the `r3000gte_lm` value embedded on the `r3000gte_lm`
    /// field of the word of this instruction, or [`None`] if this instruction
    /// does not this field.
    ///
    /// [`None`]: Option::None
    #[must_use]
    pub fn field_r3000gte_lm(&self) -> Option<u8> {
        if self.opcode().has_operand_alias(Operand::r3000gte_lm) {
            Some(self.field_r3000gte_lm_unchecked())
        } else {
            None
        }
    }
}

/// Unchecked R3000GTE opcode fields
impl Instruction {
    /// Returns the `r3000gte_sf` value embedded on the `r3000gte_sf` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r3000gte_sf`]
    /// function instead.
    ///
    /// [`field_r3000gte_sf`]: Instruction::field_r3000gte_sf
    #[must_use]
    pub fn field_r3000gte_sf_unchecked(&self) -> u8 {
        EncodedFieldMask::r3000gte_sf
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_mx` value embedded on the `r3000gte_mx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r3000gte_mx`]
    /// function instead.
    ///
    /// [`field_r3000gte_mx`]: Instruction::field_r3000gte_mx
    #[must_use]
    pub fn field_r3000gte_mx_unchecked(&self) -> u8 {
        EncodedFieldMask::r3000gte_mx
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_v` value embedded on the `r3000gte_v` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r3000gte_v`]
    /// function instead.
    ///
    /// [`field_r3000gte_v`]: Instruction::field_r3000gte_v
    #[must_use]
    pub fn field_r3000gte_v_unchecked(&self) -> u8 {
        EncodedFieldMask::r3000gte_v
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_cv` value embedded on the `r3000gte_cv` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r3000gte_cv`]
    /// function instead.
    ///
    /// [`field_r3000gte_cv`]: Instruction::field_r3000gte_cv
    #[must_use]
    pub fn field_r3000gte_cv_unchecked(&self) -> u8 {
        EncodedFieldMask::r3000gte_cv
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r3000gte_lm` value embedded on the `r3000gte_lm` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r3000gte_lm`]
    /// function instead.
    ///
    /// [`field_r3000gte_lm`]: Instruction::field_r3000gte_lm
    #[must_use]
    pub fn field_r3000gte_lm_unchecked(&self) -> u8 {
        EncodedFieldMask::r3000gte_lm
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }
}

/// Unchecked R5900 opcode fields
impl Instruction {
    /// Returns the `r5900_immediate5` value embedded on the `r5900_immediate5` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_immediate5` value. It is recommended to use the
    /// [`field_r5900_immediate5`] function instead.
    ///
    /// [`field_r5900_immediate5`]: Instruction::field_r5900_immediate5
    #[must_use]
    pub fn field_r5900_immediate5_unchecked(&self) -> u8 {
        EncodedFieldMask::r5900_immediate5
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r5900_immediate15` value embedded on the `r5900_immediate15` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_immediate15` value. It is recommended to use the
    /// [`field_r5900_immediate15`] function instead.
    ///
    /// [`field_r5900_immediate15`]: Instruction::field_r5900_immediate15
    #[must_use]
    pub fn field_r5900_immediate15_unchecked(&self) -> u16 {
        EncodedFieldMask::r5900_immediate15
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900VF`] register embedded on the `r5900_vfs` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r5900_vfs`]
    /// function instead.
    ///
    /// [`R5900VF`]: crate::registers::R5900VF
    /// [`field_r5900_vfs`]: Instruction::field_r5900_vfs
    #[must_use]
    pub fn field_r5900_vfs_unchecked(&self) -> R5900VF {
        EncodedFieldMask::r5900_vfs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900VF`] register embedded on the `r5900_vft` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r5900_vft`]
    /// function instead.
    ///
    /// [`R5900VF`]: crate::registers::R5900VF
    /// [`field_r5900_vft`]: Instruction::field_r5900_vft
    #[must_use]
    pub fn field_r5900_vft_unchecked(&self) -> R5900VF {
        EncodedFieldMask::r5900_vft
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900VF`] register embedded on the `r5900_vfd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r5900_vfd`]
    /// function instead.
    ///
    /// [`R5900VF`]: crate::registers::R5900VF
    /// [`field_r5900_vfd`]: Instruction::field_r5900_vfd
    #[must_use]
    pub fn field_r5900_vfd_unchecked(&self) -> R5900VF {
        EncodedFieldMask::r5900_vfd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900VI`] register embedded on the `r5900_vis` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r5900_vis`]
    /// function instead.
    ///
    /// [`R5900VI`]: crate::registers::R5900VI
    /// [`field_r5900_vis`]: Instruction::field_r5900_vis
    #[must_use]
    pub fn field_r5900_vis_unchecked(&self) -> R5900VI {
        EncodedFieldMask::r5900_vis
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900VI`] register embedded on the `r5900_vit` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r5900_vit`]
    /// function instead.
    ///
    /// [`R5900VI`]: crate::registers::R5900VI
    /// [`field_r5900_vit`]: Instruction::field_r5900_vit
    #[must_use]
    pub fn field_r5900_vit_unchecked(&self) -> R5900VI {
        EncodedFieldMask::r5900_vit
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R5900VI`] register embedded on the `r5900_vid` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r5900_vid`]
    /// function instead.
    ///
    /// [`R5900VI`]: crate::registers::R5900VI
    /// [`field_r5900_vid`]: Instruction::field_r5900_vid
    #[must_use]
    pub fn field_r5900_vid_unchecked(&self) -> R5900VI {
        EncodedFieldMask::r5900_vid
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r5900_xyzw_x` value embedded on the `r5900_xyzw_x` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_xyzw_x` value. It is recommended to use the
    /// [`field_r5900_xyzw_x`] function instead.
    ///
    /// [`field_r5900_xyzw_x`]: Instruction::field_r5900_xyzw_x
    #[must_use]
    pub const fn field_r5900_xyzw_x_unchecked(&self) -> bool {
        EncodedFieldMask::r5900_xyzw_x.get_shifted(self.word()) != 0
    }

    /// Returns the `r5900_xyzw_y` value embedded on the `r5900_xyzw_y` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_xyzw_y` value. It is recommended to use the
    /// [`field_r5900_xyzw_y`] function instead.
    ///
    /// [`field_r5900_xyzw_y`]: Instruction::field_r5900_xyzw_y
    #[must_use]
    pub const fn field_r5900_xyzw_y_unchecked(&self) -> bool {
        EncodedFieldMask::r5900_xyzw_y.get_shifted(self.word()) != 0
    }

    /// Returns the `r5900_xyzw_z` value embedded on the `r5900_xyzw_z` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_xyzw_z` value. It is recommended to use the
    /// [`field_r5900_xyzw_z`] function instead.
    ///
    /// [`field_r5900_xyzw_z`]: Instruction::field_r5900_xyzw_z
    #[must_use]
    pub const fn field_r5900_xyzw_z_unchecked(&self) -> bool {
        EncodedFieldMask::r5900_xyzw_z.get_shifted(self.word()) != 0
    }

    /// Returns the `r5900_xyzw_w` value embedded on the `r5900_xyzw_w` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_xyzw_w` value. It is recommended to use the
    /// [`field_r5900_xyzw_w`] function instead.
    ///
    /// [`field_r5900_xyzw_w`]: Instruction::field_r5900_xyzw_w
    #[must_use]
    pub const fn field_r5900_xyzw_w_unchecked(&self) -> bool {
        EncodedFieldMask::r5900_xyzw_w.get_shifted(self.word()) != 0
    }

    /// Returns the `r5900_n` value embedded on the `r5900_n` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_n` value. It is recommended to use the
    /// [`field_r5900_n`] function instead.
    ///
    /// [`field_r5900_n`]: Instruction::field_r5900_n
    #[must_use]
    pub fn field_r5900_n_unchecked(&self) -> u8 {
        EncodedFieldMask::r5900_n
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r5900_l` value embedded on the `r5900_l` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_l` value. It is recommended to use the
    /// [`field_r5900_l`] function instead.
    ///
    /// [`field_r5900_l`]: Instruction::field_r5900_l
    #[must_use]
    pub fn field_r5900_l_unchecked(&self) -> u8 {
        EncodedFieldMask::r5900_l
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r5900_m` value embedded on the `r5900_m` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as an `r5900_m` value. It is recommended to use the
    /// [`field_r5900_m`] function instead.
    ///
    /// [`field_r5900_m`]: Instruction::field_r5900_m
    #[must_use]
    pub fn field_r5900_m_unchecked(&self) -> u8 {
        EncodedFieldMask::r5900_m
            .get_shifted(self.word())
            .try_into()
            .unwrap()
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
        let instr = Instruction::new_no_extension(
            0x08000004,
            0x80000000,
            InstructionFlags::default(),
            IsaVersion::MIPS_III,
        );
        assert!(instr.is_valid());
        assert_eq!(instr.opcode_category(), OpcodeCategory::CORE_NORMAL);
        assert_eq!(instr.opcode(), Opcode::core_j);
        assert!(instr.opcode().is_jump());

        assert_eq!(
            Instruction::new_no_extension(
                0x08000000,
                0x80000000,
                InstructionFlags::default(),
                IsaVersion::MIPS_III
            )
            .opcode(),
            Opcode::core_j
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
        assert!(instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::core_jal);
    }

    #[test]
    fn check_lwu() {
        // lwu was introduced in MIPS III
        let flags = InstructionFlags::default();

        let instr =
            Instruction::new_no_extension(0x9C000000, 0x80000000, flags, IsaVersion::MIPS_III);
        assert!(instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::core_lwu);

        let instr =
            Instruction::new_no_extension(0x9C000000, 0x80000000, flags, IsaVersion::MIPS_II);
        assert!(!instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::ALL_INVALID);
    }

    #[test]
    fn check_invalid() {
        let instr = Instruction::new_no_extension(
            0x0000072E,
            0x80000000,
            InstructionFlags::default(),
            IsaVersion::MIPS_III,
        );
        assert!(!instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::core_dsub);
    }
}
