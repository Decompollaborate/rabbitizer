/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexV2D;
use crate::traits::{R4000AllegrexVectorRegister, Register};

impl R4000AllegrexV2D {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::C000
    }
}

impl Register for R4000AllegrexV2D {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_V2D[*self]
    }
}

impl R4000AllegrexVectorRegister for R4000AllegrexV2D {}
