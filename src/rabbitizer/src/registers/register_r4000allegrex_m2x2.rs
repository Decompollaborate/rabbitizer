/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexM2x2;
use crate::traits::Register;

impl R4000AllegrexM2x2 {
    /// Returns a default value.
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
