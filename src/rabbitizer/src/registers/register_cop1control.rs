/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::Cop1Control;
use crate::registers_meta::IntRegisterConversionError;
use crate::registers_meta::Register;

impl Cop1Control {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for Cop1Control {
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
        &register_descriptors::COP1_CONTROL
    }
}
