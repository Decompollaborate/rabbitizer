/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
use core::ops::Index;
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
        ..RegisterDescriptor::new("C000", 0, concat!("$", "0"), false)
    };
    table[R4000AllegrexV3D::C010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C010", 1, concat!("$", "1"), false)
    };
    table[R4000AllegrexV3D::C020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C020", 2, concat!("$", "2"), false)
    };
    table[R4000AllegrexV3D::C030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C030", 3, concat!("$", "3"), false)
    };
    table[R4000AllegrexV3D::C100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C100", 4, concat!("$", "4"), false)
    };
    table[R4000AllegrexV3D::C110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C110", 5, concat!("$", "5"), false)
    };
    table[R4000AllegrexV3D::C120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C120", 6, concat!("$", "6"), false)
    };
    table[R4000AllegrexV3D::C130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C130", 7, concat!("$", "7"), false)
    };
    table[R4000AllegrexV3D::C200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C200", 8, concat!("$", "8"), false)
    };
    table[R4000AllegrexV3D::C210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C210", 9, concat!("$", "9"), false)
    };
    table[R4000AllegrexV3D::C220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C220", 10, concat!("$", "10"), false)
    };
    table[R4000AllegrexV3D::C230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C230", 11, concat!("$", "11"), false)
    };
    table[R4000AllegrexV3D::C300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C300", 12, concat!("$", "12"), false)
    };
    table[R4000AllegrexV3D::C310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C310", 13, concat!("$", "13"), false)
    };
    table[R4000AllegrexV3D::C320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C320", 14, concat!("$", "14"), false)
    };
    table[R4000AllegrexV3D::C330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C330", 15, concat!("$", "15"), false)
    };
    table[R4000AllegrexV3D::C400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C400", 16, concat!("$", "16"), false)
    };
    table[R4000AllegrexV3D::C410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C410", 17, concat!("$", "17"), false)
    };
    table[R4000AllegrexV3D::C420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C420", 18, concat!("$", "18"), false)
    };
    table[R4000AllegrexV3D::C430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C430", 19, concat!("$", "19"), false)
    };
    table[R4000AllegrexV3D::C500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C500", 20, concat!("$", "20"), false)
    };
    table[R4000AllegrexV3D::C510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C510", 21, concat!("$", "21"), false)
    };
    table[R4000AllegrexV3D::C520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C520", 22, concat!("$", "22"), false)
    };
    table[R4000AllegrexV3D::C530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C530", 23, concat!("$", "23"), false)
    };
    table[R4000AllegrexV3D::C600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C600", 24, concat!("$", "24"), false)
    };
    table[R4000AllegrexV3D::C610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C610", 25, concat!("$", "25"), false)
    };
    table[R4000AllegrexV3D::C620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C620", 26, concat!("$", "26"), false)
    };
    table[R4000AllegrexV3D::C630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C630", 27, concat!("$", "27"), false)
    };
    table[R4000AllegrexV3D::C700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C700", 28, concat!("$", "28"), false)
    };
    table[R4000AllegrexV3D::C710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C710", 29, concat!("$", "29"), false)
    };
    table[R4000AllegrexV3D::C720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C720", 30, concat!("$", "30"), false)
    };
    table[R4000AllegrexV3D::C730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C730", 31, concat!("$", "31"), false)
    };
    table[R4000AllegrexV3D::R000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R000", 32, concat!("$", "32"), false)
    };
    table[R4000AllegrexV3D::R001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R001", 33, concat!("$", "33"), false)
    };
    table[R4000AllegrexV3D::R002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R002", 34, concat!("$", "34"), false)
    };
    table[R4000AllegrexV3D::R003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R003", 35, concat!("$", "35"), false)
    };
    table[R4000AllegrexV3D::R100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R100", 36, concat!("$", "36"), false)
    };
    table[R4000AllegrexV3D::R101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R101", 37, concat!("$", "37"), false)
    };
    table[R4000AllegrexV3D::R102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R102", 38, concat!("$", "38"), false)
    };
    table[R4000AllegrexV3D::R103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R103", 39, concat!("$", "39"), false)
    };
    table[R4000AllegrexV3D::R200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R200", 40, concat!("$", "40"), false)
    };
    table[R4000AllegrexV3D::R201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R201", 41, concat!("$", "41"), false)
    };
    table[R4000AllegrexV3D::R202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R202", 42, concat!("$", "42"), false)
    };
    table[R4000AllegrexV3D::R203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R203", 43, concat!("$", "43"), false)
    };
    table[R4000AllegrexV3D::R300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R300", 44, concat!("$", "44"), false)
    };
    table[R4000AllegrexV3D::R301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R301", 45, concat!("$", "45"), false)
    };
    table[R4000AllegrexV3D::R302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R302", 46, concat!("$", "46"), false)
    };
    table[R4000AllegrexV3D::R303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R303", 47, concat!("$", "47"), false)
    };
    table[R4000AllegrexV3D::R400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R400", 48, concat!("$", "48"), false)
    };
    table[R4000AllegrexV3D::R401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R401", 49, concat!("$", "49"), false)
    };
    table[R4000AllegrexV3D::R402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R402", 50, concat!("$", "50"), false)
    };
    table[R4000AllegrexV3D::R403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R403", 51, concat!("$", "51"), false)
    };
    table[R4000AllegrexV3D::R500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R500", 52, concat!("$", "52"), false)
    };
    table[R4000AllegrexV3D::R501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R501", 53, concat!("$", "53"), false)
    };
    table[R4000AllegrexV3D::R502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R502", 54, concat!("$", "54"), false)
    };
    table[R4000AllegrexV3D::R503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R503", 55, concat!("$", "55"), false)
    };
    table[R4000AllegrexV3D::R600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R600", 56, concat!("$", "56"), false)
    };
    table[R4000AllegrexV3D::R601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R601", 57, concat!("$", "57"), false)
    };
    table[R4000AllegrexV3D::R602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R602", 58, concat!("$", "58"), false)
    };
    table[R4000AllegrexV3D::R603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R603", 59, concat!("$", "59"), false)
    };
    table[R4000AllegrexV3D::R700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R700", 60, concat!("$", "60"), false)
    };
    table[R4000AllegrexV3D::R701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R701", 61, concat!("$", "61"), false)
    };
    table[R4000AllegrexV3D::R702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R702", 62, concat!("$", "62"), false)
    };
    table[R4000AllegrexV3D::R703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R703", 63, concat!("$", "63"), false)
    };
    table[R4000AllegrexV3D::C001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C001", 64, concat!("$", "64"), false)
    };
    table[R4000AllegrexV3D::C011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C011", 65, concat!("$", "65"), false)
    };
    table[R4000AllegrexV3D::C021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C021", 66, concat!("$", "66"), false)
    };
    table[R4000AllegrexV3D::C031 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C031", 67, concat!("$", "67"), false)
    };
    table[R4000AllegrexV3D::C101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C101", 68, concat!("$", "68"), false)
    };
    table[R4000AllegrexV3D::C111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C111", 69, concat!("$", "69"), false)
    };
    table[R4000AllegrexV3D::C121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C121", 70, concat!("$", "70"), false)
    };
    table[R4000AllegrexV3D::C131 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C131", 71, concat!("$", "71"), false)
    };
    table[R4000AllegrexV3D::C201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C201", 72, concat!("$", "72"), false)
    };
    table[R4000AllegrexV3D::C211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C211", 73, concat!("$", "73"), false)
    };
    table[R4000AllegrexV3D::C221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C221", 74, concat!("$", "74"), false)
    };
    table[R4000AllegrexV3D::C231 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C231", 75, concat!("$", "75"), false)
    };
    table[R4000AllegrexV3D::C301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C301", 76, concat!("$", "76"), false)
    };
    table[R4000AllegrexV3D::C311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C311", 77, concat!("$", "77"), false)
    };
    table[R4000AllegrexV3D::C321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C321", 78, concat!("$", "78"), false)
    };
    table[R4000AllegrexV3D::C331 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C331", 79, concat!("$", "79"), false)
    };
    table[R4000AllegrexV3D::C401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C401", 80, concat!("$", "80"), false)
    };
    table[R4000AllegrexV3D::C411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C411", 81, concat!("$", "81"), false)
    };
    table[R4000AllegrexV3D::C421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C421", 82, concat!("$", "82"), false)
    };
    table[R4000AllegrexV3D::C431 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C431", 83, concat!("$", "83"), false)
    };
    table[R4000AllegrexV3D::C501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C501", 84, concat!("$", "84"), false)
    };
    table[R4000AllegrexV3D::C511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C511", 85, concat!("$", "85"), false)
    };
    table[R4000AllegrexV3D::C521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C521", 86, concat!("$", "86"), false)
    };
    table[R4000AllegrexV3D::C531 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C531", 87, concat!("$", "87"), false)
    };
    table[R4000AllegrexV3D::C601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C601", 88, concat!("$", "88"), false)
    };
    table[R4000AllegrexV3D::C611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C611", 89, concat!("$", "89"), false)
    };
    table[R4000AllegrexV3D::C621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C621", 90, concat!("$", "90"), false)
    };
    table[R4000AllegrexV3D::C631 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C631", 91, concat!("$", "91"), false)
    };
    table[R4000AllegrexV3D::C701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C701", 92, concat!("$", "92"), false)
    };
    table[R4000AllegrexV3D::C711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C711", 93, concat!("$", "93"), false)
    };
    table[R4000AllegrexV3D::C721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C721", 94, concat!("$", "94"), false)
    };
    table[R4000AllegrexV3D::C731 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C731", 95, concat!("$", "95"), false)
    };
    table[R4000AllegrexV3D::R010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R010", 96, concat!("$", "96"), false)
    };
    table[R4000AllegrexV3D::R011 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R011", 97, concat!("$", "97"), false)
    };
    table[R4000AllegrexV3D::R012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R012", 98, concat!("$", "98"), false)
    };
    table[R4000AllegrexV3D::R013 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R013", 99, concat!("$", "99"), false)
    };
    table[R4000AllegrexV3D::R110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R110", 100, concat!("$", "100"), false)
    };
    table[R4000AllegrexV3D::R111 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R111", 101, concat!("$", "101"), false)
    };
    table[R4000AllegrexV3D::R112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R112", 102, concat!("$", "102"), false)
    };
    table[R4000AllegrexV3D::R113 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R113", 103, concat!("$", "103"), false)
    };
    table[R4000AllegrexV3D::R210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R210", 104, concat!("$", "104"), false)
    };
    table[R4000AllegrexV3D::R211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R211", 105, concat!("$", "105"), false)
    };
    table[R4000AllegrexV3D::R212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R212", 106, concat!("$", "106"), false)
    };
    table[R4000AllegrexV3D::R213 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R213", 107, concat!("$", "107"), false)
    };
    table[R4000AllegrexV3D::R310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R310", 108, concat!("$", "108"), false)
    };
    table[R4000AllegrexV3D::R311 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R311", 109, concat!("$", "109"), false)
    };
    table[R4000AllegrexV3D::R312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R312", 110, concat!("$", "110"), false)
    };
    table[R4000AllegrexV3D::R313 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R313", 111, concat!("$", "111"), false)
    };
    table[R4000AllegrexV3D::R410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R410", 112, concat!("$", "112"), false)
    };
    table[R4000AllegrexV3D::R411 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R411", 113, concat!("$", "113"), false)
    };
    table[R4000AllegrexV3D::R412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R412", 114, concat!("$", "114"), false)
    };
    table[R4000AllegrexV3D::R413 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R413", 115, concat!("$", "115"), false)
    };
    table[R4000AllegrexV3D::R510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R510", 116, concat!("$", "116"), false)
    };
    table[R4000AllegrexV3D::R511 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R511", 117, concat!("$", "117"), false)
    };
    table[R4000AllegrexV3D::R512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R512", 118, concat!("$", "118"), false)
    };
    table[R4000AllegrexV3D::R513 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R513", 119, concat!("$", "119"), false)
    };
    table[R4000AllegrexV3D::R610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R610", 120, concat!("$", "120"), false)
    };
    table[R4000AllegrexV3D::R611 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R611", 121, concat!("$", "121"), false)
    };
    table[R4000AllegrexV3D::R612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R612", 122, concat!("$", "122"), false)
    };
    table[R4000AllegrexV3D::R613 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R613", 123, concat!("$", "123"), false)
    };
    table[R4000AllegrexV3D::R710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R710", 124, concat!("$", "124"), false)
    };
    table[R4000AllegrexV3D::R711 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R711", 125, concat!("$", "125"), false)
    };
    table[R4000AllegrexV3D::R712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R712", 126, concat!("$", "126"), false)
    };
    table[R4000AllegrexV3D::R713 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R713", 127, concat!("$", "127"), false)
    };
    table
};
impl R4000AllegrexV3D {
    pub const fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
        match value {
            0 => Ok(Self::C000),
            1 => Ok(Self::C010),
            2 => Ok(Self::C020),
            3 => Ok(Self::C030),
            4 => Ok(Self::C100),
            5 => Ok(Self::C110),
            6 => Ok(Self::C120),
            7 => Ok(Self::C130),
            8 => Ok(Self::C200),
            9 => Ok(Self::C210),
            10 => Ok(Self::C220),
            11 => Ok(Self::C230),
            12 => Ok(Self::C300),
            13 => Ok(Self::C310),
            14 => Ok(Self::C320),
            15 => Ok(Self::C330),
            16 => Ok(Self::C400),
            17 => Ok(Self::C410),
            18 => Ok(Self::C420),
            19 => Ok(Self::C430),
            20 => Ok(Self::C500),
            21 => Ok(Self::C510),
            22 => Ok(Self::C520),
            23 => Ok(Self::C530),
            24 => Ok(Self::C600),
            25 => Ok(Self::C610),
            26 => Ok(Self::C620),
            27 => Ok(Self::C630),
            28 => Ok(Self::C700),
            29 => Ok(Self::C710),
            30 => Ok(Self::C720),
            31 => Ok(Self::C730),
            32 => Ok(Self::R000),
            33 => Ok(Self::R001),
            34 => Ok(Self::R002),
            35 => Ok(Self::R003),
            36 => Ok(Self::R100),
            37 => Ok(Self::R101),
            38 => Ok(Self::R102),
            39 => Ok(Self::R103),
            40 => Ok(Self::R200),
            41 => Ok(Self::R201),
            42 => Ok(Self::R202),
            43 => Ok(Self::R203),
            44 => Ok(Self::R300),
            45 => Ok(Self::R301),
            46 => Ok(Self::R302),
            47 => Ok(Self::R303),
            48 => Ok(Self::R400),
            49 => Ok(Self::R401),
            50 => Ok(Self::R402),
            51 => Ok(Self::R403),
            52 => Ok(Self::R500),
            53 => Ok(Self::R501),
            54 => Ok(Self::R502),
            55 => Ok(Self::R503),
            56 => Ok(Self::R600),
            57 => Ok(Self::R601),
            58 => Ok(Self::R602),
            59 => Ok(Self::R603),
            60 => Ok(Self::R700),
            61 => Ok(Self::R701),
            62 => Ok(Self::R702),
            63 => Ok(Self::R703),
            64 => Ok(Self::C001),
            65 => Ok(Self::C011),
            66 => Ok(Self::C021),
            67 => Ok(Self::C031),
            68 => Ok(Self::C101),
            69 => Ok(Self::C111),
            70 => Ok(Self::C121),
            71 => Ok(Self::C131),
            72 => Ok(Self::C201),
            73 => Ok(Self::C211),
            74 => Ok(Self::C221),
            75 => Ok(Self::C231),
            76 => Ok(Self::C301),
            77 => Ok(Self::C311),
            78 => Ok(Self::C321),
            79 => Ok(Self::C331),
            80 => Ok(Self::C401),
            81 => Ok(Self::C411),
            82 => Ok(Self::C421),
            83 => Ok(Self::C431),
            84 => Ok(Self::C501),
            85 => Ok(Self::C511),
            86 => Ok(Self::C521),
            87 => Ok(Self::C531),
            88 => Ok(Self::C601),
            89 => Ok(Self::C611),
            90 => Ok(Self::C621),
            91 => Ok(Self::C631),
            92 => Ok(Self::C701),
            93 => Ok(Self::C711),
            94 => Ok(Self::C721),
            95 => Ok(Self::C731),
            96 => Ok(Self::R010),
            97 => Ok(Self::R011),
            98 => Ok(Self::R012),
            99 => Ok(Self::R013),
            100 => Ok(Self::R110),
            101 => Ok(Self::R111),
            102 => Ok(Self::R112),
            103 => Ok(Self::R113),
            104 => Ok(Self::R210),
            105 => Ok(Self::R211),
            106 => Ok(Self::R212),
            107 => Ok(Self::R213),
            108 => Ok(Self::R310),
            109 => Ok(Self::R311),
            110 => Ok(Self::R312),
            111 => Ok(Self::R313),
            112 => Ok(Self::R410),
            113 => Ok(Self::R411),
            114 => Ok(Self::R412),
            115 => Ok(Self::R413),
            116 => Ok(Self::R510),
            117 => Ok(Self::R511),
            118 => Ok(Self::R512),
            119 => Ok(Self::R513),
            120 => Ok(Self::R610),
            121 => Ok(Self::R611),
            122 => Ok(Self::R612),
            123 => Ok(Self::R613),
            124 => Ok(Self::R710),
            125 => Ok(Self::R711),
            126 => Ok(Self::R712),
            127 => Ok(Self::R713),
            x => Err(IntRegisterConversionError::new_out_of_range(
                x,
                128,
                "R4000AllegrexV3D",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        128
    }
}
impl TryFrom<u32> for R4000AllegrexV3D {
    type Error = IntRegisterConversionError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexV3D {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexV3D> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexV3D) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R4000ALLEGREX_V3D {
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
        for (i, x) in R4000ALLEGREX_V3D.iter().enumerate() {
            assert!(x.value() as usize == i, "Broken register index?");
            x.check_valid_entry();
        }
    }
}
