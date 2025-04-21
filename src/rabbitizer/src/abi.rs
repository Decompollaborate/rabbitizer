/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub use crate::generated::Abi;

impl Abi {
    /// Returns a default value.
    ///
    /// Defaults to [`O32`].
    ///
    /// [`O32`]: Abi::O32
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
            "o64" | "O64" => Self::O64,
            "n32" | "N32" => Self::N32,
            "64" | "n64" | "N64" => Self::N64,
            "eabi32" | "EABI32" => Self::EABI32,
            "eabi64" | "EABI64" => Self::EABI64,
            _ => Self::default(),
        }
    }
}

#[cfg(feature = "pyo3")]
pub(crate) mod python_bindings {
    use super::*;

    #[pymethods]
    impl Abi {
        #[pyo3(name = "from_name")]
        #[staticmethod]
        #[must_use]
        pub fn from_name(name: &str) -> Self {
            name.into()
        }
    }
}
