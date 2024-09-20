/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{RegisterCop0, RegisterDescriptor};
pub static COP0_REGISTERS: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[RegisterCop0::Index as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Index", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RegisterCop0::Random as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Random", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RegisterCop0::EntryLo0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EntryLo0", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RegisterCop0::EntryLo1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EntryLo1", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RegisterCop0::Context as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Context", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RegisterCop0::PageMask as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("PageMask", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RegisterCop0::Wired as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Wired", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RegisterCop0::Reserved07 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved07", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RegisterCop0::BadVaddr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("BadVaddr", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RegisterCop0::Count as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Count", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RegisterCop0::EntryHi as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EntryHi", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RegisterCop0::Compare as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Compare", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RegisterCop0::Status as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Status", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RegisterCop0::Cause as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Cause", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RegisterCop0::EPC as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EPC", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RegisterCop0::PRevID as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("PRevID", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RegisterCop0::Config as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Config", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RegisterCop0::LLAddr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("LLAddr", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RegisterCop0::WatchLo as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("WatchLo", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RegisterCop0::WatchHi as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("WatchHi", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RegisterCop0::XContext as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("XContext", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RegisterCop0::Reserved21 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved21", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RegisterCop0::Reserved22 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved22", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RegisterCop0::Reserved23 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved23", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RegisterCop0::Reserved24 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved24", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RegisterCop0::Reserved25 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved25", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RegisterCop0::PErr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("PErr", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RegisterCop0::CacheErr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("CacheErr", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RegisterCop0::TagLo as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("TagLo", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RegisterCop0::TagHi as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("TagHi", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RegisterCop0::ErrorEPC as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ErrorEPC", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RegisterCop0::Reserved31 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved31", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i);
        i += 1;
    }
    table
};
