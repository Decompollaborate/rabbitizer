/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::RspVector;
use crate::traits::Register;

impl RspVector {
    #[must_use]
    pub const fn default() -> Self {
        Self::v0
    }
}

impl Register for RspVector {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::RSP_VECTOR[*self]
    }
}
