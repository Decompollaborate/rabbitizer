/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::RspVector;
use crate::traits::Register;
use crate::RegisterDescriptor;

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

impl Default for RspVector {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RspVector> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RspVector) -> &Self::Output {
        &self[index as usize]
    }
}
