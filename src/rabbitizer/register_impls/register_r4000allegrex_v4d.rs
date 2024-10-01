/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::R4000AllegrexV4D;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexV4D {
    #[must_use]
    pub const fn default() -> Self {
        Self::C000
    }
}

impl Register for R4000AllegrexV4D {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_V4D[*self]
    }
}