/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R5900VI;
use crate::registers_meta::Register;

impl R5900VI {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::vi0
    }
}

impl Register for R5900VI {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R5900_VI[*self]
    }
}
