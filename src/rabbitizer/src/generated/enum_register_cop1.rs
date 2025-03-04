/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum Cop1 {
    fv0 = 0,
    fv0f = 1,
    fv1 = 2,
    fv1f = 3,
    ft0 = 4,
    ft0f = 5,
    ft1 = 6,
    ft1f = 7,
    ft2 = 8,
    ft2f = 9,
    ft3 = 10,
    ft3f = 11,
    fa0 = 12,
    fa0f = 13,
    fa1 = 14,
    fa1f = 15,
    ft4 = 16,
    ft4f = 17,
    ft5 = 18,
    ft5f = 19,
    fs0 = 20,
    fs0f = 21,
    fs1 = 22,
    fs1f = 23,
    fs2 = 24,
    fs2f = 25,
    fs3 = 26,
    fs3f = 27,
    fs4 = 28,
    fs4f = 29,
    fs5 = 30,
    fs5f = 31,
}
pub static COP1: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[Cop1::fv0 as usize] = RegisterDescriptor {
        name_o64: Some("$fv0"),
        name_n32: Some("$fv0"),
        name_n64: Some("$fv0"),
        name_eabi32: Some("$fv0"),
        name_eabi64: Some("$fv0"),
        ..RegisterDescriptor::new(concat!("$", "fv0"), 0, concat!("$f", "0"), true)
    }
    .check_panic_chain();
    table[Cop1::fv0f as usize] = RegisterDescriptor {
        name_o64: Some("$fv1"),
        name_n32: Some("$ft14"),
        name_n64: Some("$ft12"),
        name_eabi32: Some("$fv0f"),
        name_eabi64: Some("$fv1"),
        ..RegisterDescriptor::new(concat!("$", "fv0f"), 1, concat!("$f", "1"), true)
    }
    .check_panic_chain();
    table[Cop1::fv1 as usize] = RegisterDescriptor {
        name_o64: Some("$ft0"),
        name_n32: Some("$fv1"),
        name_n64: Some("$fv1"),
        name_eabi32: Some("$fv1"),
        name_eabi64: Some("$ft0"),
        ..RegisterDescriptor::new(concat!("$", "fv1"), 2, concat!("$f", "2"), true)
    }
    .check_panic_chain();
    table[Cop1::fv1f as usize] = RegisterDescriptor {
        name_o64: Some("$ft1"),
        name_n32: Some("$ft15"),
        name_n64: Some("$ft13"),
        name_eabi32: Some("$fv1f"),
        name_eabi64: Some("$ft1"),
        ..RegisterDescriptor::new(concat!("$", "fv1f"), 3, concat!("$f", "3"), true)
    }
    .check_panic_chain();
    table[Cop1::ft0 as usize] = RegisterDescriptor {
        name_o64: Some("$ft2"),
        name_n32: Some("$ft0"),
        name_n64: Some("$ft0"),
        name_eabi32: Some("$ft0"),
        name_eabi64: Some("$ft2"),
        ..RegisterDescriptor::new(concat!("$", "ft0"), 4, concat!("$f", "4"), true)
    }
    .check_panic_chain();
    table[Cop1::ft0f as usize] = RegisterDescriptor {
        name_o64: Some("$ft3"),
        name_n32: Some("$ft1"),
        name_n64: Some("$ft1"),
        name_eabi32: Some("$ft0f"),
        name_eabi64: Some("$ft3"),
        ..RegisterDescriptor::new(concat!("$", "ft0f"), 5, concat!("$f", "5"), true)
    }
    .check_panic_chain();
    table[Cop1::ft1 as usize] = RegisterDescriptor {
        name_o64: Some("$ft4"),
        name_n32: Some("$ft2"),
        name_n64: Some("$ft2"),
        name_eabi32: Some("$ft1"),
        name_eabi64: Some("$ft4"),
        ..RegisterDescriptor::new(concat!("$", "ft1"), 6, concat!("$f", "6"), true)
    }
    .check_panic_chain();
    table[Cop1::ft1f as usize] = RegisterDescriptor {
        name_o64: Some("$ft5"),
        name_n32: Some("$ft3"),
        name_n64: Some("$ft3"),
        name_eabi32: Some("$ft1f"),
        name_eabi64: Some("$ft5"),
        ..RegisterDescriptor::new(concat!("$", "ft1f"), 7, concat!("$f", "7"), true)
    }
    .check_panic_chain();
    table[Cop1::ft2 as usize] = RegisterDescriptor {
        name_o64: Some("$ft6"),
        name_n32: Some("$ft4"),
        name_n64: Some("$ft4"),
        name_eabi32: Some("$ft2"),
        name_eabi64: Some("$ft6"),
        ..RegisterDescriptor::new(concat!("$", "ft2"), 8, concat!("$f", "8"), true)
    }
    .check_panic_chain();
    table[Cop1::ft2f as usize] = RegisterDescriptor {
        name_o64: Some("$ft7"),
        name_n32: Some("$ft5"),
        name_n64: Some("$ft5"),
        name_eabi32: Some("$ft2f"),
        name_eabi64: Some("$ft7"),
        ..RegisterDescriptor::new(concat!("$", "ft2f"), 9, concat!("$f", "9"), true)
    }
    .check_panic_chain();
    table[Cop1::ft3 as usize] = RegisterDescriptor {
        name_o64: Some("$ft8"),
        name_n32: Some("$ft6"),
        name_n64: Some("$ft6"),
        name_eabi32: Some("$ft3"),
        name_eabi64: Some("$ft8"),
        ..RegisterDescriptor::new(concat!("$", "ft3"), 10, concat!("$f", "10"), true)
    }
    .check_panic_chain();
    table[Cop1::ft3f as usize] = RegisterDescriptor {
        name_o64: Some("$ft9"),
        name_n32: Some("$ft7"),
        name_n64: Some("$ft7"),
        name_eabi32: Some("$ft3f"),
        name_eabi64: Some("$ft9"),
        ..RegisterDescriptor::new(concat!("$", "ft3f"), 11, concat!("$f", "11"), true)
    }
    .check_panic_chain();
    table[Cop1::fa0 as usize] = RegisterDescriptor {
        name_o64: Some("$fa0"),
        name_n32: Some("$fa0"),
        name_n64: Some("$fa0"),
        name_eabi32: Some("$fa0"),
        name_eabi64: Some("$fa0"),
        ..RegisterDescriptor::new(concat!("$", "fa0"), 12, concat!("$f", "12"), true)
    }
    .check_panic_chain();
    table[Cop1::fa0f as usize] = RegisterDescriptor {
        name_o64: Some("$fa1"),
        name_n32: Some("$fa1"),
        name_n64: Some("$fa1"),
        name_eabi32: Some("$fa0f"),
        name_eabi64: Some("$fa1"),
        ..RegisterDescriptor::new(concat!("$", "fa0f"), 13, concat!("$f", "13"), true)
    }
    .check_panic_chain();
    table[Cop1::fa1 as usize] = RegisterDescriptor {
        name_o64: Some("$ft10"),
        name_n32: Some("$fa2"),
        name_n64: Some("$fa2"),
        name_eabi32: Some("$fa1"),
        name_eabi64: Some("$fa2"),
        ..RegisterDescriptor::new(concat!("$", "fa1"), 14, concat!("$f", "14"), true)
    }
    .check_panic_chain();
    table[Cop1::fa1f as usize] = RegisterDescriptor {
        name_o64: Some("$ft11"),
        name_n32: Some("$fa3"),
        name_n64: Some("$fa3"),
        name_eabi32: Some("$fa1f"),
        name_eabi64: Some("$fa3"),
        ..RegisterDescriptor::new(concat!("$", "fa1f"), 15, concat!("$f", "15"), true)
    }
    .check_panic_chain();
    table[Cop1::ft4 as usize] = RegisterDescriptor {
        name_o64: Some("$ft12"),
        name_n32: Some("$fa4"),
        name_n64: Some("$fa4"),
        name_eabi32: Some("$fa2"),
        name_eabi64: Some("$fa4"),
        ..RegisterDescriptor::new(concat!("$", "ft4"), 16, concat!("$f", "16"), true)
    }
    .check_panic_chain();
    table[Cop1::ft4f as usize] = RegisterDescriptor {
        name_o64: Some("$ft13"),
        name_n32: Some("$fa5"),
        name_n64: Some("$fa5"),
        name_eabi32: Some("$fa2f"),
        name_eabi64: Some("$fa5"),
        ..RegisterDescriptor::new(concat!("$", "ft4f"), 17, concat!("$f", "17"), true)
    }
    .check_panic_chain();
    table[Cop1::ft5 as usize] = RegisterDescriptor {
        name_o64: Some("$ft14"),
        name_n32: Some("$fa6"),
        name_n64: Some("$fa6"),
        name_eabi32: Some("$fa3"),
        name_eabi64: Some("$fa6"),
        ..RegisterDescriptor::new(concat!("$", "ft5"), 18, concat!("$f", "18"), true)
    }
    .check_panic_chain();
    table[Cop1::ft5f as usize] = RegisterDescriptor {
        name_o64: Some("$ft15"),
        name_n32: Some("$fa7"),
        name_n64: Some("$fa7"),
        name_eabi32: Some("$fa3f"),
        name_eabi64: Some("$fa7"),
        ..RegisterDescriptor::new(concat!("$", "ft5f"), 19, concat!("$f", "19"), true)
    }
    .check_panic_chain();
    table[Cop1::fs0 as usize] = RegisterDescriptor {
        name_o64: Some("$fs0"),
        name_n32: Some("$fs0"),
        name_n64: Some("$ft8"),
        name_eabi32: Some("$fs0"),
        name_eabi64: Some("$fs0"),
        ..RegisterDescriptor::new(concat!("$", "fs0"), 20, concat!("$f", "20"), true)
    }
    .check_panic_chain();
    table[Cop1::fs0f as usize] = RegisterDescriptor {
        name_o64: Some("$fs1"),
        name_n32: Some("$ft8"),
        name_n64: Some("$ft9"),
        name_eabi32: Some("$fs0f"),
        name_eabi64: Some("$fs1"),
        ..RegisterDescriptor::new(concat!("$", "fs0f"), 21, concat!("$f", "21"), true)
    }
    .check_panic_chain();
    table[Cop1::fs1 as usize] = RegisterDescriptor {
        name_o64: Some("$fs2"),
        name_n32: Some("$fs1"),
        name_n64: Some("$ft10"),
        name_eabi32: Some("$fs1"),
        name_eabi64: Some("$fs2"),
        ..RegisterDescriptor::new(concat!("$", "fs1"), 22, concat!("$f", "22"), true)
    }
    .check_panic_chain();
    table[Cop1::fs1f as usize] = RegisterDescriptor {
        name_o64: Some("$fs3"),
        name_n32: Some("$ft9"),
        name_n64: Some("$ft11"),
        name_eabi32: Some("$fs1f"),
        name_eabi64: Some("$fs3"),
        ..RegisterDescriptor::new(concat!("$", "fs1f"), 23, concat!("$f", "23"), true)
    }
    .check_panic_chain();
    table[Cop1::fs2 as usize] = RegisterDescriptor {
        name_o64: Some("$fs4"),
        name_n32: Some("$fs2"),
        name_n64: Some("$fs0"),
        name_eabi32: Some("$fs2"),
        name_eabi64: Some("$fs4"),
        ..RegisterDescriptor::new(concat!("$", "fs2"), 24, concat!("$f", "24"), true)
    }
    .check_panic_chain();
    table[Cop1::fs2f as usize] = RegisterDescriptor {
        name_o64: Some("$fs5"),
        name_n32: Some("$ft10"),
        name_n64: Some("$fs1"),
        name_eabi32: Some("$fs2f"),
        name_eabi64: Some("$fs5"),
        ..RegisterDescriptor::new(concat!("$", "fs2f"), 25, concat!("$f", "25"), true)
    }
    .check_panic_chain();
    table[Cop1::fs3 as usize] = RegisterDescriptor {
        name_o64: Some("$fs6"),
        name_n32: Some("$fs3"),
        name_n64: Some("$fs2"),
        name_eabi32: Some("$fs3"),
        name_eabi64: Some("$fs6"),
        ..RegisterDescriptor::new(concat!("$", "fs3"), 26, concat!("$f", "26"), true)
    }
    .check_panic_chain();
    table[Cop1::fs3f as usize] = RegisterDescriptor {
        name_o64: Some("$fs7"),
        name_n32: Some("$ft11"),
        name_n64: Some("$fs3"),
        name_eabi32: Some("$fs3f"),
        name_eabi64: Some("$fs7"),
        ..RegisterDescriptor::new(concat!("$", "fs3f"), 27, concat!("$f", "27"), true)
    }
    .check_panic_chain();
    table[Cop1::fs4 as usize] = RegisterDescriptor {
        name_o64: Some("$fs8"),
        name_n32: Some("$fs4"),
        name_n64: Some("$fs4"),
        name_eabi32: Some("$fs4"),
        name_eabi64: Some("$fs8"),
        ..RegisterDescriptor::new(concat!("$", "fs4"), 28, concat!("$f", "28"), true)
    }
    .check_panic_chain();
    table[Cop1::fs4f as usize] = RegisterDescriptor {
        name_o64: Some("$fs9"),
        name_n32: Some("$ft12"),
        name_n64: Some("$fs5"),
        name_eabi32: Some("$fs4f"),
        name_eabi64: Some("$fs9"),
        ..RegisterDescriptor::new(concat!("$", "fs4f"), 29, concat!("$f", "29"), true)
    }
    .check_panic_chain();
    table[Cop1::fs5 as usize] = RegisterDescriptor {
        name_o64: Some("$fs10"),
        name_n32: Some("$fs5"),
        name_n64: Some("$fs6"),
        name_eabi32: Some("$fs5"),
        name_eabi64: Some("$fs10"),
        ..RegisterDescriptor::new(concat!("$", "fs5"), 30, concat!("$f", "30"), true)
    }
    .check_panic_chain();
    table[Cop1::fs5f as usize] = RegisterDescriptor {
        name_o64: Some("$fs11"),
        name_n32: Some("$ft13"),
        name_n64: Some("$fs7"),
        name_eabi32: Some("$fs5f"),
        name_eabi64: Some("$fs11"),
        ..RegisterDescriptor::new(concat!("$", "fs5f"), 31, concat!("$f", "31"), true)
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value() as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl Cop1 {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::fv0),
            1 => Ok(Self::fv0f),
            2 => Ok(Self::fv1),
            3 => Ok(Self::fv1f),
            4 => Ok(Self::ft0),
            5 => Ok(Self::ft0f),
            6 => Ok(Self::ft1),
            7 => Ok(Self::ft1f),
            8 => Ok(Self::ft2),
            9 => Ok(Self::ft2f),
            10 => Ok(Self::ft3),
            11 => Ok(Self::ft3f),
            12 => Ok(Self::fa0),
            13 => Ok(Self::fa0f),
            14 => Ok(Self::fa1),
            15 => Ok(Self::fa1f),
            16 => Ok(Self::ft4),
            17 => Ok(Self::ft4f),
            18 => Ok(Self::ft5),
            19 => Ok(Self::ft5f),
            20 => Ok(Self::fs0),
            21 => Ok(Self::fs0f),
            22 => Ok(Self::fs1),
            23 => Ok(Self::fs1f),
            24 => Ok(Self::fs2),
            25 => Ok(Self::fs2f),
            26 => Ok(Self::fs3),
            27 => Ok(Self::fs3f),
            28 => Ok(Self::fs4),
            29 => Ok(Self::fs4f),
            30 => Ok(Self::fs5),
            31 => Ok(Self::fs5f),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 32,
                register_kind: "Cop1",
            }),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        32
    }
}
impl TryFrom<u32> for Cop1 {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for Cop1 {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<Cop1> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: Cop1) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &COP1 {
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
