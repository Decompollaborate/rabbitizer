/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

#[cfg(not(feature = "std"))]
use core::error;
#[cfg(feature = "std")]
use std::error;

/// An error which can be returned when parsing a register from an integer.
///
/// This error is used as the error type for the `try_from_u32()` functions on
/// Register types and traits, such as [`Gpr::try_from_u32`] or
/// [`Register::try_from_u32`].
///
/// # Potential causes
///
/// The main cause for this issue to happen is by trying to convert a big
/// enough value into a Register type, i.e. trying to convert a value equal or
/// larger than 32 into a [`Gpr`] Register.
///
/// [`Gpr`]: crate::registers::Gpr
/// [`Gpr::try_from_u32`]: crate::registers::Gpr::try_from_u32
/// [`Register::try_from_u32`]: crate::registers_meta::Register::try_from_u32
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntRegisterConversionError {
    index: u32,
    count: u32,
    register_kind: &'static str,
}

impl IntRegisterConversionError {
    pub(crate) const fn new_out_of_range(
        index: u32,
        count: u32,
        register_kind: &'static str,
    ) -> Self {
        Self {
            index,
            count,
            register_kind,
        }
    }
}

impl fmt::Display for IntRegisterConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Index '{}' is out of range for register kind '{}', max value is {}.",
            self.index, self.register_kind, self.count
        )
    }
}

impl error::Error for IntRegisterConversionError {}
