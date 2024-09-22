/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{
    Register, RegisterDescriptor, RegisterR4000AllegrexM4x4, R4000ALLEGREX_M4X4_REGISTERS,
};

impl RegisterR4000AllegrexM4x4 {
    #[must_use]
    pub const fn default() -> Self {
        Self::M000
    }
}

impl Register for RegisterR4000AllegrexM4x4 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R4000ALLEGREX_M4X4_REGISTERS[*self]
    }
}

impl Default for RegisterR4000AllegrexM4x4 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR4000AllegrexM4x4> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR4000AllegrexM4x4) -> &Self::Output {
        &self[index as usize]
    }
}
