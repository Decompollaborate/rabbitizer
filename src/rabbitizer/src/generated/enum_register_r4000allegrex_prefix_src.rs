/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexPrefixSrc {
    x = 0,
    y = 1,
    z = 2,
    w = 3,
    abs_x = 4,
    abs_y = 5,
    abs_z = 6,
    abs_w = 7,
    zero = 8,
    one = 9,
    two = 10,
    one_half = 11,
    three = 12,
    one_third = 13,
    one_fourth = 14,
    one_sixth = 15,
    neg_x = 16,
    neg_y = 17,
    neg_z = 18,
    neg_w = 19,
    neg_abs_x = 20,
    neg_abs_y = 21,
    neg_abs_z = 22,
    neg_abs_w = 23,
    neg_zero = 24,
    neg_one = 25,
    neg_two = 26,
    neg_one_half = 27,
    neg_three = 28,
    neg_one_third = 29,
    neg_one_fourth = 30,
    neg_one_sixth = 31,
}
pub static R4000ALLEGREX_PREFIX_SRC: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[R4000AllegrexPrefixSrc::x as usize] = RegisterDescriptor {
        name: "X",
        name_numeric: "X",
        ..RegisterDescriptor::new("x", 0, concat!("$", "0"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::y as usize] = RegisterDescriptor {
        name: "Y",
        name_numeric: "Y",
        ..RegisterDescriptor::new("y", 1, concat!("$", "1"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::z as usize] = RegisterDescriptor {
        name: "Z",
        name_numeric: "Z",
        ..RegisterDescriptor::new("z", 2, concat!("$", "2"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::w as usize] = RegisterDescriptor {
        name: "W",
        name_numeric: "W",
        ..RegisterDescriptor::new("w", 3, concat!("$", "3"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::abs_x as usize] = RegisterDescriptor {
        name: "|X|",
        name_numeric: "|X|",
        ..RegisterDescriptor::new("abs_x", 4, concat!("$", "4"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::abs_y as usize] = RegisterDescriptor {
        name: "|Y|",
        name_numeric: "|Y|",
        ..RegisterDescriptor::new("abs_y", 5, concat!("$", "5"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::abs_z as usize] = RegisterDescriptor {
        name: "|Z|",
        name_numeric: "|Z|",
        ..RegisterDescriptor::new("abs_z", 6, concat!("$", "6"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::abs_w as usize] = RegisterDescriptor {
        name: "|W|",
        name_numeric: "|W|",
        ..RegisterDescriptor::new("abs_w", 7, concat!("$", "7"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::zero as usize] = RegisterDescriptor {
        name: "0",
        name_numeric: "0",
        ..RegisterDescriptor::new("zero", 8, concat!("$", "8"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::one as usize] = RegisterDescriptor {
        name: "1",
        name_numeric: "1",
        ..RegisterDescriptor::new("one", 9, concat!("$", "9"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::two as usize] = RegisterDescriptor {
        name: "2",
        name_numeric: "2",
        ..RegisterDescriptor::new("two", 10, concat!("$", "10"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::one_half as usize] = RegisterDescriptor {
        name: "1/2",
        name_numeric: "1/2",
        ..RegisterDescriptor::new("one_half", 11, concat!("$", "11"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::three as usize] = RegisterDescriptor {
        name: "3",
        name_numeric: "3",
        ..RegisterDescriptor::new("three", 12, concat!("$", "12"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::one_third as usize] = RegisterDescriptor {
        name: "1/3",
        name_numeric: "1/3",
        ..RegisterDescriptor::new("one_third", 13, concat!("$", "13"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::one_fourth as usize] = RegisterDescriptor {
        name: "1/4",
        name_numeric: "1/4",
        ..RegisterDescriptor::new("one_fourth", 14, concat!("$", "14"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::one_sixth as usize] = RegisterDescriptor {
        name: "1/6",
        name_numeric: "1/6",
        ..RegisterDescriptor::new("one_sixth", 15, concat!("$", "15"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_x as usize] = RegisterDescriptor {
        name: "-X",
        name_numeric: "-X",
        ..RegisterDescriptor::new("neg_x", 16, concat!("$", "16"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_y as usize] = RegisterDescriptor {
        name: "-Y",
        name_numeric: "-Y",
        ..RegisterDescriptor::new("neg_y", 17, concat!("$", "17"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_z as usize] = RegisterDescriptor {
        name: "-Z",
        name_numeric: "-Z",
        ..RegisterDescriptor::new("neg_z", 18, concat!("$", "18"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_w as usize] = RegisterDescriptor {
        name: "-W",
        name_numeric: "-W",
        ..RegisterDescriptor::new("neg_w", 19, concat!("$", "19"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_abs_x as usize] = RegisterDescriptor {
        name: "-|X|",
        name_numeric: "-|X|",
        ..RegisterDescriptor::new("neg_abs_x", 20, concat!("$", "20"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_abs_y as usize] = RegisterDescriptor {
        name: "-|Y|",
        name_numeric: "-|Y|",
        ..RegisterDescriptor::new("neg_abs_y", 21, concat!("$", "21"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_abs_z as usize] = RegisterDescriptor {
        name: "-|Z|",
        name_numeric: "-|Z|",
        ..RegisterDescriptor::new("neg_abs_z", 22, concat!("$", "22"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_abs_w as usize] = RegisterDescriptor {
        name: "-|W|",
        name_numeric: "-|W|",
        ..RegisterDescriptor::new("neg_abs_w", 23, concat!("$", "23"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_zero as usize] = RegisterDescriptor {
        name: "-0",
        name_numeric: "-0",
        ..RegisterDescriptor::new("neg_zero", 24, concat!("$", "24"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_one as usize] = RegisterDescriptor {
        name: "-1",
        name_numeric: "-1",
        ..RegisterDescriptor::new("neg_one", 25, concat!("$", "25"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_two as usize] = RegisterDescriptor {
        name: "-2",
        name_numeric: "-2",
        ..RegisterDescriptor::new("neg_two", 26, concat!("$", "26"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_one_half as usize] = RegisterDescriptor {
        name: "-1/2",
        name_numeric: "-1/2",
        ..RegisterDescriptor::new("neg_one_half", 27, concat!("$", "27"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_three as usize] = RegisterDescriptor {
        name: "-3",
        name_numeric: "-3",
        ..RegisterDescriptor::new("neg_three", 28, concat!("$", "28"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_one_third as usize] = RegisterDescriptor {
        name: "-1/3",
        name_numeric: "-1/3",
        ..RegisterDescriptor::new("neg_one_third", 29, concat!("$", "29"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_one_fourth as usize] = RegisterDescriptor {
        name: "-1/4",
        name_numeric: "-1/4",
        ..RegisterDescriptor::new("neg_one_fourth", 30, concat!("$", "30"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixSrc::neg_one_sixth as usize] = RegisterDescriptor {
        name: "-1/6",
        name_numeric: "-1/6",
        ..RegisterDescriptor::new("neg_one_sixth", 31, concat!("$", "31"), false)
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value() as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexPrefixSrc {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::x),
            1 => Ok(Self::y),
            2 => Ok(Self::z),
            3 => Ok(Self::w),
            4 => Ok(Self::abs_x),
            5 => Ok(Self::abs_y),
            6 => Ok(Self::abs_z),
            7 => Ok(Self::abs_w),
            8 => Ok(Self::zero),
            9 => Ok(Self::one),
            10 => Ok(Self::two),
            11 => Ok(Self::one_half),
            12 => Ok(Self::three),
            13 => Ok(Self::one_third),
            14 => Ok(Self::one_fourth),
            15 => Ok(Self::one_sixth),
            16 => Ok(Self::neg_x),
            17 => Ok(Self::neg_y),
            18 => Ok(Self::neg_z),
            19 => Ok(Self::neg_w),
            20 => Ok(Self::neg_abs_x),
            21 => Ok(Self::neg_abs_y),
            22 => Ok(Self::neg_abs_z),
            23 => Ok(Self::neg_abs_w),
            24 => Ok(Self::neg_zero),
            25 => Ok(Self::neg_one),
            26 => Ok(Self::neg_two),
            27 => Ok(Self::neg_one_half),
            28 => Ok(Self::neg_three),
            29 => Ok(Self::neg_one_third),
            30 => Ok(Self::neg_one_fourth),
            31 => Ok(Self::neg_one_sixth),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 32,
                register_kind: "R4000AllegrexPrefixSrc",
            }),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        32
    }
}
impl TryFrom<u32> for R4000AllegrexPrefixSrc {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexPrefixSrc {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexPrefixSrc> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexPrefixSrc) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R4000ALLEGREX_PREFIX_SRC {
            if x.has_dollar {
                assert!(
                    x.name.starts_with('$'),
                    "Register {} is missing dollar sign",
                    x.name
                );
                assert!(
                    x.name_o32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_o32
                );
                assert!(
                    x.name_o64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_o64
                );
                assert!(
                    x.name_n32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_n32
                );
                assert!(
                    x.name_n64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_n64
                );
                assert!(
                    x.name_eabi32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_eabi32
                );
                assert!(
                    x.name_eabi64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_eabi64
                );
            } else {
                assert!(
                    !x.name.starts_with('$'),
                    "Register {} has dollar sign when it shouldn't",
                    x.name
                );
                assert!(
                    x.name_o32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_o32
                );
                assert!(
                    x.name_o64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_o64
                );
                assert!(
                    x.name_n32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_n32
                );
                assert!(
                    x.name_n64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_n64
                );
                assert!(
                    x.name_eabi32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_eabi32
                );
                assert!(
                    x.name_eabi64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_eabi64
                );
            }
        }
    }
}
