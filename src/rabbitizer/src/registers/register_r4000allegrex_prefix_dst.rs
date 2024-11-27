/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexPrefixDst;
use crate::traits::Register;

impl R4000AllegrexPrefixDst {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::none
    }
}

impl Register for R4000AllegrexPrefixDst {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_PREFIX_DST[*self]
    }
}
