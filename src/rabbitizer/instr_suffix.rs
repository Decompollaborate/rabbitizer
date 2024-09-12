/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::InstrSuffix;

impl InstrSuffix {
    pub const fn default() -> Self {
        Self::ALL_NONE
    }
}

impl Default for InstrSuffix {
    fn default() -> Self {
        Self::ALL_NONE
    }
}