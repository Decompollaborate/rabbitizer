/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::InstrSuffix;

impl InstrSuffix {
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
