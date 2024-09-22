/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RspCop2Control {
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
pub static RSP_COP2_CONTROL: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[RspCop2Control::r0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "0"), 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RspCop2Control::r1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "1"), 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RspCop2Control::r2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "2"), 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RspCop2Control::r3 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "3"), 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RspCop2Control::r4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "4"), 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RspCop2Control::r5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "5"), 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RspCop2Control::r6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "6"), 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RspCop2Control::r7 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "7"), 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RspCop2Control::r8 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "8"), 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RspCop2Control::r9 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "9"), 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RspCop2Control::r10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "10"), 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RspCop2Control::r11 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "11"), 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RspCop2Control::r12 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "12"), 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RspCop2Control::r13 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "13"), 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RspCop2Control::r14 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "14"), 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RspCop2Control::r15 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "15"), 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RspCop2Control::r16 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "16"), 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RspCop2Control::r17 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "17"), 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RspCop2Control::r18 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "18"), 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RspCop2Control::r19 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "19"), 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RspCop2Control::r20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "20"), 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RspCop2Control::r21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "21"), 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RspCop2Control::r22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "22"), 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RspCop2Control::r23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "23"), 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RspCop2Control::r24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "24"), 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RspCop2Control::r25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "25"), 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RspCop2Control::r26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "26"), 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RspCop2Control::r27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "27"), 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RspCop2Control::r28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "28"), 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RspCop2Control::r29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "29"), 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RspCop2Control::r30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "30"), 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RspCop2Control::r31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "31"), 31, concat!("$", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
