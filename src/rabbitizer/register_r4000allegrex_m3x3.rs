/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{traits::Register, RegisterDescriptor
};
use crate::registers::{RegisterR4000AllegrexM3x3, R4000ALLEGREX_M3X3_REGISTERS};

impl RegisterR4000AllegrexM3x3 {
    #[must_use]
    pub const fn default() -> Self {
        Self::M000
    }
}

impl Register for RegisterR4000AllegrexM3x3 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R4000ALLEGREX_M3X3_REGISTERS[*self]
    }
}

impl Default for RegisterR4000AllegrexM3x3 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR4000AllegrexM3x3> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR4000AllegrexM3x3) -> &Self::Output {
        &self[index as usize]
    }
}
