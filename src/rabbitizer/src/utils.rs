/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[must_use]
pub(crate) const fn mask(value: u32, width: u32) -> u32 {
    assert!(
        width < 32,
        "This operation is defined only for bitwidths up to 31 bits."
    );

    value & ((1 << width) - 1)
}

#[must_use]
pub(crate) const fn bitmask(shift: u32, width: u32) -> u32 {
    assert!(
        shift + width <= 32,
        "Can't create a bitmask larger than 32 bits"
    );

    mask(u32::MAX, width) << shift
}

#[cfg(any(feature = "R4000ALLEGREX", feature = "R5900EE"))]
#[must_use]
pub(crate) const fn from_2s_complement<const WIDTH: u32>(number: u32) -> i32 {
    const {
        assert!(
            WIDTH < 32,
            "This operation is defined only for bitwidths between 1 and 31 bits."
        );
        assert!(
            WIDTH > 0,
            "This operation is defined only for bitwidths between 1 and 31 bits."
        );
    }

    let is_negative = number & (1 << (WIDTH - 1)) != 0;

    if is_negative {
        -(mask(!number + 1, WIDTH) as i32)
    } else {
        number as i32
    }
}

#[must_use]
#[cfg(feature = "R4000ALLEGREX")]
pub(crate) const fn floatrepr_32_from_16(mut arg: u16) -> u32 {
    // IEEE754 16-bit floats are encoded in 16 bits as follows:
    // Sign bit: 1 bit (bit 15)
    // Encoded exponent: 5 bits (bits 10 ~ 15)
    // Fraction/Mantissa: 10 bits (bits 0 ~ 9)

    let mut ret: u32 = 0;
    let sign: i32 = (arg as i32) >> 15;

    // If parameter is zero, then return zero
    if (arg & !(1 << 15)) == 0 {
        // Preserve the sign
        ret |= (sign as u32) << 31;
        return ret;
    }

    // Clear up the sign
    arg &= !(1 << 15);

    let encoded_exponent: i32 = arg as i32 >> 10;
    // Clear up the encoded exponent
    arg &= !0x7C00;

    // Exponent bias: 0xF
    let real_exponent: i32 = encoded_exponent - 0xF;

    let mantissa_is_zero: bool = arg == 0;

    if encoded_exponent == 0 {
        // subnormals

        ret |= (sign as u32) << 31;
        // no need to set the exponent part since it was already zero'd

        // Set the mantissa
        ret |= (arg as u32) >> (23 - 10);

        return ret;
    }

    if encoded_exponent == 0x1F {
        // Infinity and NaN

        ret |= (sign as u32) << 31;
        ret |= 0x7F800000;

        if !mantissa_is_zero {
            // NaN

            // Set the mantissa to any non-zero value
            ret |= (arg as u32) << (23 - 10);
        }

        return ret;
    }

    ret |= (sign as u32) << 31;

    // re-encode the exponent
    ret |= ((real_exponent + 0x7F) as u32) << 23;

    // Set the mantissa
    ret |= (arg as u32) << (23 - 10);

    ret
}

/// If `a` is `true` then `b` must be `true` too. If `a` is `false` then we
/// don't care about `b` and return `true`.
///
/// The above statement is expressed as the following truth table:
///
/// | a | b | OUT |
/// |---|---|-----|
/// | 1 | 1 |  1  |
/// | 1 | 0 |  0  |
/// | 0 | 1 |  1  |
/// | 0 | 0 |  1  |
#[inline(always)]
#[must_use]
pub(crate) const fn truth_a_implies_b(a: bool, b: bool) -> bool {
    !a || b
}

/// Returns `true` if both `a` and `b` are `true` or if both are `false`.
///
/// The above statement is expressed as the following truth table:
///
/// | a | b | OUT |
/// |---|---|-----|
/// | 1 | 1 |  1  |
/// | 1 | 0 |  0  |
/// | 0 | 1 |  0  |
/// | 0 | 0 |  1  |
#[inline(always)]
#[must_use]
pub(crate) const fn truth_both_or_neither(a: bool, b: bool) -> bool {
    !(a ^ b)
}

pub(crate) fn array_len_non_default<T, const N: usize>(array: &[T; N]) -> usize
where
    T: Default + PartialEq,
{
    let mut end_aux = N;
    let default = T::default();
    loop {
        if end_aux == 0 {
            break 0;
        }
        let end2 = end_aux - 1;
        if array[end2] != default {
            break end_aux;
        }
        end_aux = end2;
    }
}

#[cfg(feature = "encoder")]
pub struct DoubleOptIterator<I>
where
    I: Iterator,
{
    iter: I,
}

#[cfg(feature = "encoder")]
impl<I> DoubleOptIterator<I>
where
    I: Iterator,
{
    pub const fn new(iter: I) -> Self {
        Self { iter }
    }
}

#[cfg(feature = "encoder")]
impl<I> Iterator for DoubleOptIterator<I>
where
    I: Iterator,
{
    type Item = (I::Item, Option<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(x) => Some((x, self.iter.next())),
        }
    }
}

#[cfg(feature = "encoder")]
pub fn i32_hex_from_str(s: &str) -> Result<i32, core::num::ParseIntError> {
    let is_negative = s.starts_with('-');
    let s = s.trim_start_matches('-');

    let value = if s.starts_with("0x") || s.starts_with("0X") {
        i32::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)?
    } else {
        s.parse()?
    };

    Ok(if is_negative { -value } else { value })
}

#[cfg(feature = "encoder")]
pub fn u32_hex_from_str(s: &str) -> Result<u32, core::num::ParseIntError> {
    let value = if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)?
    } else {
        s.parse()?
    };

    Ok(value)
}

#[cfg(feature = "encoder")]
pub fn i16_hex_from_str(s: &str) -> Result<i16, core::num::ParseIntError> {
    let is_negative = s.starts_with('-');
    let s = s.trim_start_matches('-');

    let value = if s.starts_with("0x") || s.starts_with("0X") {
        i16::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)?
    } else {
        s.parse()?
    };

    Ok(if is_negative { -value } else { value })
}

#[cfg(feature = "encoder")]
pub fn u16_hex_from_str(s: &str) -> Result<u16, core::num::ParseIntError> {
    let value = if s.starts_with("0x") || s.starts_with("0X") {
        u16::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)?
    } else {
        s.parse()?
    };

    Ok(value)
}

#[cfg(feature = "encoder")]
pub fn i8_hex_from_str(s: &str) -> Result<i8, core::num::ParseIntError> {
    let is_negative = s.starts_with('-');
    let s = s.trim_start_matches('-');

    let value = if s.starts_with("0x") || s.starts_with("0X") {
        i8::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)?
    } else {
        s.parse()?
    };

    Ok(if is_negative { -value } else { value })
}

#[cfg(feature = "encoder")]
pub fn u8_hex_from_str(s: &str) -> Result<u8, core::num::ParseIntError> {
    let value = if s.starts_with("0x") || s.starts_with("0X") {
        u8::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)?
    } else {
        s.parse()?
    };

    Ok(value)
}
