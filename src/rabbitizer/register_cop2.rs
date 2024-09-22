/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::Cop2;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl Cop2 {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop2 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::COP2[*self]
    }
}

impl Default for Cop2 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<Cop2> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: Cop2) -> &Self::Output {
        &self[index as usize]
    }
}
