/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::isa::IsaVersion;

impl IsaVersion {
    /// Returns a default value.
    ///
    /// This defaults to [`MIPS_I`].
    ///
    /// [`MIPS_I`]: IsaVersion::MIPS_I
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
