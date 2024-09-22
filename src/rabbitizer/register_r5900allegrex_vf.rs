/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{Register, RegisterDescriptor, RegisterR5900VF, R5900_VF_REGISTERS};

impl RegisterR5900VF {
    #[must_use]
    pub const fn default() -> Self {
        Self::vf0
    }
}

impl Register for RegisterR5900VF {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R5900_VF_REGISTERS[*self]
    }
}

impl Default for RegisterR5900VF {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR5900VF> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR5900VF) -> &Self::Output {
        &self[index as usize]
    }
}
