/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::RspCop2;
use crate::registers_meta::Register;

impl RspCop2 {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for RspCop2 {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::RSP_COP2[*self]
    }
}
