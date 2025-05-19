/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::Cop0;
use crate::registers_meta::Register;

impl Cop0 {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::Index
    }
}

impl Register for Cop0 {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::COP0[*self]
    }
}
