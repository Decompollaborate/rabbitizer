/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{traits::Register, RegisterDescriptor, 
};
use crate::registers::{RegisterR4000AllegrexVConstant, R4000ALLEGREX_VCONSTANT_REGISTERS};

impl RegisterR4000AllegrexVConstant {
    #[must_use]
    pub const fn default() -> Self {
        Self::INVALID_0
    }
}

impl Register for RegisterR4000AllegrexVConstant {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R4000ALLEGREX_VCONSTANT_REGISTERS[*self]
    }
}

impl Default for RegisterR4000AllegrexVConstant {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR4000AllegrexVConstant> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR4000AllegrexVConstant) -> &Self::Output {
        &self[index as usize]
    }
}
