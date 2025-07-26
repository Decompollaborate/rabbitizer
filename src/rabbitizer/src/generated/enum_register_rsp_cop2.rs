/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RspCop2 {
    r0 = 0,
    r1 = 1,
    r2 = 2,
    r3 = 3,
    r4 = 4,
    r5 = 5,
    r6 = 6,
    r7 = 7,
    r8 = 8,
    r9 = 9,
    r10 = 10,
    r11 = 11,
    r12 = 12,
    r13 = 13,
    r14 = 14,
    r15 = 15,
    r16 = 16,
    r17 = 17,
    r18 = 18,
    r19 = 19,
    r20 = 20,
    r21 = 21,
    r22 = 22,
    r23 = 23,
    r24 = 24,
    r25 = 25,
    r26 = 26,
    r27 = 27,
    r28 = 28,
    r29 = 29,
    r30 = 30,
    r31 = 31,
}
pub static RSP_COP2: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[RspCop2::r0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "0"), 0, concat!("$", "0"), true)
    }
    .check_panic_chain();
    table[RspCop2::r1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "1"), 1, concat!("$", "1"), true)
    }
    .check_panic_chain();
    table[RspCop2::r2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "2"), 2, concat!("$", "2"), true)
    }
    .check_panic_chain();
    table[RspCop2::r3 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "3"), 3, concat!("$", "3"), true)
    }
    .check_panic_chain();
    table[RspCop2::r4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "4"), 4, concat!("$", "4"), true)
    }
    .check_panic_chain();
    table[RspCop2::r5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "5"), 5, concat!("$", "5"), true)
    }
    .check_panic_chain();
    table[RspCop2::r6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "6"), 6, concat!("$", "6"), true)
    }
    .check_panic_chain();
    table[RspCop2::r7 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "7"), 7, concat!("$", "7"), true)
    }
    .check_panic_chain();
    table[RspCop2::r8 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "8"), 8, concat!("$", "8"), true)
    }
    .check_panic_chain();
    table[RspCop2::r9 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "9"), 9, concat!("$", "9"), true)
    }
    .check_panic_chain();
    table[RspCop2::r10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "10"), 10, concat!("$", "10"), true)
    }
    .check_panic_chain();
    table[RspCop2::r11 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "11"), 11, concat!("$", "11"), true)
    }
    .check_panic_chain();
    table[RspCop2::r12 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "12"), 12, concat!("$", "12"), true)
    }
    .check_panic_chain();
    table[RspCop2::r13 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "13"), 13, concat!("$", "13"), true)
    }
    .check_panic_chain();
    table[RspCop2::r14 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "14"), 14, concat!("$", "14"), true)
    }
    .check_panic_chain();
    table[RspCop2::r15 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "15"), 15, concat!("$", "15"), true)
    }
    .check_panic_chain();
    table[RspCop2::r16 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "16"), 16, concat!("$", "16"), true)
    }
    .check_panic_chain();
    table[RspCop2::r17 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "17"), 17, concat!("$", "17"), true)
    }
    .check_panic_chain();
    table[RspCop2::r18 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "18"), 18, concat!("$", "18"), true)
    }
    .check_panic_chain();
    table[RspCop2::r19 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "19"), 19, concat!("$", "19"), true)
    }
    .check_panic_chain();
    table[RspCop2::r20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "20"), 20, concat!("$", "20"), true)
    }
    .check_panic_chain();
    table[RspCop2::r21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "21"), 21, concat!("$", "21"), true)
    }
    .check_panic_chain();
    table[RspCop2::r22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "22"), 22, concat!("$", "22"), true)
    }
    .check_panic_chain();
    table[RspCop2::r23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "23"), 23, concat!("$", "23"), true)
    }
    .check_panic_chain();
    table[RspCop2::r24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "24"), 24, concat!("$", "24"), true)
    }
    .check_panic_chain();
    table[RspCop2::r25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "25"), 25, concat!("$", "25"), true)
    }
    .check_panic_chain();
    table[RspCop2::r26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "26"), 26, concat!("$", "26"), true)
    }
    .check_panic_chain();
    table[RspCop2::r27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "27"), 27, concat!("$", "27"), true)
    }
    .check_panic_chain();
    table[RspCop2::r28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "28"), 28, concat!("$", "28"), true)
    }
    .check_panic_chain();
    table[RspCop2::r29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "29"), 29, concat!("$", "29"), true)
    }
    .check_panic_chain();
    table[RspCop2::r30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "30"), 30, concat!("$", "30"), true)
    }
    .check_panic_chain();
    table[RspCop2::r31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "31"), 31, concat!("$", "31"), true)
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value() as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl RspCop2 {
    pub const fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
        match value {
            0 => Ok(Self::r0),
            1 => Ok(Self::r1),
            2 => Ok(Self::r2),
            3 => Ok(Self::r3),
            4 => Ok(Self::r4),
            5 => Ok(Self::r5),
            6 => Ok(Self::r6),
            7 => Ok(Self::r7),
            8 => Ok(Self::r8),
            9 => Ok(Self::r9),
            10 => Ok(Self::r10),
            11 => Ok(Self::r11),
            12 => Ok(Self::r12),
            13 => Ok(Self::r13),
            14 => Ok(Self::r14),
            15 => Ok(Self::r15),
            16 => Ok(Self::r16),
            17 => Ok(Self::r17),
            18 => Ok(Self::r18),
            19 => Ok(Self::r19),
            20 => Ok(Self::r20),
            21 => Ok(Self::r21),
            22 => Ok(Self::r22),
            23 => Ok(Self::r23),
            24 => Ok(Self::r24),
            25 => Ok(Self::r25),
            26 => Ok(Self::r26),
            27 => Ok(Self::r27),
            28 => Ok(Self::r28),
            29 => Ok(Self::r29),
            30 => Ok(Self::r30),
            31 => Ok(Self::r31),
            x => Err(IntRegisterConversionError::new_out_of_range(
                x, 32, "RspCop2",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        32
    }
}
impl TryFrom<u32> for RspCop2 {
    type Error = IntRegisterConversionError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for RspCop2 {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<RspCop2> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: RspCop2) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &RSP_COP2 {
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
