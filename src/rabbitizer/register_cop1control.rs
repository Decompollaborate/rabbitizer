/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{traits::Register, RegisterDescriptor};
use crate::registers::{RegisterCop1Control, COP1CONTROL_REGISTERS};

impl RegisterCop1Control {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for RegisterCop1Control {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP1CONTROL_REGISTERS[*self]
    }
}

impl Default for RegisterCop1Control {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterCop1Control> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterCop1Control) -> &Self::Output {
        &self[index as usize]
    }
}
