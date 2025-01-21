/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::access_type::AccessType;
use crate::encoded_field_mask::EncodedFieldMask;
#[allow(deprecated)]
use crate::instr::{InstrSuffix, InstrType};
use crate::isa::{IsaExtension, IsaVersion};
use crate::opcodes::Opcode;
use crate::operands::{Operand, OperandIterator, OPERAND_COUNT_MAX};
use crate::utils;

/// Describes properties of a given [`Opcode`].
///
/// [`Opcode`]: crate::opcodes::Opcode
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct OpcodeDescriptor {
    pub(crate) name: &'static str,

    pub(crate) isa_version: IsaVersion,
    pub(crate) isa_extension: Option<IsaExtension>,

    pub(crate) operands: [Operand; OPERAND_COUNT_MAX],

    #[allow(deprecated)]
    pub(crate) instr_type: InstrType,

    pub(crate) instr_suffix: Option<InstrSuffix>,

    /// Local branch which has a "restricted" range, usually it doesn't jump outside the current function
    pub(crate) is_branch: bool,
    pub(crate) is_branch_likely: bool,

    /// The instruction can jump inside or outside its current function
    pub(crate) is_jump: bool,

    /// The target address of this jump is encoded in the instruction (MIPS: j and jal)
    pub(crate) is_jump_with_address: bool,

    /// May trigger a trap on the processor
    pub(crate) is_trap: bool,

    /// The instruction performs float (any kind of float, including double precision) operations
    pub(crate) is_float: bool,
    /// The instruction performs double precision float operations
    pub(crate) is_double: bool,

    /// The instruction holds an immediate which is treated as an unsigned value,
    /// otherwise the immediate it may hold should be treated as a Two's complement value
    pub(crate) is_unsigned: bool,

    /// The instruction modifies the state of the MIPS `rs` register
    pub(crate) modifies_rs: bool,
    /// The instruction modifies the state of the MIPS `rt` register
    pub(crate) modifies_rt: bool,
    /// The instruction modifies the state of the MIPS `rd` register
    pub(crate) modifies_rd: bool,

    /// The instruction reads the value which the MIPS `rs` register holds
    pub(crate) reads_rs: bool,
    /// The instruction reads the value which the MIPS `rt` register holds
    pub(crate) reads_rt: bool,
    /// The instruction reads the value which the MIPS `rd` register holds
    pub(crate) reads_rd: bool,

    pub(crate) reads_hi: bool,
    pub(crate) reads_lo: bool,
    pub(crate) modifies_hi: bool,
    pub(crate) modifies_lo: bool,

    pub(crate) modifies_fs: bool,
    pub(crate) modifies_ft: bool,
    pub(crate) modifies_fd: bool,
    pub(crate) reads_fs: bool,
    pub(crate) reads_ft: bool,
    pub(crate) reads_fd: bool,

    /// This instruction is not emited by a C compiler
    pub(crate) not_emitted_by_compilers: bool,

    /// The instruction can hold the "hi" value of a %hi/%lo pair
    pub(crate) can_be_hi: bool,
    /// The instruction can hold the "lo" value of a %hi/%lo pair
    pub(crate) can_be_lo: bool,
    /// The instruction can used with a "hi" instruction to load a big constant.
    pub(crate) can_be_unsigned_lo: bool,

    /// "and link" family of instructions
    ///
    /// The instruction stores the return address link in the MIPS $ra (GPR 31) register
    pub(crate) does_link: bool,

    /// This instruction performs a pointer dereference, either by loading from RAM or storing into RAM
    pub(crate) does_dereference: bool,
    /// Dereferences a pointer and loads data from RAM
    pub(crate) does_load: bool,
    /// Dereferences a pointer and stores data to RAM
    pub(crate) does_store: bool,

    /// This instruction may be the result of expanding the `move` pseudo-instruction
    pub(crate) maybe_is_move: bool,

    /// This instruction is a pseudo-instruction
    pub(crate) is_pseudo: bool,

    pub(crate) access_type: Option<AccessType>,
    pub(crate) does_unsigned_memory_access: bool,
}

impl OpcodeDescriptor {
    pub(crate) const fn default() -> Self {
        Self {
            name: "",
            isa_version: IsaVersion::default(),
            isa_extension: None,
            operands: Operand::arr0(),
            #[allow(deprecated)]
            instr_type: InstrType::default(),
            instr_suffix: None,
            is_branch: false,
            is_branch_likely: false,
            is_jump: false,
            is_jump_with_address: false,
            is_trap: false,
            is_float: false,
            is_double: false,
            is_unsigned: false,
            modifies_rs: false,
            modifies_rt: false,
            modifies_rd: false,
            reads_rs: false,
            reads_rt: false,
            reads_rd: false,
            reads_hi: false,
            reads_lo: false,
            modifies_hi: false,
            modifies_lo: false,
            modifies_fs: false,
            modifies_ft: false,
            modifies_fd: false,
            reads_fs: false,
            reads_ft: false,
            reads_fd: false,
            not_emitted_by_compilers: false,
            can_be_hi: false,
            can_be_lo: false,
            can_be_unsigned_lo: false,
            does_link: false,
            does_dereference: false,
            does_load: false,
            does_store: false,
            maybe_is_move: false,
            is_pseudo: false,
            access_type: None,
            does_unsigned_memory_access: false,
        }
    }

    pub(crate) const fn new(
        name: &'static str,
        isa_version: IsaVersion,
        isa_extension: Option<IsaExtension>,
    ) -> Self {
        Self {
            name,
            isa_version,
            isa_extension,

            ..Self::default()
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub(crate) const fn check_panic(&self) {
        assert!(
            !self.name.is_empty(),
            "An opcode must not have an empty name"
        );

        if self.isa_version as u32 == IsaVersion::EXTENSION as u32 {
            assert!(
                self.isa_extension.is_some(),
                "Opcode was marked as isa EXTENSION, but the extension type is None"
            );
        } else {
            assert!(
                self.isa_extension.is_none(),
                "Opcode was not marked as isa EXTENSION, but it has a non-None extension type"
            );
        }

        assert!(
            utils::truth_a_implies_b(self.is_branch_likely, self.is_branch),
            "An 'is_branch_likely' opcode must be `is_branch` too"
        );

        assert!(
            utils::truth_a_implies_b(self.is_jump_with_address, self.is_jump),
            "An 'is_jump_with_address' opcode must be `is_jump` too"
        );

        // Opcode must be at max either branch, jump or trap.
        assert!(
            !(self.is_branch && self.is_jump),
            "An opcode must be either branch or jump, not both"
        );
        assert!(
            !(self.is_branch && self.is_trap),
            "An opcode must be either branch or trap, not both"
        );
        assert!(
            !(self.is_jump && self.is_trap),
            "An opcode must be either jump or trap, not both"
        );

        assert!(
            utils::truth_a_implies_b(self.is_double, self.is_float),
            "An 'is_double' opcode must be `is_float` too"
        );

        // modifies_r* and reads_r* (gpr)
        assert!(
            !(self.modifies_rs && self.reads_rs),
            "An opcode must either modify or read the `rs` gpr register, not both"
        );
        assert!(
            !(self.modifies_rt && self.reads_rt),
            "An opcode must either modify or read the `rt` gpr register, not both"
        );
        assert!(
            !(self.modifies_rd && self.reads_rd),
            "An opcode must either modify or read the `rd` gpr register, not both"
        );
        assert!(
            utils::truth_both_or_neither(
                self.modifies_rs || self.reads_rs,
                self.has_operand_alias(Operand::core_rs)
            ),
            "An opcode that touches an `rs` gpr register must have an `rs` operand and viceversa"
        );
        assert!(
            utils::truth_both_or_neither(
                self.modifies_rt || self.reads_rt,
                self.has_operand_alias(Operand::core_rt)
            ),
            "An opcode that touches an `rt` gpr register must have an `rt` operand and viceversa"
        );
        assert!(
            utils::truth_both_or_neither(
                self.modifies_rd || self.reads_rd,
                self.has_operand_alias(Operand::core_rd)
            ),
            "An opcode that touches an `rd` gpr register must have an `rd` operand and viceversa"
        );

        // modifies_f* and reads_f* (fpr)
        assert!(
            !(self.modifies_fs && self.reads_fs),
            "An opcode must either modify or read the `fs` gpr register, not both"
        );
        assert!(
            !(self.modifies_ft && self.reads_ft),
            "An opcode must either modify or read the `ft` gpr register, not both"
        );
        assert!(
            !(self.modifies_fd && self.reads_fd),
            "An opcode must either modify or read the `fd` gpr register, not both"
        );
        assert!(
            utils::truth_both_or_neither(
                self.modifies_fs || self.reads_fs,
                self.has_operand_alias(Operand::core_fs)
            ),
            "An opcode that touches an `fs` gpr register must have an `fs` operand and viceversa"
        );
        assert!(
            utils::truth_both_or_neither(
                self.modifies_ft || self.reads_ft,
                self.has_operand_alias(Operand::core_ft)
            ),
            "An opcode that touches an `ft` gpr register must have an `ft` operand and viceversa"
        );
        assert!(
            utils::truth_both_or_neither(
                self.modifies_fd || self.reads_fd,
                self.has_operand_alias(Operand::core_fd)
            ),
            "An opcode that touches an `fd` gpr register must have an `fd` operand and viceversa"
        );

        assert!(
            !(self.can_be_hi && self.can_be_lo),
            "An opcode can be either a `hi` or `lo`, not both"
        );
        assert!(
            !(self.can_be_hi && self.can_be_unsigned_lo),
            "An opcode can be either a `hi` or `lo`, not both"
        );
        assert!(
            !(self.can_be_unsigned_lo && self.can_be_lo),
            "An opcode can be either an unsigned `lo` or a normal one, not both"
        );

        assert!(
            utils::truth_a_implies_b(self.can_be_unsigned_lo, self.is_unsigned),
            "An unsigned `lo` opcode must be unsigned"
        );
        assert!(
            utils::truth_a_implies_b(self.can_be_lo, !self.is_unsigned),
            "An `lo` opcode must be signed"
        );

        assert!(
            utils::truth_a_implies_b(self.does_link, self.is_branch || self.is_jump),
            "A 'does_link' opcode must be either `is_branch` or `is_jump`"
        );

        // dereference stuff
        assert!(
            utils::truth_a_implies_b(self.does_dereference, self.does_load || self.does_store),
            "Dereference must imply either reading from RAM or storing to it"
        );
        assert!(
            utils::truth_a_implies_b(self.does_load, self.does_dereference),
            "Reading from RAM must imply a dereference"
        );
        assert!(
            utils::truth_a_implies_b(self.does_store, self.does_dereference),
            "Storing to RAM must imply a dereference"
        );
        assert!(
            !(self.does_load && self.does_store),
            "Either load or store, not both"
        );
        assert!(
            utils::truth_both_or_neither(self.does_dereference, self.access_type.is_some()),
            "An opcode that does dereference memory should have a non NONE AccessType"
        );
        assert!(
            utils::truth_a_implies_b(self.does_unsigned_memory_access, self.does_dereference),
            "Unsigned memory accesses require dereferences"
        );

        // TODO: remove `&& !self.is_pseudo` check when the move pseudo is removed.
        assert!(
            utils::truth_a_implies_b(
                self.maybe_is_move && !self.is_pseudo,
                self.has_operand_alias(Operand::core_rd)
                    && self.has_operand_alias(Operand::core_rs)
                    && self.has_operand_alias(Operand::core_rt)
            ),
            "A `maybe_is_move` must use all three gpr register operands"
        );
        // assert!(utils::truth_a_implies_b(self.maybe_is_move, !self.is_pseudo), "`move` pseudo is too problematic");
    }

    pub(crate) const fn check_panic_chain(self) -> Self {
        self.check_panic();
        self
    }
}

/// Getters
impl OpcodeDescriptor {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn isa_version(&self) -> IsaVersion {
        self.isa_version
    }
    #[must_use]
    pub const fn isa_extension(&self) -> Option<IsaExtension> {
        self.isa_extension
    }

    #[must_use]
    pub const fn operands(&self) -> &[Operand; OPERAND_COUNT_MAX] {
        &self.operands
    }
    #[must_use]
    pub const fn operands_iter(&self) -> OperandIterator {
        OperandIterator::new(&self.operands)
    }

    #[must_use]
    pub const fn instr_type(&self) -> InstrType {
        self.instr_type
    }
    // #[must_use]
    // pub const fn instr_suffix(&self) -> InstrSuffix {
    //     self.instr_suffix
    // }
    #[must_use]
    pub const fn is_branch(&self) -> bool {
        self.is_branch
    }
    #[must_use]
    pub const fn is_branch_likely(&self) -> bool {
        self.is_branch_likely
    }
    #[must_use]
    pub const fn is_jump(&self) -> bool {
        self.is_jump
    }
    #[must_use]
    pub const fn is_jump_with_address(&self) -> bool {
        self.is_jump_with_address
    }
    #[must_use]
    pub const fn is_trap(&self) -> bool {
        self.is_trap
    }
    #[must_use]
    pub const fn is_float(&self) -> bool {
        self.is_float
    }
    #[must_use]
    pub const fn is_double(&self) -> bool {
        self.is_double
    }
    #[must_use]
    pub const fn is_unsigned(&self) -> bool {
        self.is_unsigned
    }
    #[must_use]
    pub const fn modifies_rs(&self) -> bool {
        self.modifies_rs
    }
    #[must_use]
    pub const fn modifies_rt(&self) -> bool {
        self.modifies_rt
    }
    #[must_use]
    pub const fn modifies_rd(&self) -> bool {
        self.modifies_rd
    }
    #[must_use]
    pub const fn reads_rs(&self) -> bool {
        self.reads_rs
    }
    #[must_use]
    pub const fn reads_rt(&self) -> bool {
        self.reads_rt
    }
    #[must_use]
    pub const fn reads_rd(&self) -> bool {
        self.reads_rd
    }
    #[must_use]
    pub const fn reads_hi(&self) -> bool {
        self.reads_hi
    }
    #[must_use]
    pub const fn reads_lo(&self) -> bool {
        self.reads_lo
    }
    #[must_use]
    pub const fn modifies_hi(&self) -> bool {
        self.modifies_hi
    }
    #[must_use]
    pub const fn modifies_lo(&self) -> bool {
        self.modifies_lo
    }
    #[must_use]
    pub const fn modifies_fs(&self) -> bool {
        self.modifies_fs
    }
    #[must_use]
    pub const fn modifies_ft(&self) -> bool {
        self.modifies_ft
    }
    #[must_use]
    pub const fn modifies_fd(&self) -> bool {
        self.modifies_fd
    }
    #[must_use]
    pub const fn reads_fs(&self) -> bool {
        self.reads_fs
    }
    #[must_use]
    pub const fn reads_ft(&self) -> bool {
        self.reads_ft
    }
    #[must_use]
    pub const fn reads_fd(&self) -> bool {
        self.reads_fd
    }
    #[must_use]
    pub const fn not_emitted_by_compilers(&self) -> bool {
        self.not_emitted_by_compilers
    }
    #[must_use]
    pub const fn can_be_hi(&self) -> bool {
        self.can_be_hi
    }
    #[must_use]
    pub const fn can_be_lo(&self) -> bool {
        self.can_be_lo
    }
    #[must_use]
    pub const fn can_be_unsigned_lo(&self) -> bool {
        self.can_be_unsigned_lo
    }
    #[must_use]
    pub const fn does_link(&self) -> bool {
        self.does_link
    }
    #[must_use]
    pub const fn does_dereference(&self) -> bool {
        self.does_dereference
    }
    #[must_use]
    pub const fn does_load(&self) -> bool {
        self.does_load
    }
    #[must_use]
    pub const fn does_store(&self) -> bool {
        self.does_store
    }
    #[must_use]
    pub const fn maybe_is_move(&self) -> bool {
        self.maybe_is_move
    }
    #[must_use]
    pub const fn is_pseudo(&self) -> bool {
        self.is_pseudo
    }
    #[must_use]
    pub const fn access_type(&self) -> Option<AccessType> {
        self.access_type
    }
    #[must_use]
    pub const fn does_unsigned_memory_access(&self) -> bool {
        self.does_unsigned_memory_access
    }
}

impl OpcodeDescriptor {
    #[must_use]
    pub fn valid_bits(&self) -> EncodedFieldMask {
        let mut bits = EncodedFieldMask::empty();

        for x in self.operands_iter() {
            bits.insert(x.get_descriptor().mask());
        }

        bits
    }

    #[must_use]
    pub const fn has_delay_slot(&self) -> bool {
        self.is_branch() || self.is_jump()
    }

    #[must_use]
    pub const fn has_any_operands(&self) -> bool {
        self.operands[0] as usize != Operand::ALL_EMPTY as usize
    }

    #[must_use]
    pub const fn has_specific_operand(&self, operand: Operand) -> bool {
        let mut i = 0;

        while i < self.operands.len() {
            let op = self.operands[i] as usize;

            if op == Operand::ALL_EMPTY as usize {
                break;
            }

            if self.operands[i] as usize == operand as usize {
                return true;
            }
            i += 1;
        }
        false
    }

    #[allow(clippy::cognitive_complexity)]
    #[must_use]
    pub const fn has_operand_alias(&self, operand: Operand) -> bool {
        if self.has_specific_operand(operand) {
            return true;
        }

        match operand {
            Operand::core_rs => {
                if self.has_specific_operand(Operand::core_immediate_base) {
                    return true;
                }
                if self.has_specific_operand(Operand::rsp_offset_rs) {
                    return true;
                }
                if self.has_specific_operand(Operand::core_maybe_rd_rs) {
                    return true;
                }
                if self.has_specific_operand(Operand::core_maybe_zero_rs) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_offset14_base) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_offset14_base_maybe_wb) {
                    return true;
                }
            }

            Operand::core_immediate => {
                if self.has_specific_operand(Operand::core_immediate_base) {
                    return true;
                }
            }

            Operand::core_rt => {}

            Operand::core_rd => {
                if self.has_specific_operand(Operand::core_maybe_rd_rs) {
                    return true;
                }
            }

            Operand::core_sa => {}
            Operand::core_zero => {}
            // Operand::core_function => {},
            Operand::core_cop0d => {
                if self.has_specific_operand(Operand::rsp_cop0d) {
                    return true;
                }
            }
            Operand::core_cop0cd => {}

            Operand::core_fs => {}
            Operand::core_ft => {}
            Operand::core_fd => {}
            Operand::core_cop1cs => {}
            Operand::core_cop2t => {}
            Operand::core_cop2d => {}
            Operand::core_cop2cd => {}
            Operand::core_op => {}
            Operand::core_hint => {}

            Operand::core_code => {
                if self.has_specific_operand(Operand::core_code_lower) {
                    return true;
                }
            }

            Operand::core_code_lower => {
                if self.has_operand_alias(Operand::core_code) {
                    return true;
                }
            }

            Operand::core_copraw => {}
            Operand::core_label => {}

            Operand::core_branch_target_label => {}

            Operand::core_immediate_base => {
                if self.has_operand_alias(Operand::core_rs) {
                    return true;
                }
                if self.has_operand_alias(Operand::core_immediate) {
                    return true;
                }
            }

            Operand::core_maybe_rd_rs => {
                if self.has_operand_alias(Operand::core_rd) {
                    return true;
                }
                if self.has_operand_alias(Operand::core_rs) {
                    return true;
                }
            }

            Operand::core_maybe_zero_rs => {
                if self.has_operand_alias(Operand::core_rs) {
                    return true;
                }
            }

            /* rsp */
            Operand::rsp_cop0d => {
                if self.has_specific_operand(Operand::core_cop0d) {
                    return true;
                }
            }

            Operand::rsp_cop2t => {}
            Operand::rsp_cop2cd => {}

            Operand::rsp_vs => {
                if self.has_specific_operand(Operand::rsp_vs_index) {
                    return true;
                }
            }

            Operand::rsp_vd => {
                if self.has_specific_operand(Operand::rsp_vd_de) {
                    return true;
                }
            }

            Operand::rsp_vt_elementhigh => {}
            Operand::rsp_vt_elementlow => {}

            Operand::rsp_vd_de => {
                if self.has_operand_alias(Operand::rsp_vd) {
                    return true;
                }
            }

            Operand::rsp_vs_index => {
                if self.has_operand_alias(Operand::rsp_vs) {
                    return true;
                }
            }

            Operand::rsp_offset_rs => {
                if self.has_operand_alias(Operand::core_rs) {
                    return true;
                }
            }
            /* rsp */

            /* r3000gte */
            Operand::r3000gte_sf => {}
            Operand::r3000gte_mx => {}
            Operand::r3000gte_v => {}
            Operand::r3000gte_cv => {}
            Operand::r3000gte_lm => {}
            /* r3000gte */

            /* r4000allegrex */
            Operand::r4000allegrex_s_vs => {}
            Operand::r4000allegrex_s_vt => {}
            Operand::r4000allegrex_s_vd => {}
            Operand::r4000allegrex_s_vt_imm => {}
            Operand::r4000allegrex_s_vd_imm => {}
            Operand::r4000allegrex_p_vs => {}
            Operand::r4000allegrex_p_vt => {}
            Operand::r4000allegrex_p_vd => {}
            Operand::r4000allegrex_t_vs => {}
            Operand::r4000allegrex_t_vt => {}
            Operand::r4000allegrex_t_vd => {}
            Operand::r4000allegrex_q_vs => {}
            Operand::r4000allegrex_q_vt => {}
            Operand::r4000allegrex_q_vd => {}
            Operand::r4000allegrex_q_vt_imm => {}
            Operand::r4000allegrex_mp_vs => {}
            Operand::r4000allegrex_mp_vt => {}
            Operand::r4000allegrex_mp_vd => {}
            Operand::r4000allegrex_mp_vs_transpose => {}
            Operand::r4000allegrex_mt_vs => {}
            Operand::r4000allegrex_mt_vt => {}
            Operand::r4000allegrex_mt_vd => {}
            Operand::r4000allegrex_mt_vs_transpose => {}
            Operand::r4000allegrex_mq_vs => {}
            Operand::r4000allegrex_mq_vt => {}
            Operand::r4000allegrex_mq_vd => {}
            Operand::r4000allegrex_mq_vs_transpose => {}
            Operand::r4000allegrex_cop2cs => {}
            Operand::r4000allegrex_cop2cd => {}

            Operand::r4000allegrex_pos => {}
            Operand::r4000allegrex_size => {}
            Operand::r4000allegrex_size_plus_pos => {}
            Operand::r4000allegrex_imm3 => {}

            Operand::r4000allegrex_offset14_base => {
                if self.has_operand_alias(Operand::core_rs) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_offset14_base_maybe_wb) {
                    return true;
                }
            }

            Operand::r4000allegrex_offset14_base_maybe_wb => {
                if self.has_operand_alias(Operand::core_rs) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_offset14_base) {
                    return true;
                }
            }

            Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => {
                if self.has_specific_operand(Operand::r4000allegrex_s_vs) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_s_vt) {
                    return true;
                }
            }

            Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => {
                if self.has_specific_operand(Operand::r4000allegrex_p_vs) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_p_vt) {
                    return true;
                }
            }

            Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => {
                if self.has_specific_operand(Operand::r4000allegrex_t_vs) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_t_vt) {
                    return true;
                }
            }

            Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => {
                if self.has_specific_operand(Operand::r4000allegrex_q_vs) {
                    return true;
                }
                if self.has_specific_operand(Operand::r4000allegrex_q_vt) {
                    return true;
                }
            }

            Operand::r4000allegrex_vconstant => {}
            Operand::r4000allegrex_power_of_two => {}
            Operand::r4000allegrex_vfpu_cc_bit => {}
            Operand::r4000allegrex_bn => {}
            Operand::r4000allegrex_int16 => {}
            Operand::r4000allegrex_float16 => {}
            Operand::r4000allegrex_p_vrot_code => {}
            Operand::r4000allegrex_t_vrot_code => {}
            Operand::r4000allegrex_q_vrot_code => {}
            Operand::r4000allegrex_wpx => {}
            Operand::r4000allegrex_wpy => {}
            Operand::r4000allegrex_wpz => {}
            Operand::r4000allegrex_wpw => {}
            Operand::r4000allegrex_rpx => {}
            Operand::r4000allegrex_rpy => {}
            Operand::r4000allegrex_rpz => {}
            Operand::r4000allegrex_rpw => {}
            /* r4000allegrex */

            /* r5900 */
            Operand::r5900_I => {}
            Operand::r5900_Q => {}
            Operand::r5900_R => {}

            Operand::r5900_ACC => {
                if self.has_specific_operand(Operand::r5900_ACCxyzw) {
                    return true;
                }
            }

            Operand::r5900_immediate5 => {}
            Operand::r5900_immediate15 => {}

            Operand::r5900_vfs => {
                if self.has_specific_operand(Operand::r5900_vfsxyzw) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vfsl) {
                    return true;
                }
            }

            Operand::r5900_vft => {
                if self.has_specific_operand(Operand::r5900_vftxyzw) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vftn) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vftm) {
                    return true;
                }
            }

            Operand::r5900_vfd => {
                if self.has_specific_operand(Operand::r5900_vfdxyzw) {
                    return true;
                }
            }

            Operand::r5900_vis => {
                if self.has_specific_operand(Operand::r5900_vis_predecr) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vis_postincr) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vis_parenthesis) {
                    return true;
                }
            }

            Operand::r5900_vit => {
                if self.has_specific_operand(Operand::r5900_vit_predecr) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vit_postincr) {
                    return true;
                }
            }

            Operand::r5900_vid => {}

            Operand::r5900_ACCxyzw => {
                if self.has_specific_operand(Operand::r5900_ACC) {
                    return true;
                }
            }

            Operand::r5900_vfsxyzw => {
                if self.has_specific_operand(Operand::r5900_vfs) {
                    return true;
                }
            }

            Operand::r5900_vftxyzw => {
                if self.has_specific_operand(Operand::r5900_vft) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vftn) {
                    return true;
                }
            }

            Operand::r5900_vfdxyzw => {
                if self.has_specific_operand(Operand::r5900_vfd) {
                    return true;
                }
            }

            Operand::r5900_vftn => {
                if self.has_specific_operand(Operand::r5900_vft) {
                    return true;
                }
                if self.has_specific_operand(Operand::r5900_vftxyzw) {
                    return true;
                }
            }

            Operand::r5900_vfsl => {
                if self.has_operand_alias(Operand::r5900_vfs) {
                    return true;
                }
            }

            Operand::r5900_vftm => {
                if self.has_operand_alias(Operand::r5900_vft) {
                    return true;
                }
            }

            Operand::r5900_vis_predecr => {
                if self.has_operand_alias(Operand::r5900_vis) {
                    return true;
                }
            }

            Operand::r5900_vit_predecr => {
                if self.has_operand_alias(Operand::r5900_vit) {
                    return true;
                }
            }

            Operand::r5900_vis_postincr => {
                if self.has_operand_alias(Operand::r5900_vis) {
                    return true;
                }
            }

            Operand::r5900_vit_postincr => {
                if self.has_operand_alias(Operand::r5900_vit) {
                    return true;
                }
            }

            Operand::r5900_vis_parenthesis => {
                if self.has_operand_alias(Operand::r5900_vis) {
                    return true;
                }
            }

            /* r5900 */
            Operand::ALL_EMPTY => {}
        }

        false
    }
}

impl Index<Opcode> for [OpcodeDescriptor] {
    type Output = OpcodeDescriptor;

    fn index(&self, index: Opcode) -> &Self::Output {
        &self[index as usize]
    }
}
