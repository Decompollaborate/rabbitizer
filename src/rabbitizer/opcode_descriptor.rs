/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::InstrSuffix;
#[allow(deprecated)]
use crate::{operand::OPERAND_COUNT_MAX, InstrType, Opcode, Operand};

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
}

impl<'a> OpcodeDescriptor<'a> {
    pub const fn new(name: &'a str, operands: [Operand; OPERAND_COUNT_MAX]) -> Self {
        Self {
            name,
            operands,
            #[allow(deprecated)]
            instr_type: InstrType::default(),
            instr_suffix: InstrSuffix::default(),
            is_branch: false,
            is_branch_likely: false,
            is_jump: false,
            is_jump_with_address: false,
        }
    }

    pub const fn check_panic(&self) {
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
