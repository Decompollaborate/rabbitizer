/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use std::ops::Index;


#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Opcode {
    J = 0,
}

impl Opcode {
    pub fn get_descriptor(&self) -> &'static OpcodeDescriptor {
        &OPCODES[*self]
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Default)]
pub struct OpcodeDescriptor<'a> {
    pub name: &'a str,

    /**
     * Local branch with "restricted" range, usually it doesn't jump outside the current function
     */
    pub is_branch: bool,
    pub is_branch_likely: bool,

    /**
     * The instruction can jump inside or outside its current function
     */
    pub is_jump: bool,

    /**
     * The target address of this jump is encoded in the instruction (MIPS: J and JAL)
     */
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
}


pub struct OpcodeDescriptorBuilder<'a> {
    inner: OpcodeDescriptor<'a>,
}

impl<'a> OpcodeDescriptorBuilder<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            inner: OpcodeDescriptor::new(name),
        }
    }

    pub const fn build(self) -> OpcodeDescriptor<'a> {
        self.inner
    }

    pub const fn is_branch(mut self) -> Self {
        self.inner.is_branch = true;
        self
    }

    pub const fn is_branch_likely(mut self) -> Self {
        self.inner.is_branch_likely = true;
        self
    }

    pub const fn is_jump(mut self) -> Self {
        self.inner.is_jump = true;
        self
    }

    pub const fn is_jump_with_address(mut self) -> Self {
        self.inner.is_jump_with_address = true;
        self
    }
}

pub static OPCODE_J: OpcodeDescriptor = OpcodeDescriptorBuilder::new("J").is_jump().is_jump_with_address().build();

pub static OPCODES: &[&OpcodeDescriptor] = &[
    &OPCODE_J
];

impl Index<Opcode> for [&'static OpcodeDescriptor<'static>] {
    type Output = &'static OpcodeDescriptor<'static>;

    fn index(&self, index: Opcode) -> &Self::Output {
        &self[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j() {
        assert_eq!(OPCODES[0], &OPCODE_J);
        assert_eq!(OPCODES[Opcode::J], &OPCODE_J);
        assert_eq!(Opcode::J.get_descriptor(), &OPCODE_J);

        assert_eq!(*OPCODES[0], OPCODE_J);
        assert_eq!(*OPCODES[Opcode::J], OPCODE_J);
        assert_eq!(*Opcode::J.get_descriptor(), OPCODE_J);
    }
}
