/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexVConstant;
use crate::traits::Register;

impl R4000AllegrexVConstant {
    #[must_use]
    pub const fn default() -> Self {
        Self::INVALID_0
    }
}

impl Register for R4000AllegrexVConstant {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_VCONSTANT[*self]
    }
}
