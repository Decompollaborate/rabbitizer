/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::abi::Abi;
use crate::display_flags::InstructionDisplayFlags;
use crate::encoded_field_mask::EncodedFieldMask;
use crate::instr::{InstrField, InstructionDisplay, InstructionFlags};
#[cfg(any(
    feature = "RSP",
    feature = "R3000GTE",
    feature = "R4000ALLEGREX",
    feature = "R5900EE",
))]
use crate::isa::IsaExtension;
use crate::isa::IsaVersion;
use crate::opcodes::{Opcode, OpcodeCategory, OpcodeDecoder};
use crate::operands::{Operand, OperandIterator, ValuedOperandIterator};
use crate::registers::*;
use crate::registers_meta::Register;
use crate::vram::{Vram, VramOffset};

/// A MIPS instruction.
///
/// For customization of a given instruction refer to the [`InstructionFlags`] struct.
///
/// For disassembling / displaying this instruction as an matching assemblable string (that would
/// encode back to the original `word`) refer to the [`display`] function.
///
/// An instance of this struct contains the decoded information corresponding to a 32bits `word`.
/// The passed `word` may not map to a valid MIPS instruction. To check validity of the decoded
/// `word` use the [`is_valid`] function. Using any of the inspection methods on an invalid
/// instruction will return garbage data, but [`display`]ing the instruction will fallback to
/// produce a string use `.word` statements with the original raw word as a way to still produce
/// matching assembly. To get an instance of a MIPS instruction that always get checked for
/// validity during construction use the [`new_checked`] function instead of the [`new`] function.
///
/// Use [`valued_operands_iter`] to iterate over the operands of the instruction.
///
/// # Examples
///
/// ### Plain disassembly
///
/// ```
/// use rabbitizer::{Instruction, Vram, InstructionFlags, InstructionDisplayFlags};
/// use rabbitizer::isa::IsaVersion;
/// use rabbitizer::opcodes::Opcode;
///
/// let vram = Vram::new(0x80000000);
/// let flags = InstructionFlags::new(IsaVersion::MIPS_I);
/// let word = 0x3C088001;
/// let instr = Instruction::new(word, vram, flags);
///
/// assert_eq!(instr.opcode(), Opcode::core_lui);
///
/// let display_flags = InstructionDisplayFlags::new();
/// let display = instr.display::<&str>(&display_flags, None, 0);
/// assert_eq!(&display.to_string(), "lui         $t0, 0x8001");
/// ```
///
/// ### Managing pseudo instructions
///
/// ```
/// use rabbitizer::{Instruction, Vram, InstructionFlags, InstructionDisplayFlags};
/// use rabbitizer::isa::IsaVersion;
/// use rabbitizer::opcodes::Opcode;
///
/// let vram = Vram::new(0x80000000);
/// let flags = InstructionFlags::new(IsaVersion::MIPS_I);
///
/// // Specify the same word for both Instruction instances.
/// let word = 0x00025022;
/// let instr_raw = Instruction::new(word, vram, flags.with_all_pseudos(false));
/// let instr_pseudo = Instruction::new(word, vram, flags.with_all_pseudos(true));
///
/// // The words get decoded as different opcodes due to the different flags.
/// assert_eq!(instr_raw.opcode(), Opcode::core_sub);
/// assert_eq!(instr_pseudo.opcode(), Opcode::core_neg);
///
/// let display_flags = InstructionDisplayFlags::new();
/// assert_eq!(&instr_raw.display::<&str>(&display_flags, None, 0).to_string(),    "sub         $t2, $zero, $v0");
/// assert_eq!(&instr_pseudo.display::<&str>(&display_flags, None, 0).to_string(), "neg         $t2, $v0");
/// ```
///
/// [`display`]: Instruction::display
/// [`is_valid`]: Instruction::is_valid
/// [`InstructionFlags`]: crate::instr::InstructionFlags
/// [`new`]: Instruction::new
/// [`new_checked`]: Instruction::new_checked
/// [`valued_operands_iter`]: Instruction::valued_operands_iter
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    word: u32,
    vram: Vram,

    opcode_decoder: OpcodeDecoder,

    flags: InstructionFlags,
}

/// Constructors
impl Instruction {
    /// Decodes a 32bits `word` as a MIPS instruction.
    ///
    /// This function does not worry about endianness of the passed `word`. If the user is reading
    /// data from a binary byte file stream (i.e. a ROM, an ELF, etc) it is up to the user to
    /// ensure the bytes read from the file stream are decoded using the properly endianness.
    /// Functions like [`u32::from_be_bytes`] and [`u32::from_le_bytes`] are recommended for
    /// handling endianness in such cases.
    ///
    /// The `vram` (Virtual RAM) argument corresponds to the virtual address associated to this
    /// instruction. It is okay for this argument to be a dummy value in the case the actual vram
    /// of this instruction is not known (i.e. disassembling a relocatable ELF), since this value
    /// is not used during the decoding process, but it may produce unexpected results on methods
    /// that rely on this value, like jump/branch inspection methods (i.e.
    /// [`get_instr_index_as_vram`], [`get_branch_offset_generic`], [`get_branch_vram_generic`]) or
    /// when [`display`]ing jump instructions. If a dummy value is wanted to be used either way, it
    /// is recommended to use a number that is a multiple of 4.
    ///
    /// The `flags` argument allows configuring the behavior of some instruction-inspection related
    /// methods (i.e. [`is_unconditional_branch`]) or affect how the passed `word` should be
    /// decoded, like (dis)allowing pseudo instructions or which [`IsaVersion`] / [`IsaExtension`]
    /// pair should be following for decoding.
    ///
    /// In the case the passed `word` does not map to a valid instruction then this function will
    /// not panic or error in any way. To check that the given word is actually a valid instruction
    /// use the [`is_valid`] method.
    ///
    /// [`is_valid`]: Instruction::is_valid
    /// [`get_instr_index_as_vram`]: Instruction::get_instr_index_as_vram
    /// [`get_branch_offset_generic`]: Instruction::get_branch_offset_generic
    /// [`get_branch_vram_generic`]: Instruction::get_branch_vram_generic
    /// [`is_unconditional_branch`]: Instruction::is_unconditional_branch
    /// [`display`]: Instruction::display
    /// [`IsaVersion`]: crate::isa::IsaVersion
    /// [`IsaExtension`]: crate::isa::IsaExtension
    /// [`u32::from_be_bytes`]: u32::from_be_bytes
    /// [`u32::from_le_bytes`]: u32::from_le_bytes
    #[must_use]
    pub const fn new(word: u32, vram: Vram, flags: InstructionFlags) -> Self {
        let opcode_decoder = OpcodeDecoder::decode(
            word,
            flags.decoding_flags(),
            flags.isa_version(),
            flags.isa_extension(),
        );

        Self {
            word,
            vram,
            opcode_decoder,
            flags,
        }
    }

    /// Like the [`new`] function, but returns [`None`] if the passed `word` does not map to a
    /// valid MIPS instruction.
    ///
    /// [`new`]: Instruction::new
    /// [`None`]: Option::None
    #[must_use]
    pub fn new_checked(word: u32, vram: Vram, flags: InstructionFlags) -> Option<Self> {
        let instr = Self::new(word, vram, flags);

        instr.is_valid().then_some(instr)
    }

    #[cfg(feature = "encoder")]
    #[must_use]
    pub(crate) const fn from_raw_parts(
        word: u32,
        vram: Vram,
        opcode_decoder: OpcodeDecoder,
        flags: InstructionFlags,
    ) -> Self {
        Self {
            word,
            vram,
            opcode_decoder,
            flags,
        }
    }
}

/// Getters and setters
impl Instruction {
    /// The raw 32bits word corresponding to this MIPS instruction.
    #[must_use]
    pub const fn word(&self) -> u32 {
        self.word
    }

    /// The vram (Virtual RAM) address for this instruction.
    #[must_use]
    pub const fn vram(&self) -> Vram {
        self.vram
    }

    /// The Instruction Set Architecture (ISA) version used to decode this instruction.
    ///
    /// This value does not necessarily correspond to the ISA version that introduced the opcode
    /// of this instruction. To get that information instead use the [`Opcode::isa_version`] method
    /// instead. To retrieve the opcode of this instruction use the [`opcode`] method.
    ///
    /// [`Opcode::isa_version`]: crate::opcodes::Opcode::isa_version
    /// [`opcode`]: Instruction::opcode
    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        self.flags.isa_version()
    }

    /// The Instruction Set Architecture (ISA) extension used to decode this instruction.
    ///
    /// This value does not necessarily correspond to the ISA extension that introduced the opcode
    /// of this instruction. To get that information instead use the [`Opcode::isa_extension`] method
    /// instead. To retrieve the opcode of this instruction use the [`opcode`] method.
    ///
    /// [`Opcode::isa_extension`]: crate::opcodes::Opcode::isa_extension
    /// [`opcode`]: Instruction::opcode
    #[cfg(any(
        feature = "RSP",
        feature = "R3000GTE",
        feature = "R4000ALLEGREX",
        feature = "R5900EE",
    ))]
    #[must_use]
    pub const fn isa_extension(&self) -> Option<IsaExtension> {
        self.flags.isa_extension()
    }

    /// The Opcode for this instruction.
    #[must_use]
    pub const fn opcode(&self) -> Opcode {
        self.opcode_decoder.opcode()
    }

    /// The category of the decoded [`Opcode`].
    ///
    /// This categorization is a construct completely made up by rabbitizer (this crate) as a way
    /// to help debugging the decoding process. This information is usually not useful for
    /// consumers of this crate.
    ///
    /// [`Opcode`]: crate::opcodes::Opcode.
    #[must_use]
    pub const fn opcode_category(&self) -> OpcodeCategory {
        self.opcode_decoder.opcode_category()
    }

    /// The flags used to initialize this Instruction.
    #[must_use]
    pub const fn flags(&self) -> &InstructionFlags {
        &self.flags
    }

    /// The Abi configured for this Instruction.
    #[must_use]
    pub const fn abi(&self) -> Abi {
        self.flags.abi()
    }
}

impl Instruction {
    /// Returns an object that implements [`Display`].
    ///
    /// The `display_flags` parameter allows customizing how the instruction will be disassembled.
    ///
    /// The `imm_override` parameter allows to replace the immediate, function or label field of
    /// the instruction with the passed [`Display`]able object, if any. This is usually useful for
    /// disassembling this instruction using proper relocation arguments or actual symbols. Note
    /// that, if a string is passed, it will be used as-is, so if you want to use relocation
    /// operators then you need to provide them yourself.
    ///
    /// The `extra_ljust` parameter allows controlling the spacing between the instruction and its
    /// operands. For example, it is common to adjust this spacing for instructions on delay slots.
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::{Instruction, Vram, InstructionFlags, InstructionDisplayFlags};
    /// use rabbitizer::isa::IsaVersion;
    ///
    /// let vram = Vram::new(0x80000000);
    /// let flags = InstructionFlags::new(IsaVersion::MIPS_I);
    /// let instr = Instruction::new(0x3C088001, vram, flags);
    ///
    /// let display_flags = InstructionDisplayFlags::new();
    /// assert_eq!(&instr.display::<&str>(&display_flags, None, 0).to_string(), "lui         $t0, 0x8001");
    ///
    /// // Change how the disassembly of the instruction is displayed.
    /// assert_eq!(
    ///     &instr.display::<&str>(&display_flags.with_named_gpr(false).with_opcode_ljust(0), None, 0).to_string(),
    ///     "lui $8, 0x8001",
    /// );
    ///
    /// // Provide a string to override the immediate field of the instruction.
    /// assert_eq!(
    ///     &instr.display(&display_flags, Some("%hi(example)"), 0).to_string(),
    ///     "lui         $t0, %hi(example)",
    /// );
    ///
    /// // Reduce the space between the opcode and the parameters.
    /// assert_eq!(
    ///     &instr.display(&display_flags, Some("%hi(example)"), -1).to_string(),
    ///     "lui        $t0, %hi(example)",
    /// );
    /// ```
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn display<'ins, 'flg, T>(
        &'ins self,
        display_flags: &'flg InstructionDisplayFlags,
        imm_override: Option<T>,
        extra_ljust: i32,
    ) -> InstructionDisplay<'ins, 'flg, T>
    where
        T: fmt::Display,
    {
        InstructionDisplay::new(self, display_flags, imm_override, extra_ljust)
    }
}

/// Instruction examination
impl Instruction {
    /// Check if this instruction is the `nop` opcode.
    #[must_use]
    pub const fn is_nop(&self) -> bool {
        OpcodeDecoder::is_nop(self.word)
    }

    /// Returns a bitmask specifying which bits are allowed to be turned on for
    /// this instruction.
    ///
    /// This is useful to check if a word has bits turned on that should be zero.
    #[must_use]
    pub fn valid_bits(&self) -> EncodedFieldMask {
        self.opcode_decoder.mandatory_bits() | self.opcode().valid_bits()
    }

    /// Returns `true` if the decoded `word` is a valid instruction.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        if !self.opcode_decoder.is_valid(self.flags.decoding_flags()) {
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
    /// [`b`]: crate::opcodes::Opcode::core_b
    /// [`j`]: crate::opcodes::Opcode::core_j
    /// [`InstructionFlags`]: crate::instr::InstructionFlags
    #[must_use]
    pub fn is_unconditional_branch(&self) -> bool {
        match self.opcode() {
            Opcode::core_b => true,
            Opcode::core_beq => {
                // in case the b pseudo instruction is disabled
                self.field().rt_impl() == Gpr::zero && self.field().rs_impl() == Gpr::zero
            }
            Opcode::core_j => self.flags().j_as_branch(),
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
    /// [`j_as_branch`] option on the [`InstructionFlags`].
    ///
    /// [`j`]: crate::opcodes::Opcode::core_j
    /// [`j_as_branch`]: crate::instr::InstructionFlags::with_j_as_branch
    /// [`InstructionFlags`]: crate::instr::InstructionFlags
    #[must_use]
    pub fn is_function_call(&self) -> bool {
        match self.opcode() {
            Opcode::core_j => !self.flags().j_as_branch(),
            opcode => opcode.does_link(),
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
    /// [`jr`]: crate::opcodes::Opcode::core_jr
    /// [`$ra`]: crate::registers::Gpr::ra
    #[must_use]
    pub fn is_return(&self) -> bool {
        match self.opcode() {
            Opcode::core_jr => self.field().rs_impl().holds_return_address(self.abi()),
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
    /// [`jr`]: crate::opcodes::Opcode::core_jr
    /// [`$ra`]: crate::registers::Gpr::ra
    #[must_use]
    pub fn is_jumptable_jump(&self) -> bool {
        match self.opcode() {
            Opcode::core_jr => !self.field().rs_impl().holds_return_address(self.abi()),
            _ => false,
        }
    }

    /// Tries to guess if the given instruction was handwritten by a human
    /// instead of generated by a C compiler.
    #[must_use]
    pub fn is_likely_handwritten(&self) -> bool {
        if self.opcode_category().handwritten_category() {
            return true;
        }

        if self.opcode().not_emitted_by_compilers() {
            return true;
        }

        if let Some(reg) = self.field().rs() {
            if reg.is_kernel(self.abi()) {
                return true;
            }
        }
        if let Some(reg) = self.field().rt() {
            if reg.is_kernel(self.abi()) {
                return true;
            }
        }
        if let Some(reg) = self.field().rd() {
            if reg.is_kernel(self.abi()) {
                return true;
            }
        }

        false
    }
}

/// Opcode examination
impl Instruction {
    /// Returns an iterator which yields [`ValuedOperand`]s.
    ///
    /// [`ValuedOperand`]: crate::operands::ValuedOperand
    #[must_use]
    pub fn valued_operands_iter(&self) -> ValuedOperandIterator<'_> {
        ValuedOperandIterator::new(self)
    }

    /// Returns an iterator which yields [`Operand`]s.
    ///
    /// It is recommended to use [`valued_operands_iter`] instead.
    ///
    /// [`Operand`]: crate::operands::Operand
    /// [`valued_operands_iter`]: Instruction::valued_operands_iter
    #[must_use]
    pub fn operands_iter(&self) -> OperandIterator<'_> {
        self.opcode().operands_iter()
    }

    /// Allow accessing any of the fields embedded in an [`Instruction`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::{Instruction, Vram, InstructionFlags};
    /// use rabbitizer::isa::IsaVersion;
    /// use rabbitizer::registers::Gpr;
    ///
    /// let vram = Vram::new(0x80000000);
    /// let flags = InstructionFlags::new(IsaVersion::MIPS_I);
    /// let word = 0x3C088001;
    /// let instr = Instruction::new(word, vram, flags);
    ///
    /// let rt = instr.field().rt();
    /// assert_eq!(rt, Some(Gpr::t0));
    /// ```
    #[must_use]
    pub const fn field(&self) -> InstrField<'_> {
        InstrField::new(self)
    }
}

impl Instruction {
    #[must_use]
    #[inline(always)]
    const fn vram_from_instr_index(&self, instr_index: u32) -> Vram {
        let vram_raw = instr_index << 2;
        // Jumps are PC-region branches. The upper bits are filled with the address in the delay slot
        let upper_bits = (self.vram.inner() + 4) & 0xF0000000;

        Vram::new(upper_bits | vram_raw)
    }

    /// Get the target [`Vram`] address this instruction jumps to.
    /// This function is intended only for direct jump instructions.
    ///
    /// [`Vram`]: crate::vram::Vram
    #[must_use]
    pub fn get_instr_index_as_vram(&self) -> Option<Vram> {
        self.field()
            .instr_index()
            .map(|instr_index| self.vram_from_instr_index(instr_index))
    }

    /// Get the target [`Vram`] address this instruction jumps to.
    /// This function is intended only for direct jump instructions.
    ///
    /// Note this function **does not check** if the opcode of this instruction
    /// actually corresponds to a jump, meaning that calling this function
    /// on an instruction that does not have this field will interpret garbage
    /// data as the returned value. It is recommended to use the
    /// [`get_instr_index_as_vram`] function instead.
    ///
    /// [`get_instr_index_as_vram`]: Instruction::get_instr_index_as_vram
    /// [`Vram`]: crate::vram::Vram
    #[must_use]
    pub(crate) const fn get_instr_index_as_vram_impl(&self) -> Vram {
        self.vram_from_instr_index(self.field().instr_index_impl())
    }

    /// Returns the offset (in bytes) that the branch instruction would branch
    /// to, relative to the instruction itself, as a [`VramOffset`] instance.
    /// This method is intended only for branch instructions.
    ///
    /// The returned value can be negative, meaning the branch instructions
    /// does a backwards branch.
    ///
    /// To get the branch offset of either a branch instruction or a jump instruction
    /// use [`get_branch_offset_generic`] instead.
    ///
    /// [`get_branch_offset_generic`]: Instruction::get_branch_offset_generic
    /// [`VramOffset`]: crate::vram::VramOffset
    #[must_use]
    pub fn get_branch_offset(&self) -> Option<VramOffset> {
        if self
            .opcode()
            .has_operand_alias(Operand::core_branch_target_label)
        {
            Some(self.get_branch_offset_impl())
        } else {
            None
        }
    }

    /// Returns the offset (in bytes) that the branch instruction would branch
    /// to, relative to the instruction itself, as a [`VramOffset`] instance.
    /// This method is intended only for branch instructions.
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
    /// [`get_branch_offset_generic`]: Instruction::get_branch_offset_generic
    /// [`get_branch_offset`]: Instruction::get_branch_offset
    /// [`VramOffset`]: crate::vram::VramOffset
    #[must_use]
    pub(crate) fn get_branch_offset_impl(&self) -> VramOffset {
        let imm: i32 = self.field().imm_i16_impl().into();

        VramOffset::new((imm + 1) << 2)
    }

    /// Returns the offset (in bytes) that the instruction would branch,
    /// relative to the instruction itself, as a [`VramOffset`] instance. This
    /// method is intended for both branch and jump with address instructions.
    ///
    /// The returned value can be either positive or negative.
    ///
    /// [`VramOffset`]: crate::vram::VramOffset
    #[must_use]
    pub fn get_branch_offset_generic(&self) -> Option<VramOffset> {
        if let Some(offset) = self.get_branch_offset() {
            Some(offset)
        } else if self.flags.j_as_branch() && self.opcode() == Opcode::core_j {
            self.get_instr_index_as_vram().map(|vram| vram - self.vram)
        } else {
            None
        }
    }

    /// Get the target [`Vram`] address this instruction jumps to.
    /// This method is intended only for branch or direct jump with address
    /// instructions.
    ///
    /// [`Vram`]: crate::vram::Vram
    #[must_use]
    pub fn get_branch_vram_generic(&self) -> Option<Vram> {
        if let Some(offset) = self.get_branch_offset() {
            Some(offset + self.vram)
        } else if self.flags.j_as_branch() && self.opcode() == Opcode::core_j {
            self.get_instr_index_as_vram()
        } else {
            None
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
            Some(self.field().rd_impl())
        } else if self.opcode().modifies_rt() {
            Some(self.field().rt_impl())
        } else if self.opcode().modifies_rs() {
            Some(self.field().rs_impl())
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
    /// Clears the given operand by setting the corresponding instruction field
    /// to zero, returning a new `Instruction` instance if the instruction has
    /// the operand, otherwise `None` is returned.
    ///
    /// Note, checking if the instruction has the operand via
    /// `has_operand_alias` will still return `true` if it previously did, even
    /// after setting that field to zero.
    #[must_use]
    pub fn clear_operand(&self, operand: Operand) -> Option<Self> {
        if self.opcode().has_operand_alias(operand) {
            let mut new_instr = *self;
            new_instr.word = self.word() & !operand.mask().bits();

            Some(new_instr)
        } else {
            None
        }
    }

    /// Clears the given operand by setting the corresponding instruction field
    /// to zero, modifying the current `Instruction` instance.
    ///
    /// Note, checking if the instruction has the operand via
    /// `has_operand_alias` will still return `true` if it previously did, even
    /// after setting that field to zero.
    pub fn clear_operand_self(&mut self, operand: Operand) -> bool {
        if let Some(new_instr) = self.clear_operand(operand) {
            *self = new_instr;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::*;

    #[test]
    fn check_j() {
        let instr = Instruction::new(
            0x08000004,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_I),
        );
        assert!(instr.is_valid());
        assert_eq!(instr.opcode_category(), OpcodeCategory::CORE_NORMAL);
        assert_eq!(instr.opcode(), Opcode::core_j);
        assert!(instr.opcode().is_jump());

        assert_eq!(
            Instruction::new(
                0x08000000,
                Vram::new(0x80000000),
                InstructionFlags::new(IsaVersion::MIPS_I),
            )
            .opcode(),
            Opcode::core_j
        );
    }

    #[test]
    fn check_jal() {
        let instr = Instruction::new(
            0x0C000004,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_I),
        );
        assert!(instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::core_jal);
    }

    #[cfg(feature = "MIPS_III")]
    #[test]
    fn check_lwu() {
        // lwu was introduced in MIPS III
        let flags = InstructionFlags::new(IsaVersion::MIPS_III);

        let instr = Instruction::new(0x9C000000, Vram::new(0x80000000), flags);
        assert!(instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::core_lwu);

        let instr = Instruction::new(
            0x9C000000,
            Vram::new(0x80000000),
            flags.with_isa_version(IsaVersion::MIPS_II),
        );
        assert!(!instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::ALL_INVALID);
    }

    #[cfg(feature = "MIPS_III")]
    #[test]
    fn check_invalid() {
        let instr = Instruction::new(
            0x0000072E,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_III),
        );
        assert!(!instr.is_valid());
        assert_eq!(instr.opcode(), Opcode::core_dsub);
    }

    #[cfg(feature = "MIPS_III")]
    #[test]
    fn check_jal_is_not_branch() {
        let instr = Instruction::new(
            0x0C00E2F6,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_III),
        );

        assert!(instr.get_branch_vram_generic().is_none());
        assert!(instr.get_instr_index_as_vram().is_some());
    }

    #[test]
    fn check_clearing() {
        let mut instr = Instruction::new(
            0x0C00E2F6,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_I),
        );

        assert_eq!(
            instr.clear_operand(Operand::core_imm_i16).map(|x| x.word()),
            None,
            "jal doesn't have an immediate field, so this should return the same word"
        );
        assert_eq!(
            instr
                .clear_operand(Operand::core_branch_target_label)
                .map(|x| x.word()),
            None,
            "jal is not a branch, so this should return the same word"
        );
        assert_eq!(
            instr.clear_operand(Operand::core_label).map(|x| x.word()),
            Some(0x0C000000),
            "jal has a label field, so clearing it should zero out the lower 26 bits"
        );

        // Check we haven't modified the original instruction
        assert_eq!(
            instr.word(),
            0x0C00E2F6,
            "original instruction should remain unchanged"
        );

        // Check modifying the instruction in place.
        let changed = instr.clear_operand_self(Operand::core_label);
        assert!(changed);
        assert_eq!(instr.word(), 0x0C000000);
    }

    #[test]
    fn test_clear_operand_self_no_effect() {
        let mut instr = Instruction::new(
            0x0C00E2F6,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_I),
        );
        let changed = instr.clear_operand_self(Operand::core_imm_i16);
        assert!(!changed);
        // Should remain unchanged
        assert_eq!(instr.word(), 0x0C00E2F6);
    }

    #[test]
    fn test_clear_operand_on_branch_instruction() {
        // beq $zero, $zero, 0x10
        let instr = Instruction::new(
            0x10000004,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_I),
        );
        // Should clear the immediate field (lower 16 bits)
        let cleared = instr
            .clear_operand(Operand::core_branch_target_label)
            .unwrap();
        assert_eq!(cleared.word(), 0x10000000);
    }

    #[test]
    fn test_clear_operand_on_register_field() {
        // add $t0, $t1, $t2: opcode=0x00, rd=8, rs=9, rt=10, imm=0x20
        let instr = Instruction::new(
            0x012A4020,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_I),
        );
        // Clear the rd field (bits 11-15)
        let cleared = instr.clear_operand(Operand::core_rd).unwrap();
        // rd field zeroed, rest unchanged
        assert_eq!(cleared.word(), 0x012A0020);
    }

    #[test]
    fn test_clear_operand_multiple_fields() {
        // ori $t0, $t1, 0x1234
        let instr = Instruction::new(
            0x35281234,
            Vram::new(0x80000000),
            InstructionFlags::new(IsaVersion::MIPS_I),
        );
        // Clear the immediate field
        let cleared_imm = instr.clear_operand(Operand::core_imm_u16).unwrap();
        assert_eq!(cleared_imm.word(), 0x35280000);

        // Clear the rt field
        let cleared_rt = instr.clear_operand(Operand::core_rt).unwrap();
        assert_eq!(cleared_rt.word(), 0x35201234);

        // Clear the rs field
        let cleared_rs = instr.clear_operand(Operand::core_rs).unwrap();
        assert_eq!(cleared_rs.word(), 0x34081234);

        assert_eq!(
            instr.word(),
            0x35281234,
            "original instruction should remain unchanged"
        );

        let new_instr = instr
            .clear_operand(Operand::core_imm_u16)
            .and_then(|x| x.clear_operand(Operand::core_rt))
            .and_then(|x| x.clear_operand(Operand::core_rs));
        assert_eq!(
            new_instr.map(|x| x.word()),
            Some(0x34000000),
            "should clear all three fields"
        );
    }
}
