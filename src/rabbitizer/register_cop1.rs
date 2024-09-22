/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterCop1, RegisterDescriptor, COP1_REGISTERS};

impl RegisterCop1 {
    #[must_use]
    pub const fn default() -> Self {
        Self::fv0
    }
}

impl Register for RegisterCop1 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP1_REGISTERS[*self]
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
