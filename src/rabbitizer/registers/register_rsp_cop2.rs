/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::RspCop2;
use crate::traits::Register;

impl RspCop2 {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for RspCop2 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::RSP_COP2[*self]
    }
}
