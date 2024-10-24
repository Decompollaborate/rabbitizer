/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::isa::IsaVersion;

impl IsaVersion {
    #[must_use]
    pub const fn default() -> Self {
        Self::MIPS_I
    }
}

impl Default for IsaVersion {
    fn default() -> Self {
        Self::default()
    }
}
