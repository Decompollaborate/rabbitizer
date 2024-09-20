/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, RegisterCop2, RegisterDescriptor, COP2_REGISTERS};

impl RegisterCop2 {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }

    #[must_use]
    pub fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP2_REGISTERS[*self]
    }

    #[must_use]
    pub fn numeric_reg(&self) -> &'static str {
        self.get_descriptor().numeric_reg()
    }

    #[must_use]
    pub fn named_reg(&self, abi: Abi) -> &'static str {
        self.get_descriptor().named_reg(abi)
    }
}

impl Default for RegisterCop2 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterCop2> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterCop2) -> &Self::Output {
        &self[index as usize]
    }
}
