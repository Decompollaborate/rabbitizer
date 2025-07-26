/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
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
        ..RegisterDescriptor::new("fl", 0, "fl", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::eq as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("eq", 1, "eq", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::lt as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("lt", 2, "lt", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::le as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("le", 3, "le", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::tr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("tr", 4, "tr", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ne as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ne", 5, "ne", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ge as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ge", 6, "ge", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::gt as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("gt", 7, "gt", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ez as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ez", 8, "ez", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::en as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("en", 9, "en", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ei as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ei", 10, "ei", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::es as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("es", 11, "es", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::nz as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("nz", 12, "nz", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::nn as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("nn", 13, "nn", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ni as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ni", 14, "ni", false)
    }
    .check_panic_chain();
    table[R4000AllegrexVCond::ns as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ns", 15, "ns", false)
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 16 {
        assert!(table[i].value() as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexVCond {
    pub const fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
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
            x => Err(IntRegisterConversionError::new_out_of_range(
                x,
                16,
                "R4000AllegrexVCond",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        16
    }
}
impl TryFrom<u32> for R4000AllegrexVCond {
    type Error = IntRegisterConversionError;
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R4000ALLEGREX_VCOND {
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
