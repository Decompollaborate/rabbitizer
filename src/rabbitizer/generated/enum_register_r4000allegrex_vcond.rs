/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexVCond {
    fl = 0,
    eq = 1,
    lt = 2,
    le = 3,
    tr = 4,
    ne = 5,
    ge = 6,
    gt = 7,
    ez = 8,
    en = 9,
    ei = 10,
    es = 11,
    nz = 12,
    nn = 13,
    ni = 14,
    ns = 15,
}
pub static R4000ALLEGREX_VCOND: [RegisterDescriptor; 16] = {
    let mut table = [RegisterDescriptor::default(); 16];
    table[R4000AllegrexVCond::fl as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("fl", 0, "fl")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::eq as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("eq", 1, "eq")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::lt as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("lt", 2, "lt")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::le as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("le", 3, "le")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::tr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("tr", 4, "tr")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ne as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ne", 5, "ne")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ge as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ge", 6, "ge")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::gt as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("gt", 7, "gt")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ez as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ez", 8, "ez")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::en as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("en", 9, "en")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ei as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ei", 10, "ei")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::es as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("es", 11, "es")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::nz as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("nz", 12, "nz")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::nn as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("nn", 13, "nn")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ni as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ni", 14, "ni")
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ns as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ns", 15, "ns")
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 16 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexVCond {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::fl),
            1 => Ok(Self::eq),
            2 => Ok(Self::lt),
            3 => Ok(Self::le),
            4 => Ok(Self::tr),
            5 => Ok(Self::ne),
            6 => Ok(Self::ge),
            7 => Ok(Self::gt),
            8 => Ok(Self::ez),
            9 => Ok(Self::en),
            10 => Ok(Self::ei),
            11 => Ok(Self::es),
            12 => Ok(Self::nz),
            13 => Ok(Self::nn),
            14 => Ok(Self::ni),
            15 => Ok(Self::ns),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 16,
                register_kind: "R4000AllegrexVCond",
            }),
        }
    }
}
impl TryFrom<u32> for R4000AllegrexVCond {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexVCond {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexVCond> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexVCond) -> &Self::Output {
        &self[index as usize]
    }
}
