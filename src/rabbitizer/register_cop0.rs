/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterCop0, RegisterDescriptor, COP0_REGISTERS};

impl RegisterCop0 {
    #[must_use]
    pub const fn default() -> Self {
        Self::Index
    }
}

impl Register for RegisterCop0 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP0_REGISTERS[*self]
    }
}

impl Default for RegisterCop0 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterCop0> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterCop0) -> &Self::Output {
        &self[index as usize]
    }
}
