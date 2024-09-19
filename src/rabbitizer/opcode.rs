/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{Opcode, OpcodeDescriptor, OPCODES};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPCODE_COUNT: usize = 943;

impl<'a> Opcode {
    pub fn get_descriptor(&self) -> &'a OpcodeDescriptor {
        &OPCODES[*self]
    }
}

impl Opcode {
    pub const fn default() -> Self {
        Self::ALL_INVALID
    }

    pub const fn is_valid(&self) -> bool {
        match self {
            Self::ALL_INVALID => false,
            _ => true,
        }
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Self::default()
    }
}
