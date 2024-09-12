/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
pub struct OpcodeDescriptor<'a> {
    pub name: &'a str,

    /// Local branch which has a "restricted" range, usually it doesn't jump outside the current function
    pub is_branch: bool,
    pub is_branch_likely: bool,

    /// The instruction can jump inside or outside its current function
    pub is_jump: bool,

    /// The target address of this jump is encoded in the instruction (MIPS: j and jal)
    pub is_jump_with_address: bool,
}

impl<'a> OpcodeDescriptor<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
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
