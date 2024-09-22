/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::{
    Register, RegisterDescriptor, RegisterR4000AllegrexVfpuControl,
    R4000ALLEGREX_VFPUCONTROL_REGISTERS,
};

impl RegisterR4000AllegrexVfpuControl {
    #[must_use]
    pub const fn default() -> Self {
        Self::VFPU_PFXS
    }
}

impl Register for RegisterR4000AllegrexVfpuControl {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &R4000ALLEGREX_VFPUCONTROL_REGISTERS[*self]
    }
}

impl Default for RegisterR4000AllegrexVfpuControl {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<RegisterR4000AllegrexVfpuControl> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: RegisterR4000AllegrexVfpuControl) -> &Self::Output {
        &self[index as usize]
    }
}
