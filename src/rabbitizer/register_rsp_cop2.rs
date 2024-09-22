/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{traits::Register, RegisterDescriptor,  };
use crate::registers::{RegisterRspCop2, RSP_COP2_REGISTERS};

impl RegisterRspCop2 {
    #[must_use]
    pub const fn default() -> Self {
        Self::r0
    }
}

impl Register for RegisterRspCop2 {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_COP2_REGISTERS[*self]
    }
}

impl Default for RegisterRspCop2 {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspCop2> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspCop2) -> &Self::Output {
        &self[index as usize]
    }
}
