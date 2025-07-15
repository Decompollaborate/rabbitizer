/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexPrefixDst;
use crate::registers_meta::Register;

impl R4000AllegrexPrefixDst {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::none
    }
}

impl Register for R4000AllegrexPrefixDst {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        Self::count()
    }

    fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        Self::try_from_u32(value)
    }

    fn descriptor_array() -> &'static [RegisterDescriptor] {
        &register_descriptors::R4000ALLEGREX_PREFIX_DST
    }
}
