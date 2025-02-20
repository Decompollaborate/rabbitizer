/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum Gpr {
    zero = 0,
    at = 1,
    v0 = 2,
    v1 = 3,
    a0 = 4,
    a1 = 5,
    a2 = 6,
    a3 = 7,
    t0 = 8,
    t1 = 9,
    t2 = 10,
    t3 = 11,
    t4 = 12,
    t5 = 13,
    t6 = 14,
    t7 = 15,
    s0 = 16,
    s1 = 17,
    s2 = 18,
    s3 = 19,
    s4 = 20,
    s5 = 21,
    s6 = 22,
    s7 = 23,
    t8 = 24,
    t9 = 25,
    k0 = 26,
    k1 = 27,
    gp = 28,
    sp = 29,
    s8 = 30,
    ra = 31,
}
pub static GPR: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[Gpr::zero as usize] = RegisterDescriptor {
        name_numeric: "$zero",
        is_zero: true,
        ..RegisterDescriptor::new(concat!("$", "zero"), 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[Gpr::at as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_assembler_temp: true,
        ..RegisterDescriptor::new(concat!("$", "at"), 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[Gpr::v0 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        holds_return_value: true,
        ..RegisterDescriptor::new(concat!("$", "v0"), 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[Gpr::v1 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        holds_return_value: true,
        ..RegisterDescriptor::new(concat!("$", "v1"), 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[Gpr::a0 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a0"), 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[Gpr::a1 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a1"), 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[Gpr::a2 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a2"), 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[Gpr::a3 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a3"), 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[Gpr::t0 as usize] = RegisterDescriptor {
        name_n32: Some("$a4"),
        name_n64: Some("$a4"),
        name_eabi32: Some("$a4"),
        name_eabi64: Some("$a4"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t0"), 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[Gpr::t1 as usize] = RegisterDescriptor {
        name_n32: Some("$a5"),
        name_n64: Some("$a5"),
        name_eabi32: Some("$a5"),
        name_eabi64: Some("$a5"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t1"), 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[Gpr::t2 as usize] = RegisterDescriptor {
        name_n32: Some("$a6"),
        name_n64: Some("$a6"),
        name_eabi32: Some("$a6"),
        name_eabi64: Some("$a6"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t2"), 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[Gpr::t3 as usize] = RegisterDescriptor {
        name_n32: Some("$a7"),
        name_n64: Some("$a7"),
        name_eabi32: Some("$a7"),
        name_eabi64: Some("$a7"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t3"), 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[Gpr::t4 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t4"), 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[Gpr::t5 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t5"), 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[Gpr::t6 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t6"), 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[Gpr::t7 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t7"), 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[Gpr::s0 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s0"), 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[Gpr::s1 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s1"), 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[Gpr::s2 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s2"), 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[Gpr::s3 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s3"), 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[Gpr::s4 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s4"), 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[Gpr::s5 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s5"), 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[Gpr::s6 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s6"), 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[Gpr::s7 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s7"), 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[Gpr::t8 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t8"), 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[Gpr::t9 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t9"), 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[Gpr::k0 as usize] = RegisterDescriptor {
        is_kernel: true,
        ..RegisterDescriptor::new(concat!("$", "k0"), 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[Gpr::k1 as usize] = RegisterDescriptor {
        is_kernel: true,
        ..RegisterDescriptor::new(concat!("$", "k1"), 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[Gpr::gp as usize] = RegisterDescriptor {
        is_global_pointer: true,
        ..RegisterDescriptor::new(concat!("$", "gp"), 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[Gpr::sp as usize] = RegisterDescriptor {
        is_stack_pointer: true,
        ..RegisterDescriptor::new(concat!("$", "sp"), 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[Gpr::s8 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s8"), 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[Gpr::ra as usize] = RegisterDescriptor {
        name_numeric: "$ra",
        is_clobbered_by_func_call: true,
        holds_return_address: true,
        ..RegisterDescriptor::new(concat!("$", "ra"), 31, concat!("$", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl Gpr {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::zero),
            1 => Ok(Self::at),
            2 => Ok(Self::v0),
            3 => Ok(Self::v1),
            4 => Ok(Self::a0),
            5 => Ok(Self::a1),
            6 => Ok(Self::a2),
            7 => Ok(Self::a3),
            8 => Ok(Self::t0),
            9 => Ok(Self::t1),
            10 => Ok(Self::t2),
            11 => Ok(Self::t3),
            12 => Ok(Self::t4),
            13 => Ok(Self::t5),
            14 => Ok(Self::t6),
            15 => Ok(Self::t7),
            16 => Ok(Self::s0),
            17 => Ok(Self::s1),
            18 => Ok(Self::s2),
            19 => Ok(Self::s3),
            20 => Ok(Self::s4),
            21 => Ok(Self::s5),
            22 => Ok(Self::s6),
            23 => Ok(Self::s7),
            24 => Ok(Self::t8),
            25 => Ok(Self::t9),
            26 => Ok(Self::k0),
            27 => Ok(Self::k1),
            28 => Ok(Self::gp),
            29 => Ok(Self::sp),
            30 => Ok(Self::s8),
            31 => Ok(Self::ra),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 32,
                register_kind: "Gpr",
            }),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        32
    }
}
impl TryFrom<u32> for Gpr {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for Gpr {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<Gpr> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: Gpr) -> &Self::Output {
        &self[index as usize]
    }
}
