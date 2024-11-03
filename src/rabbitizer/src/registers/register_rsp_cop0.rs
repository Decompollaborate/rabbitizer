/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::RspCop0;
use crate::traits::Register;

impl RspCop0 {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::SP_MEM_ADDR
    }
}

impl Register for RspCop0 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::RSP_COP0[*self]
    }
}
