/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexS;
use crate::registers_meta::IntRegisterConversionError;
use crate::registers_meta::{R4000AllegrexVectorRegister, Register};

impl R4000AllegrexS {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::S000
    }
}

impl Register for R4000AllegrexS {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        Self::count()
    }

    fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
        Self::try_from_u32(value)
    }

    fn descriptor_array() -> &'static [RegisterDescriptor] {
        &register_descriptors::R4000ALLEGREX_S
    }
}

impl R4000AllegrexVectorRegister for R4000AllegrexS {}
