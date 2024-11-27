/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexM4x4 {
    M000 = 0,
    M010 = 1,
    M020 = 2,
    M030 = 3,
    M100 = 4,
    M110 = 5,
    M120 = 6,
    M130 = 7,
    M200 = 8,
    M210 = 9,
    M220 = 10,
    M230 = 11,
    M300 = 12,
    M310 = 13,
    M320 = 14,
    M330 = 15,
    M400 = 16,
    M410 = 17,
    M420 = 18,
    M430 = 19,
    M500 = 20,
    M510 = 21,
    M520 = 22,
    M530 = 23,
    M600 = 24,
    M610 = 25,
    M620 = 26,
    M630 = 27,
    M700 = 28,
    M710 = 29,
    M720 = 30,
    M730 = 31,
    E000 = 32,
    E001 = 33,
    E002 = 34,
    E003 = 35,
    E100 = 36,
    E101 = 37,
    E102 = 38,
    E103 = 39,
    E200 = 40,
    E201 = 41,
    E202 = 42,
    E203 = 43,
    E300 = 44,
    E301 = 45,
    E302 = 46,
    E303 = 47,
    E400 = 48,
    E401 = 49,
    E402 = 50,
    E403 = 51,
    E500 = 52,
    E501 = 53,
    E502 = 54,
    E503 = 55,
    E600 = 56,
    E601 = 57,
    E602 = 58,
    E603 = 59,
    E700 = 60,
    E701 = 61,
    E702 = 62,
    E703 = 63,
    M002 = 64,
    M012 = 65,
    M022 = 66,
    M032 = 67,
    M102 = 68,
    M112 = 69,
    M122 = 70,
    M132 = 71,
    M202 = 72,
    M212 = 73,
    M222 = 74,
    M232 = 75,
    M302 = 76,
    M312 = 77,
    M322 = 78,
    M332 = 79,
    M402 = 80,
    M412 = 81,
    M422 = 82,
    M432 = 83,
    M502 = 84,
    M512 = 85,
    M522 = 86,
    M532 = 87,
    M602 = 88,
    M612 = 89,
    M622 = 90,
    M632 = 91,
    M702 = 92,
    M712 = 93,
    M722 = 94,
    M732 = 95,
    E020 = 96,
    E021 = 97,
    E022 = 98,
    E023 = 99,
    E120 = 100,
    E121 = 101,
    E122 = 102,
    E123 = 103,
    E220 = 104,
    E221 = 105,
    E222 = 106,
    E223 = 107,
    E320 = 108,
    E321 = 109,
    E322 = 110,
    E323 = 111,
    E420 = 112,
    E421 = 113,
    E422 = 114,
    E423 = 115,
    E520 = 116,
    E521 = 117,
    E522 = 118,
    E523 = 119,
    E620 = 120,
    E621 = 121,
    E622 = 122,
    E623 = 123,
    E720 = 124,
    E721 = 125,
    E722 = 126,
    E723 = 127,
}
pub static R4000ALLEGREX_M4X4: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[R4000AllegrexM4x4::M000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M000", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M010", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M020", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M030", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M100", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M110", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M120", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M130", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M200", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M210", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M220", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M230", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M300", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M310", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M320", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M330", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M400", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M410", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M420", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M430", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M500", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M510", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M520", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M530", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M600", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M610", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M620", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M630", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M700", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M710", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M720", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M730", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E000", 32, concat!("$", "32"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E001", 33, concat!("$", "33"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E002", 34, concat!("$", "34"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E003", 35, concat!("$", "35"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E100", 36, concat!("$", "36"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E101", 37, concat!("$", "37"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E102", 38, concat!("$", "38"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E103", 39, concat!("$", "39"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E200", 40, concat!("$", "40"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E201", 41, concat!("$", "41"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E202", 42, concat!("$", "42"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E203", 43, concat!("$", "43"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E300", 44, concat!("$", "44"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E301", 45, concat!("$", "45"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E302", 46, concat!("$", "46"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E303", 47, concat!("$", "47"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E400", 48, concat!("$", "48"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E401", 49, concat!("$", "49"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E402", 50, concat!("$", "50"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E403", 51, concat!("$", "51"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E500", 52, concat!("$", "52"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E501", 53, concat!("$", "53"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E502", 54, concat!("$", "54"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E503", 55, concat!("$", "55"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E600", 56, concat!("$", "56"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E601", 57, concat!("$", "57"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E602", 58, concat!("$", "58"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E603", 59, concat!("$", "59"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E700", 60, concat!("$", "60"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E701", 61, concat!("$", "61"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E702", 62, concat!("$", "62"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E703", 63, concat!("$", "63"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M002", 64, concat!("$", "64"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M012", 65, concat!("$", "65"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M022", 66, concat!("$", "66"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M032 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M032", 67, concat!("$", "67"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M102", 68, concat!("$", "68"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M112", 69, concat!("$", "69"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M122", 70, concat!("$", "70"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M132 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M132", 71, concat!("$", "71"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M202", 72, concat!("$", "72"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M212", 73, concat!("$", "73"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M222", 74, concat!("$", "74"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M232 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M232", 75, concat!("$", "75"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M302", 76, concat!("$", "76"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M312", 77, concat!("$", "77"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M322", 78, concat!("$", "78"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M332 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M332", 79, concat!("$", "79"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M402", 80, concat!("$", "80"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M412", 81, concat!("$", "81"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M422", 82, concat!("$", "82"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M432 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M432", 83, concat!("$", "83"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M502", 84, concat!("$", "84"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M512", 85, concat!("$", "85"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M522", 86, concat!("$", "86"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M532 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M532", 87, concat!("$", "87"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M602", 88, concat!("$", "88"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M612", 89, concat!("$", "89"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M622", 90, concat!("$", "90"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M632 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M632", 91, concat!("$", "91"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M702", 92, concat!("$", "92"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M712", 93, concat!("$", "93"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M722", 94, concat!("$", "94"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::M732 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M732", 95, concat!("$", "95"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E020", 96, concat!("$", "96"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E021", 97, concat!("$", "97"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E022", 98, concat!("$", "98"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E023 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E023", 99, concat!("$", "99"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E120", 100, concat!("$", "100"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E121", 101, concat!("$", "101"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E122", 102, concat!("$", "102"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E123 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E123", 103, concat!("$", "103"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E220", 104, concat!("$", "104"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E221", 105, concat!("$", "105"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E222", 106, concat!("$", "106"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E223 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E223", 107, concat!("$", "107"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E320", 108, concat!("$", "108"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E321", 109, concat!("$", "109"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E322", 110, concat!("$", "110"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E323 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E323", 111, concat!("$", "111"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E420", 112, concat!("$", "112"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E421", 113, concat!("$", "113"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E422", 114, concat!("$", "114"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E423 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E423", 115, concat!("$", "115"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E520", 116, concat!("$", "116"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E521", 117, concat!("$", "117"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E522", 118, concat!("$", "118"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E523 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E523", 119, concat!("$", "119"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E620", 120, concat!("$", "120"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E621", 121, concat!("$", "121"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E622", 122, concat!("$", "122"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E623 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E623", 123, concat!("$", "123"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E720", 124, concat!("$", "124"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E721", 125, concat!("$", "125"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E722", 126, concat!("$", "126"))
    }
    .check_panic_chain();
    table[R4000AllegrexM4x4::E723 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E723", 127, concat!("$", "127"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexM4x4 {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            0 => Ok(Self::M000),
            1 => Ok(Self::M010),
            2 => Ok(Self::M020),
            3 => Ok(Self::M030),
            4 => Ok(Self::M100),
            5 => Ok(Self::M110),
            6 => Ok(Self::M120),
            7 => Ok(Self::M130),
            8 => Ok(Self::M200),
            9 => Ok(Self::M210),
            10 => Ok(Self::M220),
            11 => Ok(Self::M230),
            12 => Ok(Self::M300),
            13 => Ok(Self::M310),
            14 => Ok(Self::M320),
            15 => Ok(Self::M330),
            16 => Ok(Self::M400),
            17 => Ok(Self::M410),
            18 => Ok(Self::M420),
            19 => Ok(Self::M430),
            20 => Ok(Self::M500),
            21 => Ok(Self::M510),
            22 => Ok(Self::M520),
            23 => Ok(Self::M530),
            24 => Ok(Self::M600),
            25 => Ok(Self::M610),
            26 => Ok(Self::M620),
            27 => Ok(Self::M630),
            28 => Ok(Self::M700),
            29 => Ok(Self::M710),
            30 => Ok(Self::M720),
            31 => Ok(Self::M730),
            32 => Ok(Self::E000),
            33 => Ok(Self::E001),
            34 => Ok(Self::E002),
            35 => Ok(Self::E003),
            36 => Ok(Self::E100),
            37 => Ok(Self::E101),
            38 => Ok(Self::E102),
            39 => Ok(Self::E103),
            40 => Ok(Self::E200),
            41 => Ok(Self::E201),
            42 => Ok(Self::E202),
            43 => Ok(Self::E203),
            44 => Ok(Self::E300),
            45 => Ok(Self::E301),
            46 => Ok(Self::E302),
            47 => Ok(Self::E303),
            48 => Ok(Self::E400),
            49 => Ok(Self::E401),
            50 => Ok(Self::E402),
            51 => Ok(Self::E403),
            52 => Ok(Self::E500),
            53 => Ok(Self::E501),
            54 => Ok(Self::E502),
            55 => Ok(Self::E503),
            56 => Ok(Self::E600),
            57 => Ok(Self::E601),
            58 => Ok(Self::E602),
            59 => Ok(Self::E603),
            60 => Ok(Self::E700),
            61 => Ok(Self::E701),
            62 => Ok(Self::E702),
            63 => Ok(Self::E703),
            64 => Ok(Self::M002),
            65 => Ok(Self::M012),
            66 => Ok(Self::M022),
            67 => Ok(Self::M032),
            68 => Ok(Self::M102),
            69 => Ok(Self::M112),
            70 => Ok(Self::M122),
            71 => Ok(Self::M132),
            72 => Ok(Self::M202),
            73 => Ok(Self::M212),
            74 => Ok(Self::M222),
            75 => Ok(Self::M232),
            76 => Ok(Self::M302),
            77 => Ok(Self::M312),
            78 => Ok(Self::M322),
            79 => Ok(Self::M332),
            80 => Ok(Self::M402),
            81 => Ok(Self::M412),
            82 => Ok(Self::M422),
            83 => Ok(Self::M432),
            84 => Ok(Self::M502),
            85 => Ok(Self::M512),
            86 => Ok(Self::M522),
            87 => Ok(Self::M532),
            88 => Ok(Self::M602),
            89 => Ok(Self::M612),
            90 => Ok(Self::M622),
            91 => Ok(Self::M632),
            92 => Ok(Self::M702),
            93 => Ok(Self::M712),
            94 => Ok(Self::M722),
            95 => Ok(Self::M732),
            96 => Ok(Self::E020),
            97 => Ok(Self::E021),
            98 => Ok(Self::E022),
            99 => Ok(Self::E023),
            100 => Ok(Self::E120),
            101 => Ok(Self::E121),
            102 => Ok(Self::E122),
            103 => Ok(Self::E123),
            104 => Ok(Self::E220),
            105 => Ok(Self::E221),
            106 => Ok(Self::E222),
            107 => Ok(Self::E223),
            108 => Ok(Self::E320),
            109 => Ok(Self::E321),
            110 => Ok(Self::E322),
            111 => Ok(Self::E323),
            112 => Ok(Self::E420),
            113 => Ok(Self::E421),
            114 => Ok(Self::E422),
            115 => Ok(Self::E423),
            116 => Ok(Self::E520),
            117 => Ok(Self::E521),
            118 => Ok(Self::E522),
            119 => Ok(Self::E523),
            120 => Ok(Self::E620),
            121 => Ok(Self::E621),
            122 => Ok(Self::E622),
            123 => Ok(Self::E623),
            124 => Ok(Self::E720),
            125 => Ok(Self::E721),
            126 => Ok(Self::E722),
            127 => Ok(Self::E723),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 128,
                register_kind: "R4000AllegrexM4x4",
            }),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        128
    }
}
impl TryFrom<u32> for R4000AllegrexM4x4 {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexM4x4 {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexM4x4> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexM4x4) -> &Self::Output {
        &self[index as usize]
    }
}
