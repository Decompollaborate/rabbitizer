/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexVConstant {
    INVALID_0 = 0,
    VFPU_HUGE = 1,
    VFPU_SQRT2 = 2,
    VFPU_SQRT1_2 = 3,
    VFPU_2_SQRTPI = 4,
    VFPU_2_PI = 5,
    VFPU_1_PI = 6,
    VFPU_PI_4 = 7,
    VFPU_PI_2 = 8,
    VFPU_PI = 9,
    VFPU_E = 10,
    VFPU_LOG2E = 11,
    VFPU_LOG10E = 12,
    VFPU_LN2 = 13,
    VFPU_LN10 = 14,
    VFPU_2PI = 15,
    VFPU_PI_6 = 16,
    VFPU_LOG10TWO = 17,
    VFPU_LOG2TEN = 18,
    VFPU_SQRT3_2 = 19,
    INVALID_20 = 20,
    INVALID_21 = 21,
    INVALID_22 = 22,
    INVALID_23 = 23,
    INVALID_24 = 24,
    INVALID_25 = 25,
    INVALID_26 = 26,
    INVALID_27 = 27,
    INVALID_28 = 28,
    INVALID_29 = 29,
    INVALID_30 = 30,
    INVALID_31 = 31,
}
pub static R4000ALLEGREX_VCONSTANT: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[R4000AllegrexVConstant::INVALID_0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_0", 0, concat!("$", "0"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_HUGE as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_HUGE", 1, concat!("$", "1"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_SQRT2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_SQRT2", 2, concat!("$", "2"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_SQRT1_2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_SQRT1_2", 3, concat!("$", "3"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_2_SQRTPI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_2_SQRTPI", 4, concat!("$", "4"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_2_PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_2_PI", 5, concat!("$", "5"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_1_PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_1_PI", 6, concat!("$", "6"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_PI_4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI_4", 7, concat!("$", "7"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_PI_2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI_2", 8, concat!("$", "8"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI", 9, concat!("$", "9"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_E as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_E", 10, concat!("$", "10"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_LOG2E as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG2E", 11, concat!("$", "11"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_LOG10E as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG10E", 12, concat!("$", "12"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_LN2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LN2", 13, concat!("$", "13"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_LN10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LN10", 14, concat!("$", "14"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_2PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_2PI", 15, concat!("$", "15"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_PI_6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI_6", 16, concat!("$", "16"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_LOG10TWO as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG10TWO", 17, concat!("$", "17"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_LOG2TEN as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG2TEN", 18, concat!("$", "18"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::VFPU_SQRT3_2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_SQRT3_2", 19, concat!("$", "19"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_20", 20, concat!("$", "20"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_21", 21, concat!("$", "21"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_22", 22, concat!("$", "22"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_23", 23, concat!("$", "23"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_24", 24, concat!("$", "24"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_25", 25, concat!("$", "25"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_26", 26, concat!("$", "26"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_27", 27, concat!("$", "27"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_28", 28, concat!("$", "28"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_29", 29, concat!("$", "29"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_30", 30, concat!("$", "30"), false)
    }
    .check_panic_chain();
    table[R4000AllegrexVConstant::INVALID_31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_31", 31, concat!("$", "31"), false)
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value() as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexVConstant {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::INVALID_0),
            1 => Ok(Self::VFPU_HUGE),
            2 => Ok(Self::VFPU_SQRT2),
            3 => Ok(Self::VFPU_SQRT1_2),
            4 => Ok(Self::VFPU_2_SQRTPI),
            5 => Ok(Self::VFPU_2_PI),
            6 => Ok(Self::VFPU_1_PI),
            7 => Ok(Self::VFPU_PI_4),
            8 => Ok(Self::VFPU_PI_2),
            9 => Ok(Self::VFPU_PI),
            10 => Ok(Self::VFPU_E),
            11 => Ok(Self::VFPU_LOG2E),
            12 => Ok(Self::VFPU_LOG10E),
            13 => Ok(Self::VFPU_LN2),
            14 => Ok(Self::VFPU_LN10),
            15 => Ok(Self::VFPU_2PI),
            16 => Ok(Self::VFPU_PI_6),
            17 => Ok(Self::VFPU_LOG10TWO),
            18 => Ok(Self::VFPU_LOG2TEN),
            19 => Ok(Self::VFPU_SQRT3_2),
            20 => Ok(Self::INVALID_20),
            21 => Ok(Self::INVALID_21),
            22 => Ok(Self::INVALID_22),
            23 => Ok(Self::INVALID_23),
            24 => Ok(Self::INVALID_24),
            25 => Ok(Self::INVALID_25),
            26 => Ok(Self::INVALID_26),
            27 => Ok(Self::INVALID_27),
            28 => Ok(Self::INVALID_28),
            29 => Ok(Self::INVALID_29),
            30 => Ok(Self::INVALID_30),
            31 => Ok(Self::INVALID_31),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 32,
                register_kind: "R4000AllegrexVConstant",
            }),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        32
    }
}
impl TryFrom<u32> for R4000AllegrexVConstant {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexVConstant {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexVConstant> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexVConstant) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R4000ALLEGREX_VCONSTANT {
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
