/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RegisterRspCop0 {
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
pub static RSP_COP0_REGISTERS: [RegisterDescriptor; 16] = {
    let mut table = [RegisterDescriptor::default(); 16];
    table[RegisterRspCop0::SP_MEM_ADDR as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_MEM_ADDR", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::SP_DRAM_ADDR as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_DRAM_ADDR", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::SP_RD_LEN as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_RD_LEN", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::SP_WR_LEN as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_WR_LEN", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::SP_STATUS as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_STATUS", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::SP_DMA_FULL as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_DMA_FULL", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::SP_DMA_BUSY as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_DMA_BUSY", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::SP_SEMAPHORE as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("SP_SEMAPHORE", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_START as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_START", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_END as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_END", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_CURRENT as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_CURRENT", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_STATUS as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_STATUS", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_CLOCK as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_CLOCK", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_BUFBUSY as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_BUFBUSY", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_PIPEBUSY as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_PIPEBUSY", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RegisterRspCop0::DPC_TMEM as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("DPC_TMEM", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 16 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
