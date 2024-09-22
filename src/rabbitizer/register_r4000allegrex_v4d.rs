/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::R4000AllegrexV4D;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexV4D {
    #[must_use]
    pub const fn default() -> Self {
        Self::C000
    }
}

impl Register for R4000AllegrexV4D {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_V4D[*self]
    }
}

impl Default for R4000AllegrexV4D {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<R4000AllegrexV4D> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: R4000AllegrexV4D) -> &Self::Output {
        &self[index as usize]
    }
}
