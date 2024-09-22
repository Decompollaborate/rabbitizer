/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexV3D {
    C000 = 0,
    C010 = 1,
    C020 = 2,
    C030 = 3,
    C100 = 4,
    C110 = 5,
    C120 = 6,
    C130 = 7,
    C200 = 8,
    C210 = 9,
    C220 = 10,
    C230 = 11,
    C300 = 12,
    C310 = 13,
    C320 = 14,
    C330 = 15,
    C400 = 16,
    C410 = 17,
    C420 = 18,
    C430 = 19,
    C500 = 20,
    C510 = 21,
    C520 = 22,
    C530 = 23,
    C600 = 24,
    C610 = 25,
    C620 = 26,
    C630 = 27,
    C700 = 28,
    C710 = 29,
    C720 = 30,
    C730 = 31,
    R000 = 32,
    R001 = 33,
    R002 = 34,
    R003 = 35,
    R100 = 36,
    R101 = 37,
    R102 = 38,
    R103 = 39,
    R200 = 40,
    R201 = 41,
    R202 = 42,
    R203 = 43,
    R300 = 44,
    R301 = 45,
    R302 = 46,
    R303 = 47,
    R400 = 48,
    R401 = 49,
    R402 = 50,
    R403 = 51,
    R500 = 52,
    R501 = 53,
    R502 = 54,
    R503 = 55,
    R600 = 56,
    R601 = 57,
    R602 = 58,
    R603 = 59,
    R700 = 60,
    R701 = 61,
    R702 = 62,
    R703 = 63,
    C001 = 64,
    C011 = 65,
    C021 = 66,
    C031 = 67,
    C101 = 68,
    C111 = 69,
    C121 = 70,
    C131 = 71,
    C201 = 72,
    C211 = 73,
    C221 = 74,
    C231 = 75,
    C301 = 76,
    C311 = 77,
    C321 = 78,
    C331 = 79,
    C401 = 80,
    C411 = 81,
    C421 = 82,
    C431 = 83,
    C501 = 84,
    C511 = 85,
    C521 = 86,
    C531 = 87,
    C601 = 88,
    C611 = 89,
    C621 = 90,
    C631 = 91,
    C701 = 92,
    C711 = 93,
    C721 = 94,
    C731 = 95,
    R010 = 96,
    R011 = 97,
    R012 = 98,
    R013 = 99,
    R110 = 100,
    R111 = 101,
    R112 = 102,
    R113 = 103,
    R210 = 104,
    R211 = 105,
    R212 = 106,
    R213 = 107,
    R310 = 108,
    R311 = 109,
    R312 = 110,
    R313 = 111,
    R410 = 112,
    R411 = 113,
    R412 = 114,
    R413 = 115,
    R510 = 116,
    R511 = 117,
    R512 = 118,
    R513 = 119,
    R610 = 120,
    R611 = 121,
    R612 = 122,
    R613 = 123,
    R710 = 124,
    R711 = 125,
    R712 = 126,
    R713 = 127,
}
pub static R4000ALLEGREX_V3D: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[R4000AllegrexV3D::C000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C000", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C010", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C020", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C030", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C100", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C110", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C120", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C130", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C200", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C210", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C220", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C230", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C300", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C310", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C320", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C330", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C400", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C410", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C420", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C430", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C500", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C510", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C520", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C530", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C600", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C610", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C620", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C630", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C700", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C710", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C720", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C730", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R000", 32, concat!("$", "32"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R001", 33, concat!("$", "33"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R002", 34, concat!("$", "34"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R003", 35, concat!("$", "35"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R100", 36, concat!("$", "36"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R101", 37, concat!("$", "37"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R102", 38, concat!("$", "38"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R103", 39, concat!("$", "39"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R200", 40, concat!("$", "40"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R201", 41, concat!("$", "41"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R202", 42, concat!("$", "42"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R203", 43, concat!("$", "43"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R300", 44, concat!("$", "44"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R301", 45, concat!("$", "45"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R302", 46, concat!("$", "46"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R303", 47, concat!("$", "47"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R400", 48, concat!("$", "48"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R401", 49, concat!("$", "49"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R402", 50, concat!("$", "50"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R403", 51, concat!("$", "51"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R500", 52, concat!("$", "52"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R501", 53, concat!("$", "53"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R502", 54, concat!("$", "54"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R503", 55, concat!("$", "55"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R600", 56, concat!("$", "56"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R601", 57, concat!("$", "57"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R602", 58, concat!("$", "58"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R603", 59, concat!("$", "59"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R700", 60, concat!("$", "60"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R701", 61, concat!("$", "61"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R702", 62, concat!("$", "62"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R703", 63, concat!("$", "63"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C001", 64, concat!("$", "64"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C011", 65, concat!("$", "65"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C021", 66, concat!("$", "66"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C031 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C031", 67, concat!("$", "67"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C101", 68, concat!("$", "68"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C111", 69, concat!("$", "69"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C121", 70, concat!("$", "70"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C131 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C131", 71, concat!("$", "71"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C201", 72, concat!("$", "72"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C211", 73, concat!("$", "73"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C221", 74, concat!("$", "74"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C231 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C231", 75, concat!("$", "75"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C301", 76, concat!("$", "76"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C311", 77, concat!("$", "77"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C321", 78, concat!("$", "78"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C331 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C331", 79, concat!("$", "79"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C401", 80, concat!("$", "80"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C411", 81, concat!("$", "81"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C421", 82, concat!("$", "82"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C431 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C431", 83, concat!("$", "83"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C501", 84, concat!("$", "84"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C511", 85, concat!("$", "85"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C521", 86, concat!("$", "86"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C531 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C531", 87, concat!("$", "87"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C601", 88, concat!("$", "88"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C611", 89, concat!("$", "89"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C621", 90, concat!("$", "90"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C631 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C631", 91, concat!("$", "91"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C701", 92, concat!("$", "92"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C711", 93, concat!("$", "93"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C721", 94, concat!("$", "94"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::C731 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C731", 95, concat!("$", "95"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R010", 96, concat!("$", "96"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R011", 97, concat!("$", "97"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R012", 98, concat!("$", "98"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R013 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R013", 99, concat!("$", "99"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R110", 100, concat!("$", "100"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R111", 101, concat!("$", "101"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R112", 102, concat!("$", "102"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R113 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R113", 103, concat!("$", "103"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R210", 104, concat!("$", "104"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R211", 105, concat!("$", "105"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R212", 106, concat!("$", "106"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R213 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R213", 107, concat!("$", "107"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R310", 108, concat!("$", "108"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R311", 109, concat!("$", "109"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R312", 110, concat!("$", "110"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R313 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R313", 111, concat!("$", "111"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R410", 112, concat!("$", "112"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R411", 113, concat!("$", "113"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R412", 114, concat!("$", "114"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R413 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R413", 115, concat!("$", "115"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R510", 116, concat!("$", "116"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R511", 117, concat!("$", "117"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R512", 118, concat!("$", "118"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R513 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R513", 119, concat!("$", "119"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R610", 120, concat!("$", "120"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R611", 121, concat!("$", "121"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R612", 122, concat!("$", "122"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R613 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R613", 123, concat!("$", "123"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R710", 124, concat!("$", "124"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R711", 125, concat!("$", "125"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R712", 126, concat!("$", "126"))
    }
    .check_panic_chain();
    table[R4000AllegrexV3D::R713 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R713", 127, concat!("$", "127"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
