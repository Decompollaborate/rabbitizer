/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::Cop1;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl Cop1 {
    #[must_use]
    pub const fn default() -> Self {
        Self::fv0
    }
}

impl Register for Cop1 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::COP1[*self]
    }
}
