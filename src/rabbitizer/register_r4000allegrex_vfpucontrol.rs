/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::R4000AllegrexVfpuControl;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R4000AllegrexVfpuControl {
    #[must_use]
    pub const fn default() -> Self {
        Self::VFPU_PFXS
    }
}

impl Register for R4000AllegrexVfpuControl {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R4000ALLEGREX_VFPUCONTROL[*self]
    }
}

impl Default for R4000AllegrexVfpuControl {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<R4000AllegrexVfpuControl> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: R4000AllegrexVfpuControl) -> &Self::Output {
        &self[index as usize]
    }
}
