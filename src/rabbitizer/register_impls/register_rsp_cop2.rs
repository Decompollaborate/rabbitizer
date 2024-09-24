/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::RspCop2;
use crate::traits::Register;
use crate::RegisterDescriptor;

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

impl Default for RspCop2 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RspCop2> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RspCop2) -> &Self::Output {
        &self[index as usize]
    }
}
