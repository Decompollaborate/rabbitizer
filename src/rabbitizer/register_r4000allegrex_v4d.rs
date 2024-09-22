/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{traits::Register, RegisterDescriptor , };
use crate::registers::{RegisterR4000AllegrexV4D, R4000ALLEGREX_V4D_REGISTERS};

impl RegisterR4000AllegrexV4D {
    #[must_use]
    pub const fn default() -> Self {
        Self::C000
    }
}

impl Register for RegisterR4000AllegrexV4D {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R4000ALLEGREX_V4D_REGISTERS[*self]
    }
}

impl Default for RegisterR4000AllegrexV4D {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR4000AllegrexV4D> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR4000AllegrexV4D) -> &Self::Output {
        &self[index as usize]
    }
}
