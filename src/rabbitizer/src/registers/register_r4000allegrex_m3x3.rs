/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexM3x3;
use crate::registers_meta::Register;

impl R4000AllegrexM3x3 {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::M000
    }
}

impl Register for R4000AllegrexM3x3 {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_M3X3[*self]
    }
}
