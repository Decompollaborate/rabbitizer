/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::R4000AllegrexS;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexS {
    #[must_use]
    pub const fn default() -> Self {
        Self::S000
    }
}

impl Register for R4000AllegrexS {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_S[*self]
    }
}

impl Default for R4000AllegrexS {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<R4000AllegrexS> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: R4000AllegrexS) -> &Self::Output {
        &self[index as usize]
    }
}
