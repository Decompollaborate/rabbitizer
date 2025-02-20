/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R5900EEVF;
use crate::registers_meta::Register;

impl R5900EEVF {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::vf0
    }
}

impl Register for R5900EEVF {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R5900EE_VF[*self]
    }
}
