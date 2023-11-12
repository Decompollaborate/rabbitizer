/* SPDX-FileCopyrightText: © 2022-2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub type SizeT = usize;

pub fn c_string_from_str(str: Option<&str>) -> (*const core::ffi::c_char, SizeT) {
    if let Some(str) = str {
        (str.as_ptr() as *const core::ffi::c_char, str.len())
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

pub fn bitrepack(fullword: u32, v: u32, s: u32, w: u32) -> u32 {
    (shiftr(fullword, s + w, 32 - (s + w)) << (s + w)) | shiftl(v, s, w) | mask(fullword, s)
}

pub fn convert_option_string_to_option_str(input: &Option<String>) -> Option<&str> {
    match input {
        None => None,
        Some(x) => Some(x.as_str()),
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
pub enum TrinaryValue {
    NONE,
    FALSE,
    TRUE,
}
