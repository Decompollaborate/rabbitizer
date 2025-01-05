/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

#[cfg(not(feature = "std"))]
use core::error;
#[cfg(feature = "std")]
use std::error;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Error {
    OutOfRangeRegisterIndex {
        index: u32,
        count: u32,
        register_kind: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OutOfRangeRegisterIndex {
                index,
                count,
                register_kind,
            } => write!(
                f,
                "Index '{}' is out of range for register kind '{}', max value is {}.",
                index, register_kind, count
            ),
        }
    }
}

impl error::Error for Error {}
