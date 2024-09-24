/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

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
