/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::R4000AllegrexPrefixSrc;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexPrefixSrc {
    #[must_use]
    pub const fn default() -> Self {
        Self::x
    }
}

impl Register for R4000AllegrexPrefixSrc {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_PREFIX_SRC[*self]
    }
}