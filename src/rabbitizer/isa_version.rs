/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::IsaVersion;

impl IsaVersion {
    pub const fn default() -> Self {
        Self::MIPS_I
    }
}

impl Default for IsaVersion {
    fn default() -> Self {
        Self::default()
    }
}
