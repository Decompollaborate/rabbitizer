/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::R4000AllegrexVConstant;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexVConstant {
    #[must_use]
    pub const fn default() -> Self {
        Self::INVALID_0
    }
}

impl Register for R4000AllegrexVConstant {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_VCONSTANT[*self]
    }
}

impl Default for R4000AllegrexVConstant {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<R4000AllegrexVConstant> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: R4000AllegrexVConstant) -> &Self::Output {
        &self[index as usize]
    }
}
