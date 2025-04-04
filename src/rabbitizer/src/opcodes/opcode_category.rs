/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::opcodes::{OpcodeCategory, OpcodeCategoryDescriptor, OPCODE_CATEGORIES};

pub const OPCODE_CATEGORY_COUNT: usize = {
    let mut count = 13;

    if cfg!(feature = "MIPS_II") {
        count += 0;
    }
    if cfg!(feature = "MIPS_III") {
        count += 0;
    }
    if cfg!(feature = "MIPS_IV") {
        count += 0;
    }

    if cfg!(feature = "RSP") {
        count += 9;
    }
    if cfg!(feature = "R3000GTE") {
        count += 7;
    }
    if cfg!(feature = "R4000ALLEGREX") {
        count += 45;
    }
    if cfg!(feature = "R5900EE") {
        count += 21;
    }

    if cfg!(feature = "RspViceMsp") {
        count += 0;
    }

    count
};

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

    #[must_use]
    pub fn handwritten_category(&self) -> bool {
        self.get_descriptor().handwritten_category()
    }
}
