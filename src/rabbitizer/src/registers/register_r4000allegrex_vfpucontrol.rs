/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::register_descriptors;
use crate::register_descriptors::RegisterDescriptor;
use crate::registers::R4000AllegrexVfpuControl;
use crate::traits::Register;

impl R4000AllegrexVfpuControl {
    /// Returns a default value.
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
