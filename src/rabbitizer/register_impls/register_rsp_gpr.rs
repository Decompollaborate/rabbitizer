/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::registers::RspGpr;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl RspGpr {
    #[must_use]
    pub const fn default() -> Self {
        Self::zero
    }
}

impl Register for RspGpr {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::RSP_GPR[*self]
    }
}
