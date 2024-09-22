/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ops::Index;

use crate::register_descriptors;
use crate::registers::R5900VF;
use crate::traits::Register;
use crate::RegisterDescriptor;

impl R5900VF {
    #[must_use]
    pub const fn default() -> Self {
        Self::vf0
    }
}

impl Register for R5900VF {
    #[must_use]
    fn get_descriptor(&self) -> &'static RegisterDescriptor {
        &register_descriptors::R5900_VF[*self]
    }
}

impl Default for R5900VF {
    fn default() -> Self {
        Self::default()
    }
}

impl Index<R5900VF> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;

    fn index(&self, index: R5900VF) -> &Self::Output {
        &self[index as usize]
    }
}
