/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexPrefixSrc;
use crate::registers_meta::Register;

impl R4000AllegrexPrefixSrc {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::x
    }
}

impl Register for R4000AllegrexPrefixSrc {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_PREFIX_SRC[*self]
    }
}
