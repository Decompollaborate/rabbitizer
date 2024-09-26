/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub const fn mask(value: u32, width: u32) -> u32 {
    value & ((1 << width) - 1)
}

pub const fn bitmask(shift: u32, width: u32) -> u32 {
    mask(u32::MAX, width) << shift
}

pub const fn from_2s_complement(number: u32, width: u32) -> i32 {
    let is_negative = number & (1 << (width - 1)) != 0;

    if is_negative {
        -(mask(!number + 1, width) as i32)
    } else {
        number as i32
    }
}
