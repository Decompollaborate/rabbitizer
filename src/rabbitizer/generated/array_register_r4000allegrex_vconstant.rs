/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{RegisterDescriptor, RegisterR4000AllegrexVConstant};
pub static R4000ALLEGREX_VCONSTANT_REGISTERS: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[RegisterR4000AllegrexVConstant::INVALID_0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_0", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_HUGE as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_HUGE", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_SQRT2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_SQRT2", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_SQRT1_2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_SQRT1_2", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_2_SQRTPI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_2_SQRTPI", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_2_PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_2_PI", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_1_PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_1_PI", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_PI_4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI_4", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_PI_2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI_2", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_E as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_E", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_LOG2E as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG2E", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_LOG10E as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG10E", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_LN2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LN2", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_LN10 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LN10", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_2PI as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_2PI", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_PI_6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PI_6", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_LOG10TWO as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG10TWO", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_LOG2TEN as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_LOG2TEN", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::VFPU_SQRT3_2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_SQRT3_2", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_20 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_20", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_21 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_21", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_22 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_22", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_23 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_23", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_24 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_24", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_25 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_25", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_26 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_26", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_27 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_27", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_28 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_28", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_29 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_29", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_30 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_30", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVConstant::INVALID_31 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("INVALID_31", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i);
        i += 1;
    }
    table
};
