/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::RspCop0;
use crate::registers_meta::Register;

impl RspCop0 {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::SP_MEM_ADDR
    }
}

impl Register for RspCop0 {
    #[must_use]
    fn as_index(&self) -> usize {
        *self as usize
    }

    #[must_use]
    fn count() -> usize {
        Self::count()
    }

    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::RSP_COP0[*self]
    }
}
