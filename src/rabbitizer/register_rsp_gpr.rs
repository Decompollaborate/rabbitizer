/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{traits::Register, RegisterDescriptor, };
use crate::registers::{RegisterRspGpr, RSP_GPR_REGISTERS};

impl RegisterRspGpr {
    #[must_use]
    pub const fn default() -> Self {
        Self::zero
    }
}

impl Register for RegisterRspGpr {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_GPR_REGISTERS[*self]
    }
}

impl Default for RegisterRspGpr {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspGpr> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspGpr) -> &Self::Output {
        &self[index as usize]
    }
}
