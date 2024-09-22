/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum RegisterR4000AllegrexM3x3 {
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
    M001 = 64,
    M011 = 65,
    M021 = 66,
    M031 = 67,
    M101 = 68,
    M111 = 69,
    M121 = 70,
    M131 = 71,
    M201 = 72,
    M211 = 73,
    M221 = 74,
    M231 = 75,
    M301 = 76,
    M311 = 77,
    M321 = 78,
    M331 = 79,
    M401 = 80,
    M411 = 81,
    M421 = 82,
    M431 = 83,
    M501 = 84,
    M511 = 85,
    M521 = 86,
    M531 = 87,
    M601 = 88,
    M611 = 89,
    M621 = 90,
    M631 = 91,
    M701 = 92,
    M711 = 93,
    M721 = 94,
    M731 = 95,
    E010 = 96,
    E011 = 97,
    E012 = 98,
    E013 = 99,
    E110 = 100,
    E111 = 101,
    E112 = 102,
    E113 = 103,
    E210 = 104,
    E211 = 105,
    E212 = 106,
    E213 = 107,
    E310 = 108,
    E311 = 109,
    E312 = 110,
    E313 = 111,
    E410 = 112,
    E411 = 113,
    E412 = 114,
    E413 = 115,
    E510 = 116,
    E511 = 117,
    E512 = 118,
    E513 = 119,
    E610 = 120,
    E611 = 121,
    E612 = 122,
    E613 = 123,
    E710 = 124,
    E711 = 125,
    E712 = 126,
    E713 = 127,
}
pub static R4000ALLEGREX_M3X3_REGISTERS: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[RegisterR4000AllegrexM3x3::M000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M000", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M010", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M020", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M030", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M100", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M110", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M120", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M130", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M200", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M210", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M220", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M230", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M300", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M310", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M320", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M330", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M400", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M410", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M420", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M430", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M500", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M510", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M520", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M530", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M600", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M610", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M620", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M630", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M700", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M710", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M720", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M730", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E000", 32, concat!("$", "32"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E001", 33, concat!("$", "33"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E002", 34, concat!("$", "34"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E003", 35, concat!("$", "35"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E100", 36, concat!("$", "36"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E101", 37, concat!("$", "37"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E102", 38, concat!("$", "38"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E103", 39, concat!("$", "39"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E200", 40, concat!("$", "40"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E201", 41, concat!("$", "41"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E202", 42, concat!("$", "42"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E203", 43, concat!("$", "43"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E300", 44, concat!("$", "44"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E301", 45, concat!("$", "45"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E302", 46, concat!("$", "46"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E303", 47, concat!("$", "47"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E400", 48, concat!("$", "48"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E401", 49, concat!("$", "49"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E402", 50, concat!("$", "50"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E403", 51, concat!("$", "51"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E500", 52, concat!("$", "52"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E501", 53, concat!("$", "53"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E502", 54, concat!("$", "54"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E503", 55, concat!("$", "55"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E600", 56, concat!("$", "56"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E601", 57, concat!("$", "57"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E602", 58, concat!("$", "58"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E603", 59, concat!("$", "59"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E700", 60, concat!("$", "60"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E701", 61, concat!("$", "61"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E702", 62, concat!("$", "62"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E703", 63, concat!("$", "63"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M001", 64, concat!("$", "64"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M011", 65, concat!("$", "65"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M021", 66, concat!("$", "66"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M031 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M031", 67, concat!("$", "67"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M101", 68, concat!("$", "68"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M111", 69, concat!("$", "69"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M121", 70, concat!("$", "70"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M131 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M131", 71, concat!("$", "71"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M201", 72, concat!("$", "72"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M211", 73, concat!("$", "73"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M221", 74, concat!("$", "74"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M231 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M231", 75, concat!("$", "75"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M301", 76, concat!("$", "76"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M311", 77, concat!("$", "77"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M321", 78, concat!("$", "78"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M331 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M331", 79, concat!("$", "79"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M401", 80, concat!("$", "80"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M411", 81, concat!("$", "81"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M421", 82, concat!("$", "82"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M431 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M431", 83, concat!("$", "83"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M501", 84, concat!("$", "84"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M511", 85, concat!("$", "85"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M521", 86, concat!("$", "86"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M531 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M531", 87, concat!("$", "87"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M601", 88, concat!("$", "88"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M611", 89, concat!("$", "89"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M621", 90, concat!("$", "90"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M631 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M631", 91, concat!("$", "91"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M701", 92, concat!("$", "92"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M711", 93, concat!("$", "93"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M721", 94, concat!("$", "94"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::M731 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("M731", 95, concat!("$", "95"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E010", 96, concat!("$", "96"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E011", 97, concat!("$", "97"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E012", 98, concat!("$", "98"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E013 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E013", 99, concat!("$", "99"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E110", 100, concat!("$", "100"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E111", 101, concat!("$", "101"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E112", 102, concat!("$", "102"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E113 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E113", 103, concat!("$", "103"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E210", 104, concat!("$", "104"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E211", 105, concat!("$", "105"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E212", 106, concat!("$", "106"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E213 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E213", 107, concat!("$", "107"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E310", 108, concat!("$", "108"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E311", 109, concat!("$", "109"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E312", 110, concat!("$", "110"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E313 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E313", 111, concat!("$", "111"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E410", 112, concat!("$", "112"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E411", 113, concat!("$", "113"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E412", 114, concat!("$", "114"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E413 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E413", 115, concat!("$", "115"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E510", 116, concat!("$", "116"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E511", 117, concat!("$", "117"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E512", 118, concat!("$", "118"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E513 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E513", 119, concat!("$", "119"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E610", 120, concat!("$", "120"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E611", 121, concat!("$", "121"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E612", 122, concat!("$", "122"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E613 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E613", 123, concat!("$", "123"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E710", 124, concat!("$", "124"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E711", 125, concat!("$", "125"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E712", 126, concat!("$", "126"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexM3x3::E713 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("E713", 127, concat!("$", "127"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
