/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{fmt, ops};

use crate::vram::VramOffset;

/// A VRAM (Virtual RAM) address.
///
/// A Vram address could be modified by a [`VramOffset`] instance. This can
/// be done by either the [`add_offset`] function or by using the `+` operator.
///
/// It is also possible to calculate the difference between two Vram addresses,
/// which will return a [`VramOffset`] instance. This can be done with the
/// [`sub_vram`] function or by using the `-` operator.
///
/// To get the raw inner value use the [`inner`] function.
///
/// [`VramOffset`]: crate::vram::VramOffset
/// [`add_offset`]: Vram::add_offset
/// [`sub_vram`]: Vram::sub_vram
/// [`inner`]: Vram::inner
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vram {
    inner: u32,
}

impl Vram {
    /// Constructs a `Vram` from a given value.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    /// Returns the internal vram value.
    #[must_use]
    pub const fn inner(&self) -> u32 {
        self.inner
    }

    /// Adds an offset to this Vram, generating a new Vram value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::vram::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0x8);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert_eq!(vram.add_offset(&offset), Vram::new(0x80000108));
    /// ```
    #[must_use]
    pub const fn add_offset(&self, rhs: &VramOffset) -> Self {
        Self::new(self.inner.wrapping_add_signed(rhs.inner()))
    }

    /// Subtracts a Vram to this Vram.
    ///
    /// # Examples
    ///
    /// ```
    /// use rabbitizer::vram::{VramOffset, Vram};
    ///
    /// let vram_a = Vram::new(0x80000100);
    /// let vram_b = Vram::new(0x80000140);
    ///
    /// assert_eq!(vram_a.sub_vram(&vram_b), VramOffset::new(-0x40));
    /// ```
    #[must_use]
    pub const fn sub_vram(&self, rhs: &Self) -> VramOffset {
        VramOffset::new((self.inner as i32).wrapping_sub_unsigned(rhs.inner()))
    }
}

impl ops::Add<VramOffset> for Vram {
    type Output = Self;

    fn add(self, rhs: VramOffset) -> Self::Output {
        self.add_offset(&rhs)
    }
}

impl ops::Add<&VramOffset> for Vram {
    type Output = Self;

    fn add(self, rhs: &VramOffset) -> Self::Output {
        self.add_offset(rhs)
    }
}

impl ops::AddAssign<VramOffset> for Vram {
    fn add_assign(&mut self, rhs: VramOffset) {
        *self = self.add_offset(&rhs)
    }
}

impl ops::AddAssign<&VramOffset> for Vram {
    fn add_assign(&mut self, rhs: &VramOffset) {
        *self = self.add_offset(rhs)
    }
}

impl ops::Sub<Self> for Vram {
    type Output = VramOffset;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_vram(&rhs)
    }
}

impl ops::Sub<&Self> for Vram {
    type Output = VramOffset;

    fn sub(self, rhs: &Self) -> Self::Output {
        self.sub_vram(rhs)
    }
}

impl fmt::Debug for Vram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vram {{ 0x{:08X} }}", self.inner)
    }
}

impl fmt::Display for Vram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08X}", self.inner)
    }
}
