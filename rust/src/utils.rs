/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub type SizeT = usize;


pub fn c_string_from_str(str: Option<&str>) -> (*const core::ffi::c_char, SizeT) {
    if let Some(str) = str {
        (str.as_ptr() as *const core::ffi::c_char, str.len().try_into().unwrap())
    } else {
        (std::ptr::null(), 0)
    }
}

pub fn mask(v: u32, w: u32) -> u32 {
    v & ((1 << w) - 1)
}

pub fn shiftl(v: u32, s: u32, w: u32) -> u32 {
    mask(v, w) << s
}

pub fn shiftr(v: u32, s: u32, w: u32) -> u32 {
    mask(v >> s, w)
}
