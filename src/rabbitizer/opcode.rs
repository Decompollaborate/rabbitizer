/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{Opcode, OpcodeDescriptor, OPCODES};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPCODE_COUNT: usize = 943;

impl Opcode {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static OpcodeDescriptor {
        &OPCODES[*self]
    }
}

impl Opcode {
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.get_descriptor().name()
    }
}

impl Opcode {
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_INVALID
    }

    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !matches!(*self, Self::ALL_INVALID)
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j() {
        assert!(OPCODES[Opcode::cpu_j].is_jump);
        assert!(Opcode::cpu_j.get_descriptor().is_jump);
        assert!(OPCODES[Opcode::cpu_j].is_jump_with_address);
        assert!(Opcode::cpu_j.get_descriptor().is_jump_with_address);
        assert!(!OPCODES[Opcode::cpu_j].is_branch);
        assert!(!Opcode::cpu_j.get_descriptor().is_branch);
    }
}
