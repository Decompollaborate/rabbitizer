/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{opcodes, OpcodeDescriptor};


#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    core_j = 0,
}

// Rust doesn't have a way to automatically get the larger value of an enum and
// I didn't want to have a `Opcode::MAX` value, so instead we manually maintain
// this constant.
pub const OPCODE_COUNT: usize = 1;

impl Opcode {
    pub fn get_descriptor(&self) -> &'static OpcodeDescriptor {
        &opcodes::OPCODES[*self]
    }
}
