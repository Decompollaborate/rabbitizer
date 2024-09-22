/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R5900VF {
    vf0 = 0,
    vf1 = 1,
    vf2 = 2,
    vf3 = 3,
    vf4 = 4,
    vf5 = 5,
    vf6 = 6,
    vf7 = 7,
    vf8 = 8,
    vf9 = 9,
    vf10 = 10,
    vf11 = 11,
    vf12 = 12,
    vf13 = 13,
    vf14 = 14,
    vf15 = 15,
    vf16 = 16,
    vf17 = 17,
    vf18 = 18,
    vf19 = 19,
    vf20 = 20,
    vf21 = 21,
    vf22 = 22,
    vf23 = 23,
    vf24 = 24,
    vf25 = 25,
    vf26 = 26,
    vf27 = 27,
    vf28 = 28,
    vf29 = 29,
    vf30 = 30,
    vf31 = 31,
}
pub static R5900_VF: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[R5900VF::vf0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf0"), 0, concat!("$vf", "0"))
    }
    .check_panic_chain();
    table[R5900VF::vf1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf1"), 1, concat!("$vf", "1"))
    }
    .check_panic_chain();
    table[R5900VF::vf2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf2"), 2, concat!("$vf", "2"))
    }
    .check_panic_chain();
    table[R5900VF::vf3 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf3"), 3, concat!("$vf", "3"))
    }
    .check_panic_chain();
    table[R5900VF::vf4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf4"), 4, concat!("$vf", "4"))
    }
    .check_panic_chain();
    table[R5900VF::vf5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf5"), 5, concat!("$vf", "5"))
    }
    .check_panic_chain();
    table[R5900VF::vf6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf6"), 6, concat!("$vf", "6"))
    }
    .check_panic_chain();
    table[R5900VF::vf7 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf7"), 7, concat!("$vf", "7"))
    }
    .check_panic_chain();
    table[R5900VF::vf8 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf8"), 8, concat!("$vf", "8"))
    }
    .check_panic_chain();
    table[R5900VF::vf9 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf9"), 9, concat!("$vf", "9"))
    }
    .check_panic_chain();
    table[R5900VF::vf10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf10"), 10, concat!("$vf", "10"))
    }
    .check_panic_chain();
    table[R5900VF::vf11 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf11"), 11, concat!("$vf", "11"))
    }
    .check_panic_chain();
    table[R5900VF::vf12 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf12"), 12, concat!("$vf", "12"))
    }
    .check_panic_chain();
    table[R5900VF::vf13 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf13"), 13, concat!("$vf", "13"))
    }
    .check_panic_chain();
    table[R5900VF::vf14 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf14"), 14, concat!("$vf", "14"))
    }
    .check_panic_chain();
    table[R5900VF::vf15 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf15"), 15, concat!("$vf", "15"))
    }
    .check_panic_chain();
    table[R5900VF::vf16 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf16"), 16, concat!("$vf", "16"))
    }
    .check_panic_chain();
    table[R5900VF::vf17 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf17"), 17, concat!("$vf", "17"))
    }
    .check_panic_chain();
    table[R5900VF::vf18 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf18"), 18, concat!("$vf", "18"))
    }
    .check_panic_chain();
    table[R5900VF::vf19 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf19"), 19, concat!("$vf", "19"))
    }
    .check_panic_chain();
    table[R5900VF::vf20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf20"), 20, concat!("$vf", "20"))
    }
    .check_panic_chain();
    table[R5900VF::vf21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf21"), 21, concat!("$vf", "21"))
    }
    .check_panic_chain();
    table[R5900VF::vf22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf22"), 22, concat!("$vf", "22"))
    }
    .check_panic_chain();
    table[R5900VF::vf23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf23"), 23, concat!("$vf", "23"))
    }
    .check_panic_chain();
    table[R5900VF::vf24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf24"), 24, concat!("$vf", "24"))
    }
    .check_panic_chain();
    table[R5900VF::vf25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf25"), 25, concat!("$vf", "25"))
    }
    .check_panic_chain();
    table[R5900VF::vf26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf26"), 26, concat!("$vf", "26"))
    }
    .check_panic_chain();
    table[R5900VF::vf27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf27"), 27, concat!("$vf", "27"))
    }
    .check_panic_chain();
    table[R5900VF::vf28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf28"), 28, concat!("$vf", "28"))
    }
    .check_panic_chain();
    table[R5900VF::vf29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf29"), 29, concat!("$vf", "29"))
    }
    .check_panic_chain();
    table[R5900VF::vf30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf30"), 30, concat!("$vf", "30"))
    }
    .check_panic_chain();
    table[R5900VF::vf31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "vf31"), 31, concat!("$vf", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
