/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R5900EEVI;
use crate::registers_meta::Register;

impl R5900EEVI {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::vi0
    }
}

impl Register for R5900EEVI {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R5900EE_VI[*self]
    }
}
