/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::isa::IsaVersion;

impl IsaVersion {
    /// Returns a default value.
    ///
    /// Defaults to [`MIPS_III`].
    ///
    /// [`MIPS_III`]: IsaVersion::MIPS_III
    #[must_use]
    pub const fn default() -> Self {
        Self::MIPS_III
    }
}

impl Default for IsaVersion {
    fn default() -> Self {
        Self::default()
    }
}
