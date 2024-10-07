/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexPrefixDst {
    none = 0,
    zero = 1,
    INVALID_2 = 2,
    one = 3,
    M = 4,
    INVALID_5 = 5,
    INVALID_6 = 6,
    INVALID_7 = 7,
}
pub static R4000ALLEGREX_PREFIX_DST: [RegisterDescriptor; 8] = {
    let mut table = [RegisterDescriptor::default(); 8];
    table[R4000AllegrexPrefixDst::none as usize] = RegisterDescriptor {
        name_numeric: "",
        name: "",
        ..RegisterDescriptor::new("none", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixDst::zero as usize] = RegisterDescriptor {
        name_numeric: "0",
        name: "0",
        ..RegisterDescriptor::new("zero", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixDst::INVALID_2 as usize] = RegisterDescriptor {
        name_numeric: "INVALID_2",
        ..RegisterDescriptor::new("INVALID_2", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixDst::one as usize] = RegisterDescriptor {
        name_numeric: "1",
        name: "1",
        ..RegisterDescriptor::new("one", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixDst::M as usize] = RegisterDescriptor {
        name_numeric: "M",
        ..RegisterDescriptor::new("M", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixDst::INVALID_5 as usize] = RegisterDescriptor {
        name_numeric: "INVALID_5",
        ..RegisterDescriptor::new("INVALID_5", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixDst::INVALID_6 as usize] = RegisterDescriptor {
        name_numeric: "INVALID_6",
        ..RegisterDescriptor::new("INVALID_6", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[R4000AllegrexPrefixDst::INVALID_7 as usize] = RegisterDescriptor {
        name_numeric: "INVALID_7",
        ..RegisterDescriptor::new("INVALID_7", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 8 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexPrefixDst {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::none),
            1 => Ok(Self::zero),
            2 => Ok(Self::INVALID_2),
            3 => Ok(Self::one),
            4 => Ok(Self::M),
            5 => Ok(Self::INVALID_5),
            6 => Ok(Self::INVALID_6),
            7 => Ok(Self::INVALID_7),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 8,
                register_kind: "R4000AllegrexPrefixDst",
            }),
        }
    }
}
impl TryFrom<u32> for R4000AllegrexPrefixDst {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexPrefixDst {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexPrefixDst> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexPrefixDst) -> &Self::Output {
        &self[index as usize]
    }
}
