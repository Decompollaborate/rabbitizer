/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::opcodes::{OpcodeCategory, OpcodeCategoryDescriptor, OPCODE_CATEGORIES};

pub const OPCODE_CATEGORY_COUNT: usize = 100;

impl OpcodeCategory {
    #[must_use]
    pub fn get_descriptor(&self) -> &'static OpcodeCategoryDescriptor {
        &OPCODE_CATEGORIES[*self]
    }
}

impl OpcodeCategory {
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.get_descriptor().name()
    }
}
