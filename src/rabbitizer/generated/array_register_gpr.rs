/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{RegisterDescriptor, RegisterGpr};
pub static GPR_REGISTERS: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[RegisterGpr::zero as usize] = RegisterDescriptor {
        is_zero: true,
        ..RegisterDescriptor::new(concat!("$", "zero"), 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RegisterGpr::at as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_assembler_temp: true,
        ..RegisterDescriptor::new(concat!("$", "at"), 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RegisterGpr::v0 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        holds_return_value: true,
        ..RegisterDescriptor::new(concat!("$", "v0"), 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RegisterGpr::v1 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        holds_return_value: true,
        ..RegisterDescriptor::new(concat!("$", "v1"), 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RegisterGpr::a0 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a0"), 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RegisterGpr::a1 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a1"), 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RegisterGpr::a2 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a2"), 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RegisterGpr::a3 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_arg: true,
        ..RegisterDescriptor::new(concat!("$", "a3"), 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RegisterGpr::t0 as usize] = RegisterDescriptor {
        name_n32: Some("$a4"),
        name_n64: Some("$a4"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t0"), 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RegisterGpr::t1 as usize] = RegisterDescriptor {
        name_n32: Some("$a5"),
        name_n64: Some("$a5"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t1"), 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RegisterGpr::t2 as usize] = RegisterDescriptor {
        name_n32: Some("$a6"),
        name_n64: Some("$a6"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t2"), 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RegisterGpr::t3 as usize] = RegisterDescriptor {
        name_n32: Some("$a7"),
        name_n64: Some("$a7"),
        is_clobbered_by_func_call: true,
        is_temp: true,
        is_arg: true,
        either_arg_or_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t3"), 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RegisterGpr::t4 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t4"), 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RegisterGpr::t5 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t5"), 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RegisterGpr::t6 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t6"), 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RegisterGpr::t7 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t7"), 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RegisterGpr::s0 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s0"), 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RegisterGpr::s1 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s1"), 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RegisterGpr::s2 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s2"), 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RegisterGpr::s3 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s3"), 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RegisterGpr::s4 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s4"), 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RegisterGpr::s5 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s5"), 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RegisterGpr::s6 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s6"), 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RegisterGpr::s7 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s7"), 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RegisterGpr::t8 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t8"), 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RegisterGpr::t9 as usize] = RegisterDescriptor {
        is_clobbered_by_func_call: true,
        is_temp: true,
        ..RegisterDescriptor::new(concat!("$", "t9"), 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RegisterGpr::k0 as usize] = RegisterDescriptor {
        is_kernel: true,
        ..RegisterDescriptor::new(concat!("$", "k0"), 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RegisterGpr::k1 as usize] = RegisterDescriptor {
        is_kernel: true,
        ..RegisterDescriptor::new(concat!("$", "k1"), 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RegisterGpr::gp as usize] = RegisterDescriptor {
        is_global_pointer: true,
        ..RegisterDescriptor::new(concat!("$", "gp"), 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RegisterGpr::sp as usize] = RegisterDescriptor {
        is_stack_pointer: true,
        ..RegisterDescriptor::new(concat!("$", "sp"), 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RegisterGpr::s8 as usize] = RegisterDescriptor {
        is_saved: true,
        ..RegisterDescriptor::new(concat!("$", "s8"), 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RegisterGpr::ra as usize] = RegisterDescriptor {
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
