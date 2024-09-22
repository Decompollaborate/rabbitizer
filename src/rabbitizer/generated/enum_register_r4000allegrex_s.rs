/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RegisterR4000AllegrexS {
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
pub static R4000ALLEGREX_S_REGISTERS: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[RegisterR4000AllegrexS::S000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S000", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S010", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S020", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S030", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S100", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S110", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S120", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S130", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S200", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S210", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S220", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S230", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S300", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S310", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S320", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S330", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S400", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S410", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S420", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S430", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S500", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S510", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S520", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S530", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S600", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S610", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S620", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S630", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S700", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S710", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S720", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S730", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S001", 32, concat!("$", "32"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S011", 33, concat!("$", "33"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S021", 34, concat!("$", "34"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S031 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S031", 35, concat!("$", "35"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S101", 36, concat!("$", "36"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S111", 37, concat!("$", "37"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S121", 38, concat!("$", "38"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S131 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S131", 39, concat!("$", "39"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S201", 40, concat!("$", "40"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S211", 41, concat!("$", "41"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S221", 42, concat!("$", "42"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S231 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S231", 43, concat!("$", "43"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S301", 44, concat!("$", "44"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S311", 45, concat!("$", "45"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S321", 46, concat!("$", "46"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S331 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S331", 47, concat!("$", "47"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S401", 48, concat!("$", "48"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S411", 49, concat!("$", "49"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S421", 50, concat!("$", "50"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S431 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S431", 51, concat!("$", "51"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S501", 52, concat!("$", "52"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S511", 53, concat!("$", "53"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S521", 54, concat!("$", "54"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S531 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S531", 55, concat!("$", "55"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S601", 56, concat!("$", "56"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S611", 57, concat!("$", "57"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S621", 58, concat!("$", "58"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S631 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S631", 59, concat!("$", "59"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S701", 60, concat!("$", "60"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S711", 61, concat!("$", "61"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S721", 62, concat!("$", "62"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S731 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S731", 63, concat!("$", "63"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S002", 64, concat!("$", "64"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S012", 65, concat!("$", "65"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S022", 66, concat!("$", "66"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S032 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S032", 67, concat!("$", "67"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S102", 68, concat!("$", "68"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S112", 69, concat!("$", "69"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S122", 70, concat!("$", "70"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S132 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S132", 71, concat!("$", "71"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S202", 72, concat!("$", "72"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S212", 73, concat!("$", "73"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S222", 74, concat!("$", "74"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S232 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S232", 75, concat!("$", "75"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S302", 76, concat!("$", "76"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S312", 77, concat!("$", "77"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S322", 78, concat!("$", "78"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S332 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S332", 79, concat!("$", "79"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S402", 80, concat!("$", "80"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S412", 81, concat!("$", "81"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S422", 82, concat!("$", "82"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S432 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S432", 83, concat!("$", "83"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S502", 84, concat!("$", "84"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S512", 85, concat!("$", "85"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S522", 86, concat!("$", "86"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S532 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S532", 87, concat!("$", "87"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S602", 88, concat!("$", "88"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S612", 89, concat!("$", "89"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S622", 90, concat!("$", "90"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S632 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S632", 91, concat!("$", "91"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S702", 92, concat!("$", "92"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S712", 93, concat!("$", "93"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S722", 94, concat!("$", "94"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S732 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S732", 95, concat!("$", "95"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S003", 96, concat!("$", "96"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S013 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S013", 97, concat!("$", "97"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S023 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S023", 98, concat!("$", "98"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S033 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S033", 99, concat!("$", "99"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S103", 100, concat!("$", "100"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S113 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S113", 101, concat!("$", "101"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S123 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S123", 102, concat!("$", "102"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S133 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S133", 103, concat!("$", "103"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S203", 104, concat!("$", "104"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S213 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S213", 105, concat!("$", "105"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S223 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S223", 106, concat!("$", "106"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S233 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S233", 107, concat!("$", "107"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S303", 108, concat!("$", "108"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S313 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S313", 109, concat!("$", "109"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S323 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S323", 110, concat!("$", "110"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S333 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S333", 111, concat!("$", "111"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S403", 112, concat!("$", "112"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S413 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S413", 113, concat!("$", "113"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S423 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S423", 114, concat!("$", "114"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S433 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S433", 115, concat!("$", "115"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S503", 116, concat!("$", "116"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S513 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S513", 117, concat!("$", "117"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S523 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S523", 118, concat!("$", "118"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S533 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S533", 119, concat!("$", "119"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S603", 120, concat!("$", "120"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S613 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S613", 121, concat!("$", "121"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S623 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S623", 122, concat!("$", "122"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S633 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S633", 123, concat!("$", "123"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S703", 124, concat!("$", "124"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S713 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S713", 125, concat!("$", "125"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S723 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S723", 126, concat!("$", "126"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexS::S733 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("S733", 127, concat!("$", "127"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
