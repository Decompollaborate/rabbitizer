/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterDescriptor, RegisterR5900VI, R5900_VI_REGISTERS};

impl RegisterR5900VI {
    #[must_use]
    pub const fn default() -> Self {
        Self::vi0
    }
}

impl Register for RegisterR5900VI {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R5900_VI_REGISTERS[*self]
    }
}

impl Default for RegisterR5900VI {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR5900VI> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR5900VI) -> &Self::Output {
        &self[index as usize]
    }
}
