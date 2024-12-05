/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{fmt, ops};

use crate::vram::Vram;

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
/// use rabbitizer::vram::{VramOffset, Vram};
///
/// let offset = VramOffset::new(-0x10);
/// let vram = Vram::new(0x80000100);
///
/// assert_eq!(vram + offset, Vram::new(0x800000F0));
/// ```
///
/// [`Vram`]: crate::vram::Vram
/// [`add_vram`]: VramOffset::add_vram
/// [`inner`]: VramOffset::inner
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    /// use rabbitizer::vram::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0x8);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert_eq!(offset.add_vram(&vram), Vram::new(0x80000108));
    /// ```
    ///
    /// [`Vram`]: crate::vram::Vram
    #[must_use]
    pub const fn add_vram(&self, rhs: &Vram) -> Vram {
        rhs.add_offset(self)
    }

    /// This offset has value zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::vram::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert!(offset.is_zero());
    /// assert_eq!(offset + vram, vram);
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.inner == 0
    }

    /// This offset is positive.
    ///
    /// If this is a branch offset then it can be interpreted as a forward branch.
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::vram::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0x20);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert!(offset.is_positive());
    /// assert_eq!((offset + vram).inner(), 0x80000120);
    /// ```
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        self.inner > 0
    }

    /// This offset is positive.
    ///
    /// If this is a branch offset then it can be interpreted as a backwards branch (i.e. a loop).
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::vram::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(-0x20);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert!(offset.is_negative());
    /// assert_eq!((offset + vram).inner(), 0x800000E0);
    /// ```
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.inner < 0
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

impl fmt::Debug for VramOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VramOffset {{ ")?;

        // `-2^31` fits on an i32, but `-(-2^31)` doesn't, so we cast to i64 to
        // avoid overflowing.
        let mut inner = self.inner as i64;
        if inner < 0 {
            inner = -inner;
            write!(f, "-")?;
        }
        write!(f, "0x{:X} }}", inner)
    }
}

impl fmt::Display for VramOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.inner)
    }
}
