/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{fmt, ops};

use crate::Vram;

/// Holds the offset (in bytes) or difference between two [`Vram`] addresses.
///
/// Note the offset hold by this instance may be negative.
///
/// This struct can be used to modify a [`Vram`] instance. This can be done by
/// either the [`add_vram`] function or by using the `+` operator.
///
/// To get the raw inner value use the [`inner`] function.
///
/// # Examples
///
/// ```
/// use rabbitizer::{VramOffset, Vram};
///
/// let offset = VramOffset::new(-0x10);
/// let vram = Vram::new(0x80000100);
///
/// assert_eq!(vram + offset, Vram::new(0x800000F0));
/// ```
///
/// [`Vram`]: crate::Vram
/// [`add_vram`]: VramOffset::add_vram
/// [`inner`]: VramOffset::inner
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VramOffset {
    inner: i32,
}

impl VramOffset {
    /// Constructs a `VramOffset` from a given value.
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self { inner: value }
    }

    /// Returns the internal branch offset value.
    #[must_use]
    pub const fn inner(&self) -> i32 {
        self.inner
    }

    /// Adds this offset to the passed [`Vram`] value and generates a new
    /// [`Vram`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0x8);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert_eq!(offset.add_vram(&vram), Vram::new(0x80000108));
    /// ```
    ///
    /// [`Vram`]: crate::Vram
    #[must_use]
    pub const fn add_vram(&self, rhs: &Vram) -> Vram {
        rhs.add_offset(self)
    }
}

impl ops::Add<Vram> for VramOffset {
    type Output = Vram;

    fn add(self, rhs: Vram) -> Self::Output {
        self.add_vram(&rhs)
    }
}

impl ops::Add<&Vram> for VramOffset {
    type Output = Vram;

    fn add(self, rhs: &Vram) -> Self::Output {
        self.add_vram(rhs)
    }
}

impl fmt::Display for VramOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.inner)
    }
}
