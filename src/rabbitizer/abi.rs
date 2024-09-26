/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::Abi;

impl Abi {
    #[must_use]
    pub const fn default() -> Self {
        Self::O32
    }
}

impl Default for Abi {
    fn default() -> Self {
        Self::default()
    }
}

impl From<&str> for Abi {
    fn from(value: &str) -> Self {
        match value {
            "32" | "o32" | "O32" => Self::O32,
            "n32" | "N32" => Self::N32,
            "64" | "n64" | "N64" => Self::N64,
            _ => Self::O32,
        }
    }
}
