/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterDescriptor, RegisterRspCop2Control, RSP_COP2_CONTROL_REGISTERS};

impl RegisterRspCop2Control {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for RegisterRspCop2Control {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_COP2_CONTROL_REGISTERS[*self]
    }
}

impl Default for RegisterRspCop2Control {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspCop2Control> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspCop2Control) -> &Self::Output {
        &self[index as usize]
    }
}
