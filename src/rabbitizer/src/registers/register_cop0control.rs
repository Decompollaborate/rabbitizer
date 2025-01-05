/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors::RegisterDescriptor;
use crate::register_descriptors::COP0_CONTROL;
use crate::registers::Cop0Control;
use crate::registers_meta::Register;

impl Cop0Control {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop0Control {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &COP0_CONTROL[*self]
    }
}
