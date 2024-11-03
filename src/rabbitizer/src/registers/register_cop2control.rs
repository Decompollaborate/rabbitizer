/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors::RegisterDescriptor;
use crate::register_descriptors::COP2_CONTROL;
use crate::registers::Cop2Control;
use crate::traits::Register;

impl Cop2Control {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop2Control {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP2_CONTROL[*self]
    }
}
