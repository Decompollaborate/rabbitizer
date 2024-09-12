/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

#[allow(deprecated)]
use crate::{operand::OPERAND_COUNT_MAX, InstrType, Opcode, Operand};
use crate::{AccessType, InstrSuffix};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
pub struct OpcodeDescriptor<'a> {
    pub(crate) name: &'a str,

    pub(crate) operands: [Operand; OPERAND_COUNT_MAX],

    #[allow(deprecated)]
    pub(crate) instr_type: InstrType,

    pub(crate) instr_suffix: InstrSuffix,

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

    pub(crate) access_type: AccessType,
    pub(crate) does_unsigned_memory_access: bool,
}

impl<'a> OpcodeDescriptor<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            operands: Operand::arr0(),
            #[allow(deprecated)]
            instr_type: InstrType::default(),
            instr_suffix: InstrSuffix::default(),
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
            does_link: false,
            does_dereference: false,
            does_load: false,
            does_store: false,
            maybe_is_move: false,
            is_pseudo: false,
            access_type: AccessType::default(),
            does_unsigned_memory_access: false,
        }
    }

    pub const fn check_panic(&self) {
        // TODO: the rest of checks

        assert!(
            self.name.len() != 0,
            "An opcode should not have an empty name"
        );
        assert!(
            !(self.is_branch && self.is_jump),
            "An opcode should be either branch or jump, not both"
        );
    }

    pub(crate) const fn check_panic_chain(self) -> Self {
        self.check_panic();
        self
    }
}

impl<'a> OpcodeDescriptor<'a> {
    // getters and setters
}

impl Index<Opcode> for [OpcodeDescriptor<'static>] {
    type Output = OpcodeDescriptor<'static>;

    fn index(&self, index: Opcode) -> &Self::Output {
        &self[index as usize]
    }
}