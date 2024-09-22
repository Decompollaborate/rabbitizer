/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RegisterCop1 {
    fv0,
    fv0f,
    fv1,
    fv1f,
    ft0,
    ft0f,
    ft1,
    ft1f,
    ft2,
    ft2f,
    ft3,
    ft3f,
    fa0,
    fa0f,
    fa1,
    fa1f,
    ft4,
    ft4f,
    ft5,
    ft5f,
    fs0,
    fs0f,
    fs1,
    fs1f,
    fs2,
    fs2f,
    fs3,
    fs3f,
    fs4,
    fs4f,
    fs5,
    fs5f,
}
pub static COP1_REGISTERS: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[RegisterCop1::fv0 as usize] = RegisterDescriptor {
        name_n32: Some("$fv0"),
        name_n64: Some("$fv0"),
        ..RegisterDescriptor::new(concat!("$", "fv0"), 0, concat!("$f", "0"))
    }
    .check_panic_chain();
    table[RegisterCop1::fv0f as usize] = RegisterDescriptor {
        name_n32: Some("$ft14"),
        name_n64: Some("$ft12"),
        ..RegisterDescriptor::new(concat!("$", "fv0f"), 1, concat!("$f", "1"))
    }
    .check_panic_chain();
    table[RegisterCop1::fv1 as usize] = RegisterDescriptor {
        name_n32: Some("$fv1"),
        name_n64: Some("$fv1"),
        ..RegisterDescriptor::new(concat!("$", "fv1"), 2, concat!("$f", "2"))
    }
    .check_panic_chain();
    table[RegisterCop1::fv1f as usize] = RegisterDescriptor {
        name_n32: Some("$ft15"),
        name_n64: Some("$ft13"),
        ..RegisterDescriptor::new(concat!("$", "fv1f"), 3, concat!("$f", "3"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft0 as usize] = RegisterDescriptor {
        name_n32: Some("$ft0"),
        name_n64: Some("$ft0"),
        ..RegisterDescriptor::new(concat!("$", "ft0"), 4, concat!("$f", "4"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft0f as usize] = RegisterDescriptor {
        name_n32: Some("$ft1"),
        name_n64: Some("$ft1"),
        ..RegisterDescriptor::new(concat!("$", "ft0f"), 5, concat!("$f", "5"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft1 as usize] = RegisterDescriptor {
        name_n32: Some("$ft2"),
        name_n64: Some("$ft2"),
        ..RegisterDescriptor::new(concat!("$", "ft1"), 6, concat!("$f", "6"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft1f as usize] = RegisterDescriptor {
        name_n32: Some("$ft3"),
        name_n64: Some("$ft3"),
        ..RegisterDescriptor::new(concat!("$", "ft1f"), 7, concat!("$f", "7"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft2 as usize] = RegisterDescriptor {
        name_n32: Some("$ft4"),
        name_n64: Some("$ft4"),
        ..RegisterDescriptor::new(concat!("$", "ft2"), 8, concat!("$f", "8"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft2f as usize] = RegisterDescriptor {
        name_n32: Some("$ft5"),
        name_n64: Some("$ft5"),
        ..RegisterDescriptor::new(concat!("$", "ft2f"), 9, concat!("$f", "9"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft3 as usize] = RegisterDescriptor {
        name_n32: Some("$ft6"),
        name_n64: Some("$ft6"),
        ..RegisterDescriptor::new(concat!("$", "ft3"), 10, concat!("$f", "10"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft3f as usize] = RegisterDescriptor {
        name_n32: Some("$ft7"),
        name_n64: Some("$ft7"),
        ..RegisterDescriptor::new(concat!("$", "ft3f"), 11, concat!("$f", "11"))
    }
    .check_panic_chain();
    table[RegisterCop1::fa0 as usize] = RegisterDescriptor {
        name_n32: Some("$fa0"),
        name_n64: Some("$fa0"),
        ..RegisterDescriptor::new(concat!("$", "fa0"), 12, concat!("$f", "12"))
    }
    .check_panic_chain();
    table[RegisterCop1::fa0f as usize] = RegisterDescriptor {
        name_n32: Some("$fa1"),
        name_n64: Some("$fa1"),
        ..RegisterDescriptor::new(concat!("$", "fa0f"), 13, concat!("$f", "13"))
    }
    .check_panic_chain();
    table[RegisterCop1::fa1 as usize] = RegisterDescriptor {
        name_n32: Some("$fa2"),
        name_n64: Some("$fa2"),
        ..RegisterDescriptor::new(concat!("$", "fa1"), 14, concat!("$f", "14"))
    }
    .check_panic_chain();
    table[RegisterCop1::fa1f as usize] = RegisterDescriptor {
        name_n32: Some("$fa3"),
        name_n64: Some("$fa3"),
        ..RegisterDescriptor::new(concat!("$", "fa1f"), 15, concat!("$f", "15"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft4 as usize] = RegisterDescriptor {
        name_n32: Some("$fa4"),
        name_n64: Some("$fa4"),
        ..RegisterDescriptor::new(concat!("$", "ft4"), 16, concat!("$f", "16"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft4f as usize] = RegisterDescriptor {
        name_n32: Some("$fa5"),
        name_n64: Some("$fa5"),
        ..RegisterDescriptor::new(concat!("$", "ft4f"), 17, concat!("$f", "17"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft5 as usize] = RegisterDescriptor {
        name_n32: Some("$fa6"),
        name_n64: Some("$fa6"),
        ..RegisterDescriptor::new(concat!("$", "ft5"), 18, concat!("$f", "18"))
    }
    .check_panic_chain();
    table[RegisterCop1::ft5f as usize] = RegisterDescriptor {
        name_n32: Some("$fa7"),
        name_n64: Some("$fa7"),
        ..RegisterDescriptor::new(concat!("$", "ft5f"), 19, concat!("$f", "19"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs0 as usize] = RegisterDescriptor {
        name_n32: Some("$fs0"),
        name_n64: Some("$ft8"),
        ..RegisterDescriptor::new(concat!("$", "fs0"), 20, concat!("$f", "20"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs0f as usize] = RegisterDescriptor {
        name_n32: Some("$ft8"),
        name_n64: Some("$ft9"),
        ..RegisterDescriptor::new(concat!("$", "fs0f"), 21, concat!("$f", "21"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs1 as usize] = RegisterDescriptor {
        name_n32: Some("$fs1"),
        name_n64: Some("$ft10"),
        ..RegisterDescriptor::new(concat!("$", "fs1"), 22, concat!("$f", "22"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs1f as usize] = RegisterDescriptor {
        name_n32: Some("$ft9"),
        name_n64: Some("$ft11"),
        ..RegisterDescriptor::new(concat!("$", "fs1f"), 23, concat!("$f", "23"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs2 as usize] = RegisterDescriptor {
        name_n32: Some("$fs2"),
        name_n64: Some("$fs0"),
        ..RegisterDescriptor::new(concat!("$", "fs2"), 24, concat!("$f", "24"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs2f as usize] = RegisterDescriptor {
        name_n32: Some("$ft10"),
        name_n64: Some("$fs1"),
        ..RegisterDescriptor::new(concat!("$", "fs2f"), 25, concat!("$f", "25"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs3 as usize] = RegisterDescriptor {
        name_n32: Some("$fs3"),
        name_n64: Some("$fs2"),
        ..RegisterDescriptor::new(concat!("$", "fs3"), 26, concat!("$f", "26"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs3f as usize] = RegisterDescriptor {
        name_n32: Some("$ft11"),
        name_n64: Some("$fs3"),
        ..RegisterDescriptor::new(concat!("$", "fs3f"), 27, concat!("$f", "27"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs4 as usize] = RegisterDescriptor {
        name_n32: Some("$fs4"),
        name_n64: Some("$fs4"),
        ..RegisterDescriptor::new(concat!("$", "fs4"), 28, concat!("$f", "28"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs4f as usize] = RegisterDescriptor {
        name_n32: Some("$ft12"),
        name_n64: Some("$fs5"),
        ..RegisterDescriptor::new(concat!("$", "fs4f"), 29, concat!("$f", "29"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs5 as usize] = RegisterDescriptor {
        name_n32: Some("$fs5"),
        name_n64: Some("$fs6"),
        ..RegisterDescriptor::new(concat!("$", "fs5"), 30, concat!("$f", "30"))
    }
    .check_panic_chain();
    table[RegisterCop1::fs5f as usize] = RegisterDescriptor {
        name_n32: Some("$ft13"),
        name_n64: Some("$fs7"),
        ..RegisterDescriptor::new(concat!("$", "fs5f"), 31, concat!("$f", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
