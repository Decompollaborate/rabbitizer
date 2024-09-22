/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::Cop0;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl Cop0 {
    #[must_use]
    pub const fn default() -> Self {
        Self::Index
    }
}

impl Register for Cop0 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::COP0[*self]
    }
}

impl Default for Cop0 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<Cop0> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: Cop0) -> &Self::Output {
        &self[index as usize]
    }
}
