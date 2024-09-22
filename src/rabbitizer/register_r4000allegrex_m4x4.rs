/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::R4000AllegrexM4x4;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexM4x4 {
    #[must_use]
    pub const fn default() -> Self {
        Self::M000
    }
}

impl Register for R4000AllegrexM4x4 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_M4X4[*self]
    }
}

impl Default for R4000AllegrexM4x4 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<R4000AllegrexM4x4> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: R4000AllegrexM4x4) -> &Self::Output {
        &self[index as usize]
    }
}
