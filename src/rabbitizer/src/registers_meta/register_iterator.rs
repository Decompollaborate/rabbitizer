/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{iter::FusedIterator, marker::PhantomData};

use super::Register;

/// A generic struct that allows iterating (by implementing the Iterator trait) every register of a
/// given register kind in ascending order.
///
/// The recommended way to instance this struct is by using the associated function [`iter()`] of
/// the desired register type.
///
/// # Examples
///
/// ```
/// use rabbitizer::{registers::Gpr, registers_meta::Register};
///
/// let mut gpr_iter = Gpr::iter();
///
/// assert_eq!(gpr_iter.next(), Some(Gpr::zero));
/// assert_eq!(gpr_iter.next(), Some(Gpr::at));
/// assert_eq!(gpr_iter.next(), Some(Gpr::v0));
/// ```
///
/// [`iter()`]: crate::registers_meta::Register::iter
pub struct RegisterIterator<T>
where
    T: Register,
{
    index: u32,
    end: u32,

    // We take advantage of the type system to allow making a single iterator for every register
    // type by "storing" the type in the struct itself.
    // This `PhantomData` is here only because the compiler requires every generic type to be
    // used in at least a single field of the struct.
    phantom: PhantomData<T>,
}

impl<T> RegisterIterator<T>
where
    T: Register,
{
    /// Constructs a register iterator.
    ///
    /// It is strongly recommended to use the associated function [`iter()`] of the register type
    /// instead of using this function directly.
    ///
    /// [`iter()`]: crate::registers_meta::Register::iter
    #[must_use]
    pub fn new() -> Self {
        Self {
            index: 0,
            end: T::count() as u32,
            phantom: PhantomData,
        }
    }
}

impl<T> Default for RegisterIterator<T>
where
    T: Register,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for RegisterIterator<T>
where
    T: Register + TryFrom<u32>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            None
        } else {
            let reg = T::try_from(self.index).ok();
            self.index = self.index.saturating_add(1);
            reg
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = self.index.saturating_add(n as u32);
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.index) as usize;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        // The size_hint is always accurate.
        self.size_hint().0
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    fn is_sorted(self) -> bool {
        // Registers are always sorted.
        true
    }

    fn max(self) -> Option<Self::Item> {
        // Since registers are always sorted then the last element **is** the
        // max element.
        self.last()
    }

    fn min(mut self) -> Option<Self::Item> {
        // Since registers are always sorted then the first element **is** the
        // min element.
        self.next()
    }
}

impl<T> DoubleEndedIterator for RegisterIterator<T>
where
    T: Register + TryFrom<u32>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            None
        } else {
            self.end = self.end.saturating_sub(1);
            T::try_from(self.end).ok()
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.end = self.end.saturating_sub(n as u32);
        self.next_back()
    }
}

impl<T> ExactSizeIterator for RegisterIterator<T> where T: Register + TryFrom<u32> {}
impl<T> FusedIterator for RegisterIterator<T> where T: Register + TryFrom<u32> {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::registers::Gpr;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_iterator_gpr() {
        let mut gpr_iter = Gpr::iter();

        assert_eq!(gpr_iter.next(), Some(Gpr::zero));
        assert_eq!(gpr_iter.next(), Some(Gpr::at));
        assert_eq!(gpr_iter.next(), Some(Gpr::v0));
        assert_eq!(gpr_iter.next(), Some(Gpr::v1));
        assert_eq!(gpr_iter.next(), Some(Gpr::a0));
        assert_eq!(gpr_iter.next(), Some(Gpr::a1));
        assert_eq!(gpr_iter.next(), Some(Gpr::a2));
        assert_eq!(gpr_iter.next(), Some(Gpr::a3));
        assert_eq!(gpr_iter.next(), Some(Gpr::t0));
        assert_eq!(gpr_iter.next(), Some(Gpr::t1));
        assert_eq!(gpr_iter.next(), Some(Gpr::t2));
        assert_eq!(gpr_iter.next(), Some(Gpr::t3));
        assert_eq!(gpr_iter.next(), Some(Gpr::t4));
        assert_eq!(gpr_iter.next(), Some(Gpr::t5));
        assert_eq!(gpr_iter.next(), Some(Gpr::t6));
        assert_eq!(gpr_iter.next(), Some(Gpr::t7));
        assert_eq!(gpr_iter.next(), Some(Gpr::s0));
        assert_eq!(gpr_iter.next(), Some(Gpr::s1));
        assert_eq!(gpr_iter.next(), Some(Gpr::s2));
        assert_eq!(gpr_iter.next(), Some(Gpr::s3));
        assert_eq!(gpr_iter.next(), Some(Gpr::s4));
        assert_eq!(gpr_iter.next(), Some(Gpr::s5));
        assert_eq!(gpr_iter.next(), Some(Gpr::s6));
        assert_eq!(gpr_iter.next(), Some(Gpr::s7));
        assert_eq!(gpr_iter.next(), Some(Gpr::t8));
        assert_eq!(gpr_iter.next(), Some(Gpr::t9));
        assert_eq!(gpr_iter.next(), Some(Gpr::k0));
        assert_eq!(gpr_iter.next(), Some(Gpr::k1));
        assert_eq!(gpr_iter.next(), Some(Gpr::gp));
        assert_eq!(gpr_iter.next(), Some(Gpr::sp));
        assert_eq!(gpr_iter.next(), Some(Gpr::s8));
        assert_eq!(gpr_iter.next(), Some(Gpr::ra));

        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_iterator_gpr_back() {
        let mut gpr_iter = Gpr::iter();

        assert_eq!(gpr_iter.next_back(), Some(Gpr::ra));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s8));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::sp));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::gp));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::k1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::k0));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t9));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t8));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s7));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s6));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s5));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s4));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s3));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s2));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s0));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t7));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t6));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t5));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t4));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t3));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t2));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t0));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::a3));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::a2));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::a1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::a0));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::v1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::v0));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::at));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::zero));

        assert_eq!(gpr_iter.next_back(), None);
        assert_eq!(gpr_iter.next_back(), None);
        assert_eq!(gpr_iter.next_back(), None);
        assert_eq!(gpr_iter.next_back(), None);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_iterator_gpr_forward_back() {
        let mut gpr_iter = Gpr::iter();

        assert_eq!(gpr_iter.next(), Some(Gpr::zero));
        assert_eq!(gpr_iter.next(), Some(Gpr::at));
        assert_eq!(gpr_iter.next(), Some(Gpr::v0));
        assert_eq!(gpr_iter.next(), Some(Gpr::v1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::ra));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s8));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::sp));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::gp));
        assert_eq!(gpr_iter.next(), Some(Gpr::a0));
        assert_eq!(gpr_iter.next(), Some(Gpr::a1));
        assert_eq!(gpr_iter.next(), Some(Gpr::a2));
        assert_eq!(gpr_iter.next(), Some(Gpr::a3));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::k1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::k0));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t9));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::t8));
        assert_eq!(gpr_iter.next(), Some(Gpr::t0));
        assert_eq!(gpr_iter.next(), Some(Gpr::t1));
        assert_eq!(gpr_iter.next(), Some(Gpr::t2));
        assert_eq!(gpr_iter.next(), Some(Gpr::t3));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s7));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s6));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s5));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s4));
        assert_eq!(gpr_iter.next(), Some(Gpr::t4));
        assert_eq!(gpr_iter.next(), Some(Gpr::t5));
        assert_eq!(gpr_iter.next(), Some(Gpr::t6));
        assert_eq!(gpr_iter.next(), Some(Gpr::t7));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s3));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s2));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s1));
        assert_eq!(gpr_iter.next_back(), Some(Gpr::s0));

        assert_eq!(gpr_iter.next_back(), None);
        assert_eq!(gpr_iter.next_back(), None);
        assert_eq!(gpr_iter.next_back(), None);
        assert_eq!(gpr_iter.next_back(), None);
    }

    #[test]
    fn test_iterator_gpr_nth() {
        let mut gpr_iter = Gpr::iter().skip(31);

        assert_eq!(gpr_iter.next(), Some(Gpr::ra));

        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);

        let mut gpr_iter = Gpr::iter();

        assert_eq!(gpr_iter.nth(31), Some(Gpr::ra));

        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next(), None);

        let mut gpr_iter = Gpr::iter();

        assert_eq!(gpr_iter.nth_back(31), Some(Gpr::zero));

        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next_back(), None);
        assert_eq!(gpr_iter.next(), None);
        assert_eq!(gpr_iter.next_back(), None);

        assert_eq!(Gpr::iter().count(), Gpr::count());

        let gpr_iter = Gpr::iter().skip(8);
        assert_eq!(gpr_iter.count(), Gpr::count() - 8);
    }

    #[test]
    fn test_iterator_gpr_last() {
        assert_eq!(Gpr::iter().last(), Some(Gpr::ra));

        let gpr_iter = Gpr::iter().skip(32);
        assert_eq!(gpr_iter.last(), None);

        let mut gpr_iter = Gpr::iter();
        gpr_iter.next_back();
        gpr_iter.next_back();
        assert_eq!(gpr_iter.last(), Some(Gpr::sp));
    }
}
