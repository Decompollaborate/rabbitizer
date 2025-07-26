/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexS {
    S000 = 0,
    S010 = 1,
    S020 = 2,
    S030 = 3,
    S100 = 4,
    S110 = 5,
    S120 = 6,
    S130 = 7,
    S200 = 8,
    S210 = 9,
    S220 = 10,
    S230 = 11,
    S300 = 12,
    S310 = 13,
    S320 = 14,
    S330 = 15,
    S400 = 16,
    S410 = 17,
    S420 = 18,
    S430 = 19,
    S500 = 20,
    S510 = 21,
    S520 = 22,
    S530 = 23,
    S600 = 24,
    S610 = 25,
    S620 = 26,
    S630 = 27,
    S700 = 28,
    S710 = 29,
    S720 = 30,
    S730 = 31,
    S001 = 32,
    S011 = 33,
    S021 = 34,
    S031 = 35,
    S101 = 36,
    S111 = 37,
    S121 = 38,
    S131 = 39,
    S201 = 40,
    S211 = 41,
    S221 = 42,
    S231 = 43,
    S301 = 44,
    S311 = 45,
    S321 = 46,
    S331 = 47,
    S401 = 48,
    S411 = 49,
    S421 = 50,
    S431 = 51,
    S501 = 52,
    S511 = 53,
    S521 = 54,
    S531 = 55,
    S601 = 56,
    S611 = 57,
    S621 = 58,
    S631 = 59,
    S701 = 60,
    S711 = 61,
    S721 = 62,
    S731 = 63,
    S002 = 64,
    S012 = 65,
    S022 = 66,
    S032 = 67,
    S102 = 68,
    S112 = 69,
    S122 = 70,
    S132 = 71,
    S202 = 72,
    S212 = 73,
    S222 = 74,
    S232 = 75,
    S302 = 76,
    S312 = 77,
    S322 = 78,
    S332 = 79,
    S402 = 80,
    S412 = 81,
    S422 = 82,
    S432 = 83,
    S502 = 84,
    S512 = 85,
    S522 = 86,
    S532 = 87,
    S602 = 88,
    S612 = 89,
    S622 = 90,
    S632 = 91,
    S702 = 92,
    S712 = 93,
    S722 = 94,
    S732 = 95,
    S003 = 96,
    S013 = 97,
    S023 = 98,
    S033 = 99,
    S103 = 100,
    S113 = 101,
    S123 = 102,
    S133 = 103,
    S203 = 104,
    S213 = 105,
    S223 = 106,
    S233 = 107,
    S303 = 108,
    S313 = 109,
    S323 = 110,
    S333 = 111,
    S403 = 112,
    S413 = 113,
    S423 = 114,
    S433 = 115,
    S503 = 116,
    S513 = 117,
    S523 = 118,
    S533 = 119,
    S603 = 120,
    S613 = 121,
    S623 = 122,
    S633 = 123,
    S703 = 124,
    S713 = 125,
    S723 = 126,
    S733 = 127,
}
pub static R4000ALLEGREX_S: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[R4000AllegrexS::S000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S000", 0, concat!("$", "0"), false)
    };
    table[R4000AllegrexS::S010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S010", 1, concat!("$", "1"), false)
    };
    table[R4000AllegrexS::S020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S020", 2, concat!("$", "2"), false)
    };
    table[R4000AllegrexS::S030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S030", 3, concat!("$", "3"), false)
    };
    table[R4000AllegrexS::S100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S100", 4, concat!("$", "4"), false)
    };
    table[R4000AllegrexS::S110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S110", 5, concat!("$", "5"), false)
    };
    table[R4000AllegrexS::S120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S120", 6, concat!("$", "6"), false)
    };
    table[R4000AllegrexS::S130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S130", 7, concat!("$", "7"), false)
    };
    table[R4000AllegrexS::S200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S200", 8, concat!("$", "8"), false)
    };
    table[R4000AllegrexS::S210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S210", 9, concat!("$", "9"), false)
    };
    table[R4000AllegrexS::S220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S220", 10, concat!("$", "10"), false)
    };
    table[R4000AllegrexS::S230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S230", 11, concat!("$", "11"), false)
    };
    table[R4000AllegrexS::S300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S300", 12, concat!("$", "12"), false)
    };
    table[R4000AllegrexS::S310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S310", 13, concat!("$", "13"), false)
    };
    table[R4000AllegrexS::S320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S320", 14, concat!("$", "14"), false)
    };
    table[R4000AllegrexS::S330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S330", 15, concat!("$", "15"), false)
    };
    table[R4000AllegrexS::S400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S400", 16, concat!("$", "16"), false)
    };
    table[R4000AllegrexS::S410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S410", 17, concat!("$", "17"), false)
    };
    table[R4000AllegrexS::S420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S420", 18, concat!("$", "18"), false)
    };
    table[R4000AllegrexS::S430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S430", 19, concat!("$", "19"), false)
    };
    table[R4000AllegrexS::S500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S500", 20, concat!("$", "20"), false)
    };
    table[R4000AllegrexS::S510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S510", 21, concat!("$", "21"), false)
    };
    table[R4000AllegrexS::S520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S520", 22, concat!("$", "22"), false)
    };
    table[R4000AllegrexS::S530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S530", 23, concat!("$", "23"), false)
    };
    table[R4000AllegrexS::S600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S600", 24, concat!("$", "24"), false)
    };
    table[R4000AllegrexS::S610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S610", 25, concat!("$", "25"), false)
    };
    table[R4000AllegrexS::S620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S620", 26, concat!("$", "26"), false)
    };
    table[R4000AllegrexS::S630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S630", 27, concat!("$", "27"), false)
    };
    table[R4000AllegrexS::S700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S700", 28, concat!("$", "28"), false)
    };
    table[R4000AllegrexS::S710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S710", 29, concat!("$", "29"), false)
    };
    table[R4000AllegrexS::S720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S720", 30, concat!("$", "30"), false)
    };
    table[R4000AllegrexS::S730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S730", 31, concat!("$", "31"), false)
    };
    table[R4000AllegrexS::S001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S001", 32, concat!("$", "32"), false)
    };
    table[R4000AllegrexS::S011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S011", 33, concat!("$", "33"), false)
    };
    table[R4000AllegrexS::S021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S021", 34, concat!("$", "34"), false)
    };
    table[R4000AllegrexS::S031 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S031", 35, concat!("$", "35"), false)
    };
    table[R4000AllegrexS::S101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S101", 36, concat!("$", "36"), false)
    };
    table[R4000AllegrexS::S111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S111", 37, concat!("$", "37"), false)
    };
    table[R4000AllegrexS::S121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S121", 38, concat!("$", "38"), false)
    };
    table[R4000AllegrexS::S131 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S131", 39, concat!("$", "39"), false)
    };
    table[R4000AllegrexS::S201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S201", 40, concat!("$", "40"), false)
    };
    table[R4000AllegrexS::S211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S211", 41, concat!("$", "41"), false)
    };
    table[R4000AllegrexS::S221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S221", 42, concat!("$", "42"), false)
    };
    table[R4000AllegrexS::S231 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S231", 43, concat!("$", "43"), false)
    };
    table[R4000AllegrexS::S301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S301", 44, concat!("$", "44"), false)
    };
    table[R4000AllegrexS::S311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S311", 45, concat!("$", "45"), false)
    };
    table[R4000AllegrexS::S321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S321", 46, concat!("$", "46"), false)
    };
    table[R4000AllegrexS::S331 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S331", 47, concat!("$", "47"), false)
    };
    table[R4000AllegrexS::S401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S401", 48, concat!("$", "48"), false)
    };
    table[R4000AllegrexS::S411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S411", 49, concat!("$", "49"), false)
    };
    table[R4000AllegrexS::S421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S421", 50, concat!("$", "50"), false)
    };
    table[R4000AllegrexS::S431 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S431", 51, concat!("$", "51"), false)
    };
    table[R4000AllegrexS::S501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S501", 52, concat!("$", "52"), false)
    };
    table[R4000AllegrexS::S511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S511", 53, concat!("$", "53"), false)
    };
    table[R4000AllegrexS::S521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S521", 54, concat!("$", "54"), false)
    };
    table[R4000AllegrexS::S531 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S531", 55, concat!("$", "55"), false)
    };
    table[R4000AllegrexS::S601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S601", 56, concat!("$", "56"), false)
    };
    table[R4000AllegrexS::S611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S611", 57, concat!("$", "57"), false)
    };
    table[R4000AllegrexS::S621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S621", 58, concat!("$", "58"), false)
    };
    table[R4000AllegrexS::S631 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S631", 59, concat!("$", "59"), false)
    };
    table[R4000AllegrexS::S701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S701", 60, concat!("$", "60"), false)
    };
    table[R4000AllegrexS::S711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S711", 61, concat!("$", "61"), false)
    };
    table[R4000AllegrexS::S721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S721", 62, concat!("$", "62"), false)
    };
    table[R4000AllegrexS::S731 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S731", 63, concat!("$", "63"), false)
    };
    table[R4000AllegrexS::S002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S002", 64, concat!("$", "64"), false)
    };
    table[R4000AllegrexS::S012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S012", 65, concat!("$", "65"), false)
    };
    table[R4000AllegrexS::S022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S022", 66, concat!("$", "66"), false)
    };
    table[R4000AllegrexS::S032 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S032", 67, concat!("$", "67"), false)
    };
    table[R4000AllegrexS::S102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S102", 68, concat!("$", "68"), false)
    };
    table[R4000AllegrexS::S112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S112", 69, concat!("$", "69"), false)
    };
    table[R4000AllegrexS::S122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S122", 70, concat!("$", "70"), false)
    };
    table[R4000AllegrexS::S132 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S132", 71, concat!("$", "71"), false)
    };
    table[R4000AllegrexS::S202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S202", 72, concat!("$", "72"), false)
    };
    table[R4000AllegrexS::S212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S212", 73, concat!("$", "73"), false)
    };
    table[R4000AllegrexS::S222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S222", 74, concat!("$", "74"), false)
    };
    table[R4000AllegrexS::S232 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S232", 75, concat!("$", "75"), false)
    };
    table[R4000AllegrexS::S302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S302", 76, concat!("$", "76"), false)
    };
    table[R4000AllegrexS::S312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S312", 77, concat!("$", "77"), false)
    };
    table[R4000AllegrexS::S322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S322", 78, concat!("$", "78"), false)
    };
    table[R4000AllegrexS::S332 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S332", 79, concat!("$", "79"), false)
    };
    table[R4000AllegrexS::S402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S402", 80, concat!("$", "80"), false)
    };
    table[R4000AllegrexS::S412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S412", 81, concat!("$", "81"), false)
    };
    table[R4000AllegrexS::S422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S422", 82, concat!("$", "82"), false)
    };
    table[R4000AllegrexS::S432 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S432", 83, concat!("$", "83"), false)
    };
    table[R4000AllegrexS::S502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S502", 84, concat!("$", "84"), false)
    };
    table[R4000AllegrexS::S512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S512", 85, concat!("$", "85"), false)
    };
    table[R4000AllegrexS::S522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S522", 86, concat!("$", "86"), false)
    };
    table[R4000AllegrexS::S532 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S532", 87, concat!("$", "87"), false)
    };
    table[R4000AllegrexS::S602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S602", 88, concat!("$", "88"), false)
    };
    table[R4000AllegrexS::S612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S612", 89, concat!("$", "89"), false)
    };
    table[R4000AllegrexS::S622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S622", 90, concat!("$", "90"), false)
    };
    table[R4000AllegrexS::S632 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S632", 91, concat!("$", "91"), false)
    };
    table[R4000AllegrexS::S702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S702", 92, concat!("$", "92"), false)
    };
    table[R4000AllegrexS::S712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S712", 93, concat!("$", "93"), false)
    };
    table[R4000AllegrexS::S722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S722", 94, concat!("$", "94"), false)
    };
    table[R4000AllegrexS::S732 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S732", 95, concat!("$", "95"), false)
    };
    table[R4000AllegrexS::S003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S003", 96, concat!("$", "96"), false)
    };
    table[R4000AllegrexS::S013 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S013", 97, concat!("$", "97"), false)
    };
    table[R4000AllegrexS::S023 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S023", 98, concat!("$", "98"), false)
    };
    table[R4000AllegrexS::S033 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S033", 99, concat!("$", "99"), false)
    };
    table[R4000AllegrexS::S103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S103", 100, concat!("$", "100"), false)
    };
    table[R4000AllegrexS::S113 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S113", 101, concat!("$", "101"), false)
    };
    table[R4000AllegrexS::S123 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S123", 102, concat!("$", "102"), false)
    };
    table[R4000AllegrexS::S133 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S133", 103, concat!("$", "103"), false)
    };
    table[R4000AllegrexS::S203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S203", 104, concat!("$", "104"), false)
    };
    table[R4000AllegrexS::S213 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S213", 105, concat!("$", "105"), false)
    };
    table[R4000AllegrexS::S223 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S223", 106, concat!("$", "106"), false)
    };
    table[R4000AllegrexS::S233 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S233", 107, concat!("$", "107"), false)
    };
    table[R4000AllegrexS::S303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S303", 108, concat!("$", "108"), false)
    };
    table[R4000AllegrexS::S313 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S313", 109, concat!("$", "109"), false)
    };
    table[R4000AllegrexS::S323 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S323", 110, concat!("$", "110"), false)
    };
    table[R4000AllegrexS::S333 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S333", 111, concat!("$", "111"), false)
    };
    table[R4000AllegrexS::S403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S403", 112, concat!("$", "112"), false)
    };
    table[R4000AllegrexS::S413 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S413", 113, concat!("$", "113"), false)
    };
    table[R4000AllegrexS::S423 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S423", 114, concat!("$", "114"), false)
    };
    table[R4000AllegrexS::S433 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S433", 115, concat!("$", "115"), false)
    };
    table[R4000AllegrexS::S503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S503", 116, concat!("$", "116"), false)
    };
    table[R4000AllegrexS::S513 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S513", 117, concat!("$", "117"), false)
    };
    table[R4000AllegrexS::S523 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S523", 118, concat!("$", "118"), false)
    };
    table[R4000AllegrexS::S533 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S533", 119, concat!("$", "119"), false)
    };
    table[R4000AllegrexS::S603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S603", 120, concat!("$", "120"), false)
    };
    table[R4000AllegrexS::S613 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S613", 121, concat!("$", "121"), false)
    };
    table[R4000AllegrexS::S623 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S623", 122, concat!("$", "122"), false)
    };
    table[R4000AllegrexS::S633 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S633", 123, concat!("$", "123"), false)
    };
    table[R4000AllegrexS::S703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S703", 124, concat!("$", "124"), false)
    };
    table[R4000AllegrexS::S713 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S713", 125, concat!("$", "125"), false)
    };
    table[R4000AllegrexS::S723 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S723", 126, concat!("$", "126"), false)
    };
    table[R4000AllegrexS::S733 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S733", 127, concat!("$", "127"), false)
    };
    table
};
impl R4000AllegrexS {
    pub const fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
        match value {
            0 => Ok(Self::S000),
            1 => Ok(Self::S010),
            2 => Ok(Self::S020),
            3 => Ok(Self::S030),
            4 => Ok(Self::S100),
            5 => Ok(Self::S110),
            6 => Ok(Self::S120),
            7 => Ok(Self::S130),
            8 => Ok(Self::S200),
            9 => Ok(Self::S210),
            10 => Ok(Self::S220),
            11 => Ok(Self::S230),
            12 => Ok(Self::S300),
            13 => Ok(Self::S310),
            14 => Ok(Self::S320),
            15 => Ok(Self::S330),
            16 => Ok(Self::S400),
            17 => Ok(Self::S410),
            18 => Ok(Self::S420),
            19 => Ok(Self::S430),
            20 => Ok(Self::S500),
            21 => Ok(Self::S510),
            22 => Ok(Self::S520),
            23 => Ok(Self::S530),
            24 => Ok(Self::S600),
            25 => Ok(Self::S610),
            26 => Ok(Self::S620),
            27 => Ok(Self::S630),
            28 => Ok(Self::S700),
            29 => Ok(Self::S710),
            30 => Ok(Self::S720),
            31 => Ok(Self::S730),
            32 => Ok(Self::S001),
            33 => Ok(Self::S011),
            34 => Ok(Self::S021),
            35 => Ok(Self::S031),
            36 => Ok(Self::S101),
            37 => Ok(Self::S111),
            38 => Ok(Self::S121),
            39 => Ok(Self::S131),
            40 => Ok(Self::S201),
            41 => Ok(Self::S211),
            42 => Ok(Self::S221),
            43 => Ok(Self::S231),
            44 => Ok(Self::S301),
            45 => Ok(Self::S311),
            46 => Ok(Self::S321),
            47 => Ok(Self::S331),
            48 => Ok(Self::S401),
            49 => Ok(Self::S411),
            50 => Ok(Self::S421),
            51 => Ok(Self::S431),
            52 => Ok(Self::S501),
            53 => Ok(Self::S511),
            54 => Ok(Self::S521),
            55 => Ok(Self::S531),
            56 => Ok(Self::S601),
            57 => Ok(Self::S611),
            58 => Ok(Self::S621),
            59 => Ok(Self::S631),
            60 => Ok(Self::S701),
            61 => Ok(Self::S711),
            62 => Ok(Self::S721),
            63 => Ok(Self::S731),
            64 => Ok(Self::S002),
            65 => Ok(Self::S012),
            66 => Ok(Self::S022),
            67 => Ok(Self::S032),
            68 => Ok(Self::S102),
            69 => Ok(Self::S112),
            70 => Ok(Self::S122),
            71 => Ok(Self::S132),
            72 => Ok(Self::S202),
            73 => Ok(Self::S212),
            74 => Ok(Self::S222),
            75 => Ok(Self::S232),
            76 => Ok(Self::S302),
            77 => Ok(Self::S312),
            78 => Ok(Self::S322),
            79 => Ok(Self::S332),
            80 => Ok(Self::S402),
            81 => Ok(Self::S412),
            82 => Ok(Self::S422),
            83 => Ok(Self::S432),
            84 => Ok(Self::S502),
            85 => Ok(Self::S512),
            86 => Ok(Self::S522),
            87 => Ok(Self::S532),
            88 => Ok(Self::S602),
            89 => Ok(Self::S612),
            90 => Ok(Self::S622),
            91 => Ok(Self::S632),
            92 => Ok(Self::S702),
            93 => Ok(Self::S712),
            94 => Ok(Self::S722),
            95 => Ok(Self::S732),
            96 => Ok(Self::S003),
            97 => Ok(Self::S013),
            98 => Ok(Self::S023),
            99 => Ok(Self::S033),
            100 => Ok(Self::S103),
            101 => Ok(Self::S113),
            102 => Ok(Self::S123),
            103 => Ok(Self::S133),
            104 => Ok(Self::S203),
            105 => Ok(Self::S213),
            106 => Ok(Self::S223),
            107 => Ok(Self::S233),
            108 => Ok(Self::S303),
            109 => Ok(Self::S313),
            110 => Ok(Self::S323),
            111 => Ok(Self::S333),
            112 => Ok(Self::S403),
            113 => Ok(Self::S413),
            114 => Ok(Self::S423),
            115 => Ok(Self::S433),
            116 => Ok(Self::S503),
            117 => Ok(Self::S513),
            118 => Ok(Self::S523),
            119 => Ok(Self::S533),
            120 => Ok(Self::S603),
            121 => Ok(Self::S613),
            122 => Ok(Self::S623),
            123 => Ok(Self::S633),
            124 => Ok(Self::S703),
            125 => Ok(Self::S713),
            126 => Ok(Self::S723),
            127 => Ok(Self::S733),
            x => Err(IntRegisterConversionError::new_out_of_range(
                x,
                128,
                "R4000AllegrexS",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        128
    }
}
impl TryFrom<u32> for R4000AllegrexS {
    type Error = IntRegisterConversionError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexS {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexS> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexS) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R4000ALLEGREX_S {
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
        for (i, x) in R4000ALLEGREX_S.iter().enumerate() {
            assert!(x.value() as usize == i, "Broken register index?");
            x.check_valid_entry();
        }
    }
}
