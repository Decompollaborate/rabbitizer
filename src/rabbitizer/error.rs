/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{error, fmt};

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
