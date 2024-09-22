/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterDescriptor, RegisterR4000AllegrexS, R4000ALLEGREX_S_REGISTERS};

impl RegisterR4000AllegrexS {
    #[must_use]
    pub const fn default() -> Self {
        Self::S000
    }
}

impl Register for RegisterR4000AllegrexS {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R4000ALLEGREX_S_REGISTERS[*self]
    }
}

impl Default for RegisterR4000AllegrexS {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR4000AllegrexS> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR4000AllegrexS) -> &Self::Output {
        &self[index as usize]
    }
}
