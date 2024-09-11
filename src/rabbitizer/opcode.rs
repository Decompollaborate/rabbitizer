/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{opcodes, OpcodeDescriptor};


#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Opcode {
    J = 0,

    MAX,
}

impl Opcode {
    pub fn get_descriptor(&self) -> &'static OpcodeDescriptor {
        assert_ne!(*self, Opcode::MAX);

        &opcodes::OPCODES[*self]
    }
}
