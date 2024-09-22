/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::RspCop2Control;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl RspCop2Control {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for RspCop2Control {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::RSP_COP2_CONTROL[*self]
    }
}

impl Default for RspCop2Control {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RspCop2Control> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RspCop2Control) -> &Self::Output {
        &self[index as usize]
    }
}
