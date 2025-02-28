/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::isa::IsaVersion;

impl IsaVersion {
    /// Returns a default ISA version.
    ///
    /// The returned ISA version may change depending on the configured features.
    #[must_use]
    pub const fn default() -> Self {
        #[cfg(feature = "MIPS_III")]
        {
            Self::MIPS_III
        }
        #[cfg(not(feature = "MIPS_III"))]
        {
            Self::MIPS_I
        }
    }
}

impl Default for IsaVersion {
    fn default() -> Self {
        Self::default()
    }
}
