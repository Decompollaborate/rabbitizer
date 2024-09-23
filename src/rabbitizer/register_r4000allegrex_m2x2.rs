/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::R4000AllegrexM2x2;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexM2x2 {
    #[must_use]
    pub const fn default() -> Self {
        Self::M000
    }
}

impl Register for R4000AllegrexM2x2 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_M2X2[*self]
    }
}

impl Default for R4000AllegrexM2x2 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<R4000AllegrexM2x2> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: R4000AllegrexM2x2) -> &Self::Output {
        &self[index as usize]
    }
}