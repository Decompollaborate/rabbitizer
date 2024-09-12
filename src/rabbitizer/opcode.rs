/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{Opcode, OpcodeDescriptor, OPCODES};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub const OPCODE_COUNT: usize = 1052;

impl Opcode {
    pub fn get_descriptor(&self) -> &'static OpcodeDescriptor {
        &OPCODES[*self]
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Self::cpu_INVALID
    }
}
