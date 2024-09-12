/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub const fn mask(value: u32, width: u32) -> u32 {
    value & ((1 << width) - 1)
}

pub const fn bitmask(shift: u32, width: u32) -> u32 {
    mask(u32::MAX, width) << shift
}
