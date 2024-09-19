/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Abi, RegisterCop1, RegisterDescriptor, COP1_REGISTERS};

impl RegisterCop1 {
    #[must_use]
    pub const fn default() -> Self {
        Self::fv0
    }

    #[must_use]
    pub fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP1_REGISTERS[*self]
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

impl Default for RegisterCop1 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterCop1> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterCop1) -> &Self::Output {
        &self[index as usize]
    }
}
