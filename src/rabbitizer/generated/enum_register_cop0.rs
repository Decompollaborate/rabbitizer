/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum Cop0 {
    Index = 0,
    Random = 1,
    EntryLo0 = 2,
    EntryLo1 = 3,
    Context = 4,
    PageMask = 5,
    Wired = 6,
    Reserved07 = 7,
    BadVaddr = 8,
    Count = 9,
    EntryHi = 10,
    Compare = 11,
    Status = 12,
    Cause = 13,
    EPC = 14,
    PRevID = 15,
    Config = 16,
    LLAddr = 17,
    WatchLo = 18,
    WatchHi = 19,
    XContext = 20,
    Reserved21 = 21,
    Reserved22 = 22,
    Reserved23 = 23,
    Reserved24 = 24,
    Reserved25 = 25,
    PErr = 26,
    CacheErr = 27,
    TagLo = 28,
    TagHi = 29,
    ErrorEPC = 30,
    Reserved31 = 31,
}
pub static COP0: [RegisterDescriptor; 32] = {
    let mut table = [RegisterDescriptor::default(); 32];
    table[Cop0::Index as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Index", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[Cop0::Random as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Random", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[Cop0::EntryLo0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EntryLo0", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[Cop0::EntryLo1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EntryLo1", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[Cop0::Context as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Context", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[Cop0::PageMask as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("PageMask", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[Cop0::Wired as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Wired", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[Cop0::Reserved07 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved07", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[Cop0::BadVaddr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("BadVaddr", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[Cop0::Count as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Count", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[Cop0::EntryHi as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EntryHi", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[Cop0::Compare as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Compare", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[Cop0::Status as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Status", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[Cop0::Cause as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Cause", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[Cop0::EPC as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("EPC", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[Cop0::PRevID as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("PRevID", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[Cop0::Config as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("Config", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[Cop0::LLAddr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("LLAddr", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[Cop0::WatchLo as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("WatchLo", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[Cop0::WatchHi as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("WatchHi", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[Cop0::XContext as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("XContext", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[Cop0::Reserved21 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved21", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[Cop0::Reserved22 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved22", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[Cop0::Reserved23 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved23", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[Cop0::Reserved24 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved24", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[Cop0::Reserved25 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved25", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[Cop0::PErr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("PErr", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[Cop0::CacheErr as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("CacheErr", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[Cop0::TagLo as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("TagLo", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[Cop0::TagHi as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("TagHi", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[Cop0::ErrorEPC as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("ErrorEPC", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[Cop0::Reserved31 as usize] = RegisterDescriptor {
        is_reserved: true,
        ..RegisterDescriptor::new("Reserved31", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 32 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl Cop0 {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::Index),
            1 => Ok(Self::Random),
            2 => Ok(Self::EntryLo0),
            3 => Ok(Self::EntryLo1),
            4 => Ok(Self::Context),
            5 => Ok(Self::PageMask),
            6 => Ok(Self::Wired),
            7 => Ok(Self::Reserved07),
            8 => Ok(Self::BadVaddr),
            9 => Ok(Self::Count),
            10 => Ok(Self::EntryHi),
            11 => Ok(Self::Compare),
            12 => Ok(Self::Status),
            13 => Ok(Self::Cause),
            14 => Ok(Self::EPC),
            15 => Ok(Self::PRevID),
            16 => Ok(Self::Config),
            17 => Ok(Self::LLAddr),
            18 => Ok(Self::WatchLo),
            19 => Ok(Self::WatchHi),
            20 => Ok(Self::XContext),
            21 => Ok(Self::Reserved21),
            22 => Ok(Self::Reserved22),
            23 => Ok(Self::Reserved23),
            24 => Ok(Self::Reserved24),
            25 => Ok(Self::Reserved25),
            26 => Ok(Self::PErr),
            27 => Ok(Self::CacheErr),
            28 => Ok(Self::TagLo),
            29 => Ok(Self::TagHi),
            30 => Ok(Self::ErrorEPC),
            31 => Ok(Self::Reserved31),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 32,
                register_kind: "Cop0",
            }),
        }
    }
}
impl TryFrom<u32> for Cop0 {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for Cop0 {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<Cop0> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: Cop0) -> &Self::Output {
        &self[index as usize]
    }
}
