/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RspVector {
    v0 = 0,
    v1 = 1,
    v2 = 2,
    v3 = 3,
    v4 = 4,
    v5 = 5,
    v6 = 6,
    v7 = 7,
    v8 = 8,
    v9 = 9,
    v10 = 10,
    v11 = 11,
    v12 = 12,
    v13 = 13,
    v14 = 14,
    v15 = 15,
    v16 = 16,
    v17 = 17,
    v18 = 18,
    v19 = 19,
    v20 = 20,
    v21 = 21,
    v22 = 22,
    v23 = 23,
    v24 = 24,
    v25 = 25,
    v26 = 26,
    v27 = 27,
    v28 = 28,
    v29 = 29,
    v30 = 30,
    v31 = 31,
}
pub static RSP_VECTOR: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[RspVector::v0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v0"), 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RspVector::v1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v1"), 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RspVector::v2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v2"), 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RspVector::v3 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v3"), 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RspVector::v4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v4"), 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RspVector::v5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v5"), 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RspVector::v6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v6"), 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RspVector::v7 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v7"), 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RspVector::v8 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v8"), 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RspVector::v9 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v9"), 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RspVector::v10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v10"), 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RspVector::v11 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v11"), 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RspVector::v12 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v12"), 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RspVector::v13 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v13"), 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RspVector::v14 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v14"), 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RspVector::v15 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v15"), 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RspVector::v16 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v16"), 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RspVector::v17 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v17"), 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RspVector::v18 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v18"), 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RspVector::v19 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v19"), 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RspVector::v20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v20"), 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RspVector::v21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v21"), 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RspVector::v22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v22"), 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RspVector::v23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v23"), 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RspVector::v24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v24"), 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RspVector::v25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v25"), 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RspVector::v26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v26"), 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RspVector::v27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v27"), 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RspVector::v28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v28"), 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RspVector::v29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v29"), 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RspVector::v30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v30"), 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RspVector::v31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "v31"), 31, concat!("$", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
