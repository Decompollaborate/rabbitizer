/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterCop2, RegisterDescriptor, COP2_REGISTERS};

impl RegisterCop2 {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for RegisterCop2 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP2_REGISTERS[*self]
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
