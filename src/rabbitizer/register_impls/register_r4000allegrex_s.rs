/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::R4000AllegrexS;
use crate::traits::{R4000AllegrexVectorRegister, Register};
use crate::RegisterDescriptor;

impl R4000AllegrexS {
    #[must_use]
    pub const fn default() -> Self {
        Self::S000
    }
}

impl Register for R4000AllegrexS {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_S[*self]
    }
}

impl R4000AllegrexVectorRegister for R4000AllegrexS {}
