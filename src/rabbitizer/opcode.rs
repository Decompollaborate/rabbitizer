/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{Opcode, OpcodeDescriptor, OPCODES};

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub(crate) const OPCODE_COUNT: usize = 1052;

impl<'a> Opcode {
    pub fn get_descriptor(&self) -> &'a OpcodeDescriptor {
        &OPCODES[*self]
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Self::cpu_INVALID => false,
            Self::rsp_INVALID => false,
            Self::r3000gte_INVALID => false,
            Self::r4000allegrex_INVALID => false,
            Self::r5900_INVALID => false,
            Self::cpu_MAX => false,
            Self::rsp_MAX => false,
            Self::r3000gte_MAX => false,
            Self::r4000allegrex_MAX => false,
            Self::r5900_MAX => false,
            _ => true,
        }
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Self::cpu_INVALID
    }
}
