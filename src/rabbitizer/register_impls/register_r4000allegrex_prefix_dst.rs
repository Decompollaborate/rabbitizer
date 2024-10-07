/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::R4000AllegrexPrefixDst;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexPrefixDst {
    #[must_use]
    pub const fn default() -> Self {
        Self::none
    }
}

impl Register for R4000AllegrexPrefixDst {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_PREFIX_DST[*self]
    }
}
