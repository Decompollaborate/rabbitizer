/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::Cop1Control;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl Cop1Control {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop1Control {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::COP1CONTROL[*self]
    }
}

impl Default for Cop1Control {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<Cop1Control> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: Cop1Control) -> &Self::Output {
        &self[index as usize]
    }
}
