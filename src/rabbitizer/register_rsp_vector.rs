/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterDescriptor, RegisterRspVector, RSP_VECTOR_REGISTERS};

impl RegisterRspVector {
    #[must_use]
    pub const fn default() -> Self {
        Self::v0
    }
}

impl Register for RegisterRspVector {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &RSP_VECTOR_REGISTERS[*self]
    }
}

impl Default for RegisterRspVector {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterRspVector> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterRspVector) -> &Self::Output {
        &self[index as usize]
    }
}
