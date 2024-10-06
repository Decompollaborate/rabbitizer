/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::registers::*;
use crate::traits::{R4000AllegrexVectorRegister, Register};
use crate::utils;
use crate::Abi;
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

    #[must_use]
    pub const fn abi(&self) -> Abi {
        self.flags.abi()
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

        // TODO: check for reserved or invalid registers

        // Check there are no reminder bits besides the ones we consider valid
        // for this instruction.
        ((!valid_bits) & self.word()) == 0
    }

    /// Check if the instruction is an instruction that will always branch
    /// unconditionally.
    ///
    /// This is always true for the [`b`] instruction.
    ///
    /// Some compilers use the [`j`] instruction for unconditional branches
    /// instead of the [`b`] instruction. Treating [`j`] as an unconditional
    /// branch depends on how the [`InstructionFlags`] of this instruction
    /// instance was configured.
    ///
    /// [`b`]: crate::Opcode::core_b
    /// [`j`]: crate::Opcode::core_j
    /// [`InstructionFlags`]: crate::InstructionFlags
    #[must_use]
    pub fn is_unconditional_branch(&self) -> bool {
        match self.opcode() {
            Opcode::core_b => true,
            Opcode::core_beq => {
                // in case the b pseudo instruction is disabled
                self.field_rt_unchecked() == Gpr::zero && self.field_rs_unchecked() == Gpr::zero
            }
            Opcode::core_j => self.flags().treat_j_as_unconditional_branch,
            _ => false,
        }
    }

    /// Check if this is an instruction used for function calls.
    ///
    /// This is always true for "and link" instructions.
    ///
    /// Some compilers use the [`j`] instruction for tail call optimizations,
    /// meaning we may require to give special treatment to this instruction if
    /// we are analyzing code emitted by one of those compilers, like clearing
    /// registers after a tail call. This can be configured by turning off the
    /// `treat_j_as_unconditional_branch` option on the [`InstructionFlags`].
    ///
    /// [`j`]: crate::Opcode::core_j
    /// [`InstructionFlags`]: crate::InstructionFlags
    #[must_use]
    pub fn is_function_call(&self) -> bool {
        if self.opcode().does_link() {
            return true;
        }

        match self.opcode() {
            Opcode::core_j => !self.flags().treat_j_as_unconditional_branch,
            _ => false,
        }
    }

    /// Check if the instruction and its register is the one usually used for
    /// returning from a function.
    ///
    /// Specfically, this checks if the instruction is a `jr $ra`.
    ///
    /// Returns `false` if the instruction is not a [`jr`] or if it is a [`jr`]
    /// but the register is not [`$ra`].
    ///
    /// [`jr`]: crate::Opcode::core_jr
    /// [`$ra`]: crate::registers::Gpr::ra
    #[must_use]
    pub fn is_return(&self) -> bool {
        match self.opcode() {
            Opcode::core_jr => self.field_rs_unchecked().holds_return_address(self.abi()),
            _ => false,
        }
    }

    /// Check if the instruction and its register is the one usually used for
    /// jumping with jumptables..
    ///
    /// Specfically, this checks if the instruction is a [`jr`] but its register
    /// is not [`$ra`].
    ///
    /// Returns `false` if the instruction is not a [`jr`] or if it is a [`jr`]
    /// but the register is [`$ra`].
    ///
    /// [`jr`]: crate::Opcode::core_jr
    /// [`$ra`]: crate::registers::Gpr::ra
    #[must_use]
    pub fn is_jumptable_jump(&self) -> bool {
        match self.opcode() {
            Opcode::core_jr => !self.field_rs_unchecked().holds_return_address(self.abi()),
            _ => false,
        }
    }

    /// Tries to guess if the given instruction was handwritten by a human
    /// instead of generated by a C compiler.
    #[must_use]
    pub fn is_likely_handwritten(&self) -> bool {
        if self._handwritten_category {
            return true;
        }

        if self.opcode().not_emitted_by_compilers() {
            return true;
        }

        if let Some(reg) = self.field_rs() {
            if reg.is_kernel(self.abi()) {
                return true;
            }
        }
        if let Some(reg) = self.field_rt() {
            if reg.is_kernel(self.abi()) {
                return true;
            }
        }
        if let Some(reg) = self.field_rd() {
            if reg.is_kernel(self.abi()) {
                return true;
            }
        }

        false
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

/// Unchecked RSP opcode fields
impl Instruction {
    /// Returns the [`RspCop0`] register embedded on the `rsp_cop0d` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_rsp_cop0d`]
    /// function instead.
    ///
    /// [`RspCop0`]: crate::registers::RspCop0
    /// [`field_rsp_cop0d`]: Instruction::field_rsp_cop0d
    #[must_use]
    pub fn field_rsp_cop0d_unchecked(&self) -> RspCop0 {
        EncodedFieldMask::cop0d
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspCop2`] register embedded on the `rsp_cop2t` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_rsp_cop2t`]
    /// function instead.
    ///
    /// [`RspCop2`]: crate::registers::RspCop2
    /// [`field_rsp_cop2t`]: Instruction::field_rsp_cop2t
    #[must_use]
    pub fn field_rsp_cop2t_unchecked(&self) -> RspCop2 {
        EncodedFieldMask::cop2t
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspCop2`] register embedded on the `rsp_cop2cd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_rsp_cop2cd`]
    /// function instead.
    ///
    /// [`RspCop2`]: crate::registers::RspCop2
    /// [`field_rsp_cop2cd`]: Instruction::field_rsp_cop2cd
    #[must_use]
    pub fn field_rsp_cop2cd_unchecked(&self) -> RspCop2 {
        EncodedFieldMask::cop2cd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vs` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_rsp_vs`]
    /// function instead.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`field_rsp_vs`]: Instruction::field_rsp_vs
    #[must_use]
    pub fn field_rsp_vs_unchecked(&self) -> RspVector {
        EncodedFieldMask::rsp_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vt` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_rsp_vs`]
    /// function instead.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`field_rsp_vt`]: Instruction::field_rsp_vt
    #[must_use]
    pub fn field_rsp_vt_unchecked(&self) -> RspVector {
        EncodedFieldMask::rsp_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`RspVector`] register embedded on the `rsp_vd` field of the word
    /// of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_rsp_vd`]
    /// function instead.
    ///
    /// [`RspVector`]: crate::registers::RspVector
    /// [`field_rsp_vd`]: Instruction::field_rsp_vd
    #[must_use]
    pub fn field_rsp_vd_unchecked(&self) -> RspVector {
        EncodedFieldMask::rsp_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_elementhigh` value embedded on the `rsp_elementhigh`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`field_rsp_elementhigh`] function instead.
    ///
    /// [`field_rsp_elementhigh`]: Instruction::field_rsp_elementhigh
    #[must_use]
    pub fn field_rsp_elementhigh_unchecked(&self) -> u8 {
        EncodedFieldMask::rsp_elementhigh
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_elementlow` value embedded on the `rsp_elementlow` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`field_rsp_elementlow`] function instead.
    ///
    /// [`field_rsp_elementlow`]: Instruction::field_rsp_elementlow
    #[must_use]
    pub fn field_rsp_elementlow_unchecked(&self) -> u8 {
        EncodedFieldMask::rsp_elementlow
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_index` value embedded on the `rsp_index` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`field_rsp_index`] function instead.
    ///
    /// [`field_rsp_index`]: Instruction::field_rsp_index
    #[must_use]
    pub fn field_rsp_index_unchecked(&self) -> u8 {
        EncodedFieldMask::rsp_index
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_offset` value embedded on the `rsp_offset` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`field_rsp_offset`] function instead.
    ///
    /// [`field_rsp_offset`]: Instruction::field_rsp_offset
    #[must_use]
    pub fn field_rsp_offset_unchecked(&self) -> u8 {
        EncodedFieldMask::rsp_offset
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `rsp_de` value embedded on the `rsp_de` field of the word of
    /// this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the
    /// [`field_rsp_de`] function instead.
    ///
    /// [`field_rsp_de`]: Instruction::field_rsp_de
    #[must_use]
    pub fn field_rsp_de_unchecked(&self) -> u8 {
        EncodedFieldMask::rsp_de
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

/// Unchecked R4000ALLEGREX opcode fields
impl Instruction {
    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_s_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`field_r4000allegrex_s_vs`]: Instruction::field_r4000allegrex_s_vs
    #[must_use]
    pub fn field_r4000allegrex_s_vs_unchecked(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_s_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`field_r4000allegrex_s_vt`]: Instruction::field_r4000allegrex_s_vt
    #[must_use]
    pub fn field_r4000allegrex_s_vt_unchecked(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_s_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`field_r4000allegrex_s_vd`]: Instruction::field_r4000allegrex_s_vd
    #[must_use]
    pub fn field_r4000allegrex_s_vd_unchecked(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vt_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_s_vt_imm`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`field_r4000allegrex_s_vt_imm`]: Instruction::field_r4000allegrex_s_vt_imm
    #[must_use]
    pub fn field_r4000allegrex_s_vt_imm_unchecked(&self) -> R4000AllegrexS {
        let upper = EncodedFieldMask::r4000allegrex_vt_imm_upper.get_shifted(self.word());
        let lower = EncodedFieldMask::r4000allegrex_vt_imm_lower.get_shifted(self.word());

        ((upper << 5) | lower).try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexS`] register embedded on the `r4000allegrex_vd_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_s_vd_imm`]
    /// function instead.
    ///
    /// [`R4000AllegrexS`]: crate::registers::R4000AllegrexS
    /// [`field_r4000allegrex_s_vd_imm`]: Instruction::field_r4000allegrex_s_vd_imm
    #[must_use]
    pub fn field_r4000allegrex_s_vd_imm_unchecked(&self) -> R4000AllegrexS {
        EncodedFieldMask::r4000allegrex_vd_imm
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_p_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`field_r4000allegrex_p_vs`]: Instruction::field_r4000allegrex_p_vs
    #[must_use]
    pub fn field_r4000allegrex_p_vs_unchecked(&self) -> R4000AllegrexV2D {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_p_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`field_r4000allegrex_p_vt`]: Instruction::field_r4000allegrex_p_vt
    #[must_use]
    pub fn field_r4000allegrex_p_vt_unchecked(&self) -> R4000AllegrexV2D {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV2D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_p_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexV2D`]: crate::registers::R4000AllegrexV2D
    /// [`field_r4000allegrex_p_vd`]: Instruction::field_r4000allegrex_p_vd
    #[must_use]
    pub fn field_r4000allegrex_p_vd_unchecked(&self) -> R4000AllegrexV2D {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_t_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`field_r4000allegrex_t_vs`]: Instruction::field_r4000allegrex_t_vs
    #[must_use]
    pub fn field_r4000allegrex_t_vs_unchecked(&self) -> R4000AllegrexV3D {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_t_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`field_r4000allegrex_t_vt`]: Instruction::field_r4000allegrex_t_vt
    #[must_use]
    pub fn field_r4000allegrex_t_vt_unchecked(&self) -> R4000AllegrexV3D {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV3D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_t_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexV3D`]: crate::registers::R4000AllegrexV3D
    /// [`field_r4000allegrex_t_vd`]: Instruction::field_r4000allegrex_t_vd
    #[must_use]
    pub fn field_r4000allegrex_t_vd_unchecked(&self) -> R4000AllegrexV3D {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_q_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`field_r4000allegrex_q_vs`]: Instruction::field_r4000allegrex_q_vs
    #[must_use]
    pub fn field_r4000allegrex_q_vs_unchecked(&self) -> R4000AllegrexV4D {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_q_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`field_r4000allegrex_q_vt`]: Instruction::field_r4000allegrex_q_vt
    #[must_use]
    pub fn field_r4000allegrex_q_vt_unchecked(&self) -> R4000AllegrexV4D {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`field_r4000allegrex_q_vd`]: Instruction::field_r4000allegrex_q_vd
    #[must_use]
    pub fn field_r4000allegrex_q_vd_unchecked(&self) -> R4000AllegrexV4D {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexV4D`] register embedded on the `r4000allegrex_vt_imm`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_q_vt_imm`]
    /// function instead.
    ///
    /// [`R4000AllegrexV4D`]: crate::registers::R4000AllegrexV4D
    /// [`field_r4000allegrex_q_vt_imm`]: Instruction::field_r4000allegrex_q_vt_imm
    #[must_use]
    pub fn field_r4000allegrex_q_vt_imm_unchecked(&self) -> R4000AllegrexV4D {
        let upper = EncodedFieldMask::r4000allegrex_vt_6_imm_upper.get_shifted(self.word());
        let lower = EncodedFieldMask::r4000allegrex_vt_imm_lower.get_shifted(self.word());

        ((upper << 5) | lower).try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mp_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`field_r4000allegrex_mp_vs`]: Instruction::field_r4000allegrex_mp_vs
    #[must_use]
    pub fn field_r4000allegrex_mp_vs_unchecked(&self) -> R4000AllegrexM2x2 {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mp_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`field_r4000allegrex_mp_vt`]: Instruction::field_r4000allegrex_mp_vt
    #[must_use]
    pub fn field_r4000allegrex_mp_vt_unchecked(&self) -> R4000AllegrexM2x2 {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`field_r4000allegrex_mp_vd`]: Instruction::field_r4000allegrex_mp_vd
    #[must_use]
    pub fn field_r4000allegrex_mp_vd_unchecked(&self) -> R4000AllegrexM2x2 {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM2x2`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mp_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM2x2`]: crate::registers::R4000AllegrexM2x2
    /// [`field_r4000allegrex_mp_vs_transpose`]: Instruction::field_r4000allegrex_mp_vs_transpose
    #[must_use]
    pub fn field_r4000allegrex_mp_vs_transpose_unchecked(&self) -> R4000AllegrexM2x2 {
        // For whatever reason the transpose just toggles bit 5, no clue why.

        let value = EncodedFieldMask::r4000allegrex_vs.get_shifted(self.word()) ^ 0x20;

        value.try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mt_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`field_r4000allegrex_mt_vs`]: Instruction::field_r4000allegrex_mt_vs
    #[must_use]
    pub fn field_r4000allegrex_mt_vs_unchecked(&self) -> R4000AllegrexM3x3 {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mt_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`field_r4000allegrex_mt_vt`]: Instruction::field_r4000allegrex_mt_vt
    #[must_use]
    pub fn field_r4000allegrex_mt_vt_unchecked(&self) -> R4000AllegrexM3x3 {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`field_r4000allegrex_mt_vd`]: Instruction::field_r4000allegrex_mt_vd
    #[must_use]
    pub fn field_r4000allegrex_mt_vd_unchecked(&self) -> R4000AllegrexM3x3 {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM3x3`] register embedded on the `r4000allegrex_vs_transpose`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mt_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM3x3`]: crate::registers::R4000AllegrexM3x3
    /// [`field_r4000allegrex_mt_vs_transpose`]: Instruction::field_r4000allegrex_mt_vs_transpose
    #[must_use]
    pub fn field_r4000allegrex_mt_vs_transpose_unchecked(&self) -> R4000AllegrexM3x3 {
        // For whatever reason the transpose just toggles bit 5, no clue why.

        let value = EncodedFieldMask::r4000allegrex_vs.get_shifted(self.word()) ^ 0x20;

        value.try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mq_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`field_r4000allegrex_mq_vs`]: Instruction::field_r4000allegrex_mq_vs
    #[must_use]
    pub fn field_r4000allegrex_mq_vs_unchecked(&self) -> R4000AllegrexM4x4 {
        EncodedFieldMask::r4000allegrex_vs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vt`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mq_vt`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`field_r4000allegrex_mq_vt`]: Instruction::field_r4000allegrex_mq_vt
    #[must_use]
    pub fn field_r4000allegrex_mq_vt_unchecked(&self) -> R4000AllegrexM4x4 {
        EncodedFieldMask::r4000allegrex_vt
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_q_vd`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`field_r4000allegrex_mq_vd`]: Instruction::field_r4000allegrex_mq_vd
    #[must_use]
    pub fn field_r4000allegrex_mq_vd_unchecked(&self) -> R4000AllegrexM4x4 {
        EncodedFieldMask::r4000allegrex_vd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexM4x4`] register embedded on the `r4000allegrex_vs_transpose`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_mq_vs`]
    /// function instead.
    ///
    /// [`R4000AllegrexM4x4`]: crate::registers::R4000AllegrexM4x4
    /// [`field_r4000allegrex_mq_vs_transpose`]: Instruction::field_r4000allegrex_mq_vs_transpose
    #[must_use]
    pub fn field_r4000allegrex_mq_vs_transpose_unchecked(&self) -> R4000AllegrexM4x4 {
        // For whatever reason the transpose just toggles bit 5, no clue why.

        let value = EncodedFieldMask::r4000allegrex_vs.get_shifted(self.word()) ^ 0x20;

        value.try_into().unwrap()
    }

    /// Returns the [`R4000AllegrexVfpuControl`] register embedded on the `r4000allegrex_cop2cs`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_cop2cs`]
    /// function instead.
    ///
    /// [`R4000AllegrexVfpuControl`]: crate::registers::R4000AllegrexVfpuControl
    /// [`field_r4000allegrex_cop2cs`]: Instruction::field_r4000allegrex_cop2cs
    #[must_use]
    pub fn field_r4000allegrex_cop2cs_unchecked(&self) -> R4000AllegrexVfpuControl {
        EncodedFieldMask::r4000allegrex_cop2cs
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexVfpuControl`] register embedded on the `r4000allegrex_cop2cd`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_cop2cd`]
    /// function instead.
    ///
    /// [`R4000AllegrexVfpuControl`]: crate::registers::R4000AllegrexVfpuControl
    /// [`field_r4000allegrex_cop2cd`]: Instruction::field_r4000allegrex_cop2cd
    #[must_use]
    pub fn field_r4000allegrex_cop2cd_unchecked(&self) -> R4000AllegrexVfpuControl {
        EncodedFieldMask::r4000allegrex_cop2cd
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_pos` value embedded on the `r4000allegrex_pos` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_pos`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_pos`]: Instruction::field_r4000allegrex_pos
    #[must_use]
    pub fn field_r4000allegrex_pos_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_pos
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_size` value embedded on the `r4000allegrex_size` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_size`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_size`]: Instruction::field_r4000allegrex_size
    #[must_use]
    pub fn field_r4000allegrex_size_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_size
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_size_plus_pos` value embedded on the `r4000allegrex_size_plus_pos` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_size_plus_pos`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_size_plus_pos`]: Instruction::field_r4000allegrex_size_plus_pos
    #[must_use]
    pub fn field_r4000allegrex_size_plus_pos_unchecked(&self) -> i8 {
        EncodedFieldMask::r4000allegrex_size_plus_pos
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_imm3` value embedded on the `r4000allegrex_imm3` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_imm3`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_imm3`]: Instruction::field_r4000allegrex_imm3
    #[must_use]
    pub fn field_r4000allegrex_imm3_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_imm3
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_offset14` value embedded on the `r4000allegrex_offset14` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_offset14`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_offset14`]: Instruction::field_r4000allegrex_offset14
    #[must_use]
    pub fn field_r4000allegrex_offset14_unchecked(&self) -> u16 {
        let value: u16 = EncodedFieldMask::r4000allegrex_offset14
            .get_shifted(self.word())
            .try_into()
            .unwrap();

        value << 2
    }

    /// Returns the `r4000allegrex_wb` value embedded on the `r4000allegrex_wb` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_wb`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_wb`]: Instruction::field_r4000allegrex_wb
    #[must_use]
    pub const fn field_r4000allegrex_wb_unchecked(&self) -> bool {
        EncodedFieldMask::r4000allegrex_wb.get_shifted(self.word()) != 0
    }

    /// Returns the `r4000allegrex_vcmp_cond` value embedded on the `r4000allegrex_vcmp_cond` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_vcmp_cond`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_vcmp_cond`]: Instruction::field_r4000allegrex_vcmp_cond
    #[must_use]
    pub fn field_r4000allegrex_vcmp_cond_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_vcmp_cond
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the [`R4000AllegrexVConstant`] register embedded on the `r4000allegrex_vconstant`
    /// field of the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as this field. It is recommended to use the [`field_r4000allegrex_vconstant`]
    /// function instead.
    ///
    /// [`R4000AllegrexVConstant`]: crate::registers::R4000AllegrexVConstant
    /// [`field_r4000allegrex_vconstant`]: Instruction::field_r4000allegrex_vconstant
    #[must_use]
    pub fn field_r4000allegrex_vconstant_unchecked(&self) -> R4000AllegrexVConstant {
        EncodedFieldMask::r4000allegrex_vconstant
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_power_of_two` value embedded on the `r4000allegrex_power_of_two` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_power_of_two`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_power_of_two`]: Instruction::field_r4000allegrex_power_of_two
    #[must_use]
    pub fn field_r4000allegrex_power_of_two_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_power_of_two
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_vfpu_cc_bit` value embedded on the `r4000allegrex_vfpu_cc_bit` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_vfpu_cc_bit`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_vfpu_cc_bit`]: Instruction::field_r4000allegrex_vfpu_cc_bit
    #[must_use]
    pub fn field_r4000allegrex_vfpu_cc_bit_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_vfpu_cc_bit
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_bn` value embedded on the `r4000allegrex_bn` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_bn`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_bn`]: Instruction::field_r4000allegrex_bn
    #[must_use]
    pub fn field_r4000allegrex_bn_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_bn
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_intfloat16` value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_intfloat16`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_intfloat16`]: Instruction::field_r4000allegrex_intfloat16
    #[must_use]
    pub fn field_r4000allegrex_intfloat16_unchecked(&self) -> u16 {
        EncodedFieldMask::r4000allegrex_intfloat16
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction, but interpreted as a 16 bits signed value.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`field_r4000allegrex_int16`] function instead.
    ///
    /// [`field_r4000allegrex_int16`]: Instruction::field_r4000allegrex_int16
    #[must_use]
    pub fn field_r4000allegrex_int16_unchecked(&self) -> i16 {
        utils::from_2s_complement(self.field_r4000allegrex_intfloat16_unchecked() as u32, 16) as i16
    }

    /// Returns the value embedded on the `r4000allegrex_intfloat16` field of
    /// the word of this instruction, but interpreted as a 16 bits float.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the
    /// [`field_r4000allegrex_float16`] function instead.
    ///
    /// [`field_r4000allegrex_float16`]: Instruction::field_r4000allegrex_float16
    #[must_use]
    pub fn field_r4000allegrex_float16_unchecked(&self) -> f32 {
        // Ideally this function would return a f16, but that type is not stable yet.

        let hex = utils::floatrepr_32_from_16(self.field_r4000allegrex_intfloat16_unchecked());

        f32::from_bits(hex)
    }

    /// Returns the `r4000allegrex_vrot_code` value embedded on the `r4000allegrex_vrot_code` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_vrot_code`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_vrot_code`]: Instruction::field_r4000allegrex_vrot_code
    #[must_use]
    pub fn field_r4000allegrex_vrot_code_unchecked(&self) -> u8 {
        EncodedFieldMask::r4000allegrex_vrot_code
            .get_shifted(self.word())
            .try_into()
            .unwrap()
    }

    /// Returns the `r4000allegrex_wpx` value embedded on the `r4000allegrex_wpx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_wpx`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_wpx`]: Instruction::field_r4000allegrex_wpx
    #[must_use]
    pub const fn field_r4000allegrex_wpx_unchecked(&self) -> u8 {
        let c = ((self.word() & utils::bitmask(8, 1)) >> 8) as u8;
        let d = (self.word() & utils::bitmask(0, 2)) as u8;

        (c << 2) | d
    }

    /// Returns the `r4000allegrex_wpy` value embedded on the `r4000allegrex_wpy` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_wpy`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_wpy`]: Instruction::field_r4000allegrex_wpy
    #[must_use]
    pub const fn field_r4000allegrex_wpy_unchecked(&self) -> u8 {
        let c = ((self.word() & utils::bitmask(9, 1)) >> 9) as u8;
        let d = ((self.word() & utils::bitmask(2, 2)) >> 2) as u8;

        (c << 2) | d
    }

    /// Returns the `r4000allegrex_wpz` value embedded on the `r4000allegrex_wpz` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_wpz`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_wpz`]: Instruction::field_r4000allegrex_wpz
    #[must_use]
    pub const fn field_r4000allegrex_wpz_unchecked(&self) -> u8 {
        let c = ((self.word() & utils::bitmask(10, 1)) >> 10) as u8;
        let d = ((self.word() & utils::bitmask(4, 2)) >> 4) as u8;

        (c << 2) | d
    }

    /// Returns the `r4000allegrex_wpw` value embedded on the `r4000allegrex_wpw` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_wpw`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_wpw`]: Instruction::field_r4000allegrex_wpw
    #[must_use]
    pub const fn field_r4000allegrex_wpw_unchecked(&self) -> u8 {
        let c = ((self.word() & utils::bitmask(11, 1)) >> 11) as u8;
        let d = ((self.word() & utils::bitmask(6, 2)) >> 6) as u8;

        (c << 2) | d
    }

    /// Returns the `r4000allegrex_rpx` value embedded on the `r4000allegrex_rpx` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_rpx`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_rpx`]: Instruction::field_r4000allegrex_rpx
    #[must_use]
    pub const fn field_r4000allegrex_rpx_unchecked(&self) -> u8 {
        let a = ((self.word() & utils::bitmask(16, 1)) >> 16) as u8;
        let b = ((self.word() & utils::bitmask(12, 1)) >> 12) as u8;
        let c = ((self.word() & utils::bitmask(8, 1)) >> 8) as u8;
        let d = (self.word() & utils::bitmask(0, 2)) as u8;

        (a << 4) | (b << 3) | (c << 2) | d
    }

    /// Returns the `r4000allegrex_rpy` value embedded on the `r4000allegrex_rpy` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_rpy`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_rpy`]: Instruction::field_r4000allegrex_rpy
    #[must_use]
    pub const fn field_r4000allegrex_rpy_unchecked(&self) -> u8 {
        let a = ((self.word() & utils::bitmask(17, 1)) >> 17) as u8;
        let b = ((self.word() & utils::bitmask(13, 1)) >> 13) as u8;
        let c = ((self.word() & utils::bitmask(9, 1)) >> 9) as u8;
        let d = ((self.word() & utils::bitmask(2, 2)) >> 2) as u8;

        (a << 4) | (b << 3) | (c << 2) | d
    }

    /// Returns the `r4000allegrex_rpz` value embedded on the `r4000allegrex_rpz` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_rpz`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_rpz`]: Instruction::field_r4000allegrex_rpz
    #[must_use]
    pub const fn field_r4000allegrex_rpz_unchecked(&self) -> u8 {
        let a = ((self.word() & utils::bitmask(18, 1)) >> 18) as u8;
        let b = ((self.word() & utils::bitmask(14, 1)) >> 14) as u8;
        let c = ((self.word() & utils::bitmask(10, 1)) >> 10) as u8;
        let d = ((self.word() & utils::bitmask(4, 2)) >> 4) as u8;

        (a << 4) | (b << 3) | (c << 2) | d
    }

    /// Returns the `r4000allegrex_rpw` value embedded on the `r4000allegrex_rpw` field of
    /// the word of this instruction.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually has this field, meaning that calling this function on an
    /// instruction that does not have this field will interpret garbage data
    /// as the return value. It is recommended to use the [`field_r4000allegrex_rpw`]
    /// function instead.
    ///
    /// [`field_r4000allegrex_rpw`]: Instruction::field_r4000allegrex_rpw
    #[must_use]
    pub const fn field_r4000allegrex_rpw_unchecked(&self) -> u8 {
        let a = ((self.word() & utils::bitmask(19, 1)) >> 19) as u8;
        let b = ((self.word() & utils::bitmask(15, 1)) >> 15) as u8;
        let c = ((self.word() & utils::bitmask(11, 1)) >> 11) as u8;
        let d = ((self.word() & utils::bitmask(6, 2)) >> 6) as u8;

        (a << 4) | (b << 3) | (c << 2) | d
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

    /// Get the target vram address this instruction jumps to.
    /// This function is intended only for direct jump instructions.
    #[must_use]
    pub fn get_instr_index_as_vram(&self) -> Option<u32> {
        self.field_instr_index()
            .map(|instr_index| self.vram_from_instr_index(instr_index))
    }

    /// Get the target vram address this instruction jumps to.
    /// This function is intended only for direct jump instructions.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually corresponds to a jump, meaning that calling this function
    /// on an instruction that does not have this field will interpret garbage
    /// data as the returned value. It is recommended to use the
    /// [`get_instr_index_as_vram`] function instead.
    ///
    /// [`get_instr_index_as_vram`]: Instruction::get_instr_index_as_vram
    #[must_use]
    pub const fn get_instr_index_as_vram_unchecked(&self) -> u32 {
        self.vram_from_instr_index(self.field_instr_index_unchecked())
    }

    /// Returns the offset (in bytes) that the branch instruction would branch
    /// to, relative to the instruction itself. This method is intended only
    /// for branch instructions.
    ///
    /// The returned value can be negative, meaning the branch instructions
    /// does a backwards branch.
    ///
    /// To get the branch offset of either a branch instruction or a jump instruction
    /// use [`get_branch_offset_generic`] instead.
    ///
    /// [`get_branch_offset_generic`]: Instruction::get_branch_offset_generic
    #[must_use]
    pub fn get_branch_offset(&self) -> Option<i32> {
        if self
            .opcode()
            .has_operand_alias(Operand::core_branch_target_label)
        {
            Some(self.get_branch_offset_unchecked())
        } else {
            None
        }
    }

    /// Returns the offset (in bytes) that the branch instruction would branch
    /// to, relative to the instruction itself. This method is intended only
    /// for branch instructions.
    ///
    /// The returned value can be negative, meaning the branch instructions
    /// does a backwards branch.
    ///
    /// To get the branch offset of either a branch instruction or a jump instruction
    /// use [`get_branch_offset_generic`] instead.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually corresponds to a branch, meaning that calling this function
    /// on an instruction that does not have this field will interpret garbage
    /// data as the returned value. It is recommended to use the
    /// [`get_branch_offset`] function instead.
    ///
    /// [`get_branch_offset_generic_unchecked`]: Instruction::get_branch_offset_generic_unchecked
    /// [`get_branch_offset`]: Instruction::get_branch_offset
    #[must_use]
    pub fn get_branch_offset_unchecked(&self) -> i32 {
        let imm = self.field_immediate_unchecked();
        let diff = utils::from_2s_complement(imm as u32, 16);

        (diff + 1) * 4
    }

    /// Returns the offset (in bytes) that the instruction would branch,
    /// relative to the instruction itself. This method is intended for both branch
    /// and jump instructions.
    ///
    /// The returned value can be either positive or negative.
    #[must_use]
    pub fn get_branch_offset_generic(&self) -> Option<i32> {
        if self.opcode().has_operand_alias(Operand::core_label) {
            Some(self.get_instr_index_as_vram_unchecked() as i32 - self.vram as i32)
        } else {
            self.get_branch_offset()
        }
    }

    /// Get the target vram address this instruction jumps to.
    /// This method is intended only for branch or direct jump instructions.
    #[must_use]
    pub fn get_branch_vram_generic(&self) -> Option<u32> {
        if self
            .opcode()
            .has_operand_alias(Operand::core_branch_target_label)
        {
            Some((self.vram as i32 + self.get_branch_offset_unchecked()) as u32)
        } else {
            self.get_instr_index_as_vram()
        }
    }

    /// Returns the general purpose register ([`Gpr`]) which this instruction
    /// modifies, or a negative value if the instruction does not modify the
    /// state of any [`Gpr`].
    ///
    /// [`Gpr`]: crate::registers::Gpr
    #[must_use]
    pub fn get_destination_gpr(&self) -> Option<Gpr> {
        if self.opcode().modifies_rd() {
            Some(self.field_rd_unchecked())
        } else if self.opcode().modifies_rt() {
            Some(self.field_rt_unchecked())
        } else if self.opcode().modifies_rs() {
            Some(self.field_rs_unchecked())
        } else {
            None
        }
    }

    /// Returns `true` if the [`Gpr`] which is modified by this register is
    /// [`$zero`].
    /// Returns `false` if this instruction does not modify a [`Gpr`] or the
    /// modified [`Gpr`] is not [`$zero`].
    ///
    /// [`Gpr`]: crate::registers::Gpr
    /// [`$zero`]: crate::registers::Gpr::zero
    #[must_use]
    pub fn outputs_to_gpr_zero(&self) -> bool {
        self.get_destination_gpr() == Some(Gpr::zero)
    }
}

impl Instruction {
    #[must_use]
    pub fn get_processed_rsp_offset_unchecked(&self) -> u16 {
        let offset: u16 = self.field_rsp_offset_unchecked().into();

        // TODO: do not depend on the opcode to process the offset itself.
        match self.opcode() {
            Opcode::rsp_lsv | Opcode::rsp_ssv => offset << 1,

            Opcode::rsp_llv | Opcode::rsp_slv => offset << 2,

            Opcode::rsp_ldv
            | Opcode::rsp_sdv
            | Opcode::rsp_lpv
            | Opcode::rsp_spv
            | Opcode::rsp_luv
            | Opcode::rsp_suv => offset << 3,

            Opcode::rsp_lqv
            | Opcode::rsp_sqv
            | Opcode::rsp_lrv
            | Opcode::rsp_srv
            | Opcode::rsp_lhv
            | Opcode::rsp_shv
            | Opcode::rsp_lfv
            | Opcode::rsp_sfv
            | Opcode::rsp_ltv
            | Opcode::rsp_stv
            | Opcode::rsp_swv => offset << 4,

            _ => offset,
        }
    }
}

impl Instruction {
    pub(crate) fn process_r4000allegrex_vcmp_operands<T>(
        cond: u8,
        vs: T,
        vt: T,
    ) -> (u8, Option<T>, Option<T>)
    where
        T: R4000AllegrexVectorRegister,
    {
        match cond {
            0 | 4 => {
                // We can omit those two registers if they both are zero
                if vs == T::default() && vt == T::default() {
                    (cond, None, None)
                } else {
                    (cond, Some(vs), Some(vt))
                }
            }
            1 | 2 | 3 | 5 | 6 | 7 => (cond, Some(vs), Some(vt)),
            8..=15 => {
                // We can omit the vt register if it is zero.
                if vt == T::default() {
                    (cond, Some(vs), None)
                } else {
                    (cond, Some(vs), Some(vt))
                }
            }
            _ => unreachable!(
                "Unhandled field_r4000allegrex_vcmp_cond_unchecked value? '{}'",
                cond
            ),
        }
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_s_args(
        &self,
    ) -> Option<(u8, Option<R4000AllegrexS>, Option<R4000AllegrexS>)> {
        if self
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt)
        {
            Some(self.get_r4000allegrex_vcmp_s_args_unchecked())
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_s_args_unchecked(
        &self,
    ) -> (u8, Option<R4000AllegrexS>, Option<R4000AllegrexS>) {
        let cond = self.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = self.field_r4000allegrex_s_vs_unchecked();
        let vt = self.field_r4000allegrex_s_vs_unchecked();

        Self::process_r4000allegrex_vcmp_operands(cond, vs, vt)
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_p_args(
        &self,
    ) -> Option<(u8, Option<R4000AllegrexV2D>, Option<R4000AllegrexV2D>)> {
        if self
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt)
        {
            Some(self.get_r4000allegrex_vcmp_p_args_unchecked())
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_p_args_unchecked(
        &self,
    ) -> (u8, Option<R4000AllegrexV2D>, Option<R4000AllegrexV2D>) {
        let cond = self.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = self.field_r4000allegrex_p_vs_unchecked();
        let vt = self.field_r4000allegrex_p_vs_unchecked();

        Self::process_r4000allegrex_vcmp_operands(cond, vs, vt)
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_t_args(
        &self,
    ) -> Option<(u8, Option<R4000AllegrexV3D>, Option<R4000AllegrexV3D>)> {
        if self
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt)
        {
            Some(self.get_r4000allegrex_vcmp_t_args_unchecked())
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_t_args_unchecked(
        &self,
    ) -> (u8, Option<R4000AllegrexV3D>, Option<R4000AllegrexV3D>) {
        let cond = self.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = self.field_r4000allegrex_t_vs_unchecked();
        let vt = self.field_r4000allegrex_t_vs_unchecked();

        Self::process_r4000allegrex_vcmp_operands(cond, vs, vt)
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_q_args(
        &self,
    ) -> Option<(u8, Option<R4000AllegrexV4D>, Option<R4000AllegrexV4D>)> {
        if self
            .opcode()
            .has_operand_alias(Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt)
        {
            Some(self.get_r4000allegrex_vcmp_q_args_unchecked())
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_r4000allegrex_vcmp_q_args_unchecked(
        &self,
    ) -> (u8, Option<R4000AllegrexV4D>, Option<R4000AllegrexV4D>) {
        let cond = self.field_r4000allegrex_vcmp_cond_unchecked();
        let vs = self.field_r4000allegrex_q_vs_unchecked();
        let vt = self.field_r4000allegrex_q_vs_unchecked();

        Self::process_r4000allegrex_vcmp_operands(cond, vs, vt)
    }
}

// impl Instruction {
//     pub fn blank_out(&mut self) {
//     }
// }

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
