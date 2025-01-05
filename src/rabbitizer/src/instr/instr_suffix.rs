/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::instr::InstrSuffix;

impl InstrSuffix {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::ALL_NONE
    }
}

impl Default for InstrSuffix {
    fn default() -> Self {
        Self::ALL_NONE
    }
}
