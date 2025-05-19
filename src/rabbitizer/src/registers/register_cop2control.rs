/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors::RegisterDescriptor;
use crate::register_descriptors::COP2_CONTROL;
use crate::registers::Cop2Control;
use crate::registers_meta::Register;

impl Cop2Control {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop2Control {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP2_CONTROL[*self]
    }
}
