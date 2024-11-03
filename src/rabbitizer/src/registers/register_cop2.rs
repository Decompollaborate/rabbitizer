/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::Cop2;
use crate::traits::Register;

impl Cop2 {
    /// Returns a default value.
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
