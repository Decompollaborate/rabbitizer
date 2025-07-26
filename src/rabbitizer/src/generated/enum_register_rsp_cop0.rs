/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RspCop0 {
    SP_MEM_ADDR = 0,
    SP_DRAM_ADDR = 1,
    SP_RD_LEN = 2,
    SP_WR_LEN = 3,
    SP_STATUS = 4,
    SP_DMA_FULL = 5,
    SP_DMA_BUSY = 6,
    SP_SEMAPHORE = 7,
    DPC_START = 8,
    DPC_END = 9,
    DPC_CURRENT = 10,
    DPC_STATUS = 11,
    DPC_CLOCK = 12,
    DPC_BUFBUSY = 13,
    DPC_PIPEBUSY = 14,
    DPC_TMEM = 15,
}
pub static RSP_COP0: [RegisterDescriptor; 16] = {
    let mut table = [RegisterDescriptor::default(); 16];
    table[RspCop0::SP_MEM_ADDR as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_MEM_ADDR", 0, concat!("$", "0"), false)
    };
    table[RspCop0::SP_DRAM_ADDR as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_DRAM_ADDR", 1, concat!("$", "1"), false)
    };
    table[RspCop0::SP_RD_LEN as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_RD_LEN", 2, concat!("$", "2"), false)
    };
    table[RspCop0::SP_WR_LEN as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_WR_LEN", 3, concat!("$", "3"), false)
    };
    table[RspCop0::SP_STATUS as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_STATUS", 4, concat!("$", "4"), false)
    };
    table[RspCop0::SP_DMA_FULL as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_DMA_FULL", 5, concat!("$", "5"), false)
    };
    table[RspCop0::SP_DMA_BUSY as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_DMA_BUSY", 6, concat!("$", "6"), false)
    };
    table[RspCop0::SP_SEMAPHORE as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_SEMAPHORE", 7, concat!("$", "7"), false)
    };
    table[RspCop0::DPC_START as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_START", 8, concat!("$", "8"), false)
    };
    table[RspCop0::DPC_END as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_END", 9, concat!("$", "9"), false)
    };
    table[RspCop0::DPC_CURRENT as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_CURRENT", 10, concat!("$", "10"), false)
    };
    table[RspCop0::DPC_STATUS as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_STATUS", 11, concat!("$", "11"), false)
    };
    table[RspCop0::DPC_CLOCK as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_CLOCK", 12, concat!("$", "12"), false)
    };
    table[RspCop0::DPC_BUFBUSY as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_BUFBUSY", 13, concat!("$", "13"), false)
    };
    table[RspCop0::DPC_PIPEBUSY as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_PIPEBUSY", 14, concat!("$", "14"), false)
    };
    table[RspCop0::DPC_TMEM as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_TMEM", 15, concat!("$", "15"), false)
    };
    table
};
impl RspCop0 {
    pub const fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
        match value {
            0 => Ok(Self::SP_MEM_ADDR),
            1 => Ok(Self::SP_DRAM_ADDR),
            2 => Ok(Self::SP_RD_LEN),
            3 => Ok(Self::SP_WR_LEN),
            4 => Ok(Self::SP_STATUS),
            5 => Ok(Self::SP_DMA_FULL),
            6 => Ok(Self::SP_DMA_BUSY),
            7 => Ok(Self::SP_SEMAPHORE),
            8 => Ok(Self::DPC_START),
            9 => Ok(Self::DPC_END),
            10 => Ok(Self::DPC_CURRENT),
            11 => Ok(Self::DPC_STATUS),
            12 => Ok(Self::DPC_CLOCK),
            13 => Ok(Self::DPC_BUFBUSY),
            14 => Ok(Self::DPC_PIPEBUSY),
            15 => Ok(Self::DPC_TMEM),
            x => Err(IntRegisterConversionError::new_out_of_range(
                x, 16, "RspCop0",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        16
    }
}
impl TryFrom<u32> for RspCop0 {
    type Error = IntRegisterConversionError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for RspCop0 {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<RspCop0> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: RspCop0) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &RSP_COP0 {
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
    #[test]
    fn check_descriptor_valid() {
        for (i, x) in RSP_COP0.iter().enumerate() {
            assert!(x.value() as usize == i, "Broken register index?");
            x.check_valid_entry();
        }
    }
}
