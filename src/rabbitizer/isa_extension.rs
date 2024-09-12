/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::IsaExtension;

impl IsaExtension {
    pub const fn default() -> Self {
        Self::NONE
    }
}

impl Default for IsaExtension {
    fn default() -> Self {
        Self::NONE
    }
}
