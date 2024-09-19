/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::IsaExtension;

impl IsaExtension {
    #[must_use]
    pub const fn default() -> Self {
        Self::NONE
    }
}

impl Default for IsaExtension {
    fn default() -> Self {
        Self::default()
    }
}
