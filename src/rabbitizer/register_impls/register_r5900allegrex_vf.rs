/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

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
