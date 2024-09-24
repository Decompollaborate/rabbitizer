/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

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

impl Default for RspGpr {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RspGpr> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RspGpr) -> &Self::Output {
        &self[index as usize]
    }
}
