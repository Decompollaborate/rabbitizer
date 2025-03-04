/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[must_use]
pub const fn mask(value: u32, width: u32) -> u32 {
    assert!(
        width < 32,
        "This operation is defined only for bitwidths up to 31 bits."
    );

    value & ((1 << width) - 1)
}

#[must_use]
pub const fn bitmask(shift: u32, width: u32) -> u32 {
    assert!(
        shift + width <= 32,
        "Can't create a bitmask larger than 32 bits"
    );

    mask(u32::MAX, width) << shift
}

#[must_use]
pub const fn from_2s_complement(number: u32, width: u32) -> i32 {
    assert!(
        width < 32,
        "This operation is defined only for bitwidths between 1 and 31 bits."
    );
    assert!(
        width > 0,
        "This operation is defined only for bitwidths between 1 and 31 bits."
    );

    let is_negative = number & (1 << (width - 1)) != 0;

    if is_negative {
        -(mask(!number + 1, width) as i32)
    } else {
        number as i32
    }
}

#[must_use]
#[cfg(feature = "R4000ALLEGREX")]
pub const fn floatrepr_32_from_16(mut arg: u16) -> u32 {
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
pub const fn truth_a_implies_b(a: bool, b: bool) -> bool {
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
pub const fn truth_both_or_neither(a: bool, b: bool) -> bool {
    !(a ^ b)
}
