/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexVConstant;
use crate::registers_meta::Register;

impl R4000AllegrexVConstant {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::INVALID_0
    }
}

impl Register for R4000AllegrexVConstant {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_VCONSTANT[*self]
    }
}
