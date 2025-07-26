/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexV4D {
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
    C002 = 64,
    C012 = 65,
    C022 = 66,
    C032 = 67,
    C102 = 68,
    C112 = 69,
    C122 = 70,
    C132 = 71,
    C202 = 72,
    C212 = 73,
    C222 = 74,
    C232 = 75,
    C302 = 76,
    C312 = 77,
    C322 = 78,
    C332 = 79,
    C402 = 80,
    C412 = 81,
    C422 = 82,
    C432 = 83,
    C502 = 84,
    C512 = 85,
    C522 = 86,
    C532 = 87,
    C602 = 88,
    C612 = 89,
    C622 = 90,
    C632 = 91,
    C702 = 92,
    C712 = 93,
    C722 = 94,
    C732 = 95,
    R020 = 96,
    R021 = 97,
    R022 = 98,
    R023 = 99,
    R120 = 100,
    R121 = 101,
    R122 = 102,
    R123 = 103,
    R220 = 104,
    R221 = 105,
    R222 = 106,
    R223 = 107,
    R320 = 108,
    R321 = 109,
    R322 = 110,
    R323 = 111,
    R420 = 112,
    R421 = 113,
    R422 = 114,
    R423 = 115,
    R520 = 116,
    R521 = 117,
    R522 = 118,
    R523 = 119,
    R620 = 120,
    R621 = 121,
    R622 = 122,
    R623 = 123,
    R720 = 124,
    R721 = 125,
    R722 = 126,
    R723 = 127,
}
pub static R4000ALLEGREX_V4D: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[R4000AllegrexV4D::C000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C000", 0, concat!("$", "0"), false)
    };
    table[R4000AllegrexV4D::C010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C010", 1, concat!("$", "1"), false)
    };
    table[R4000AllegrexV4D::C020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C020", 2, concat!("$", "2"), false)
    };
    table[R4000AllegrexV4D::C030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C030", 3, concat!("$", "3"), false)
    };
    table[R4000AllegrexV4D::C100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C100", 4, concat!("$", "4"), false)
    };
    table[R4000AllegrexV4D::C110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C110", 5, concat!("$", "5"), false)
    };
    table[R4000AllegrexV4D::C120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C120", 6, concat!("$", "6"), false)
    };
    table[R4000AllegrexV4D::C130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C130", 7, concat!("$", "7"), false)
    };
    table[R4000AllegrexV4D::C200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C200", 8, concat!("$", "8"), false)
    };
    table[R4000AllegrexV4D::C210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C210", 9, concat!("$", "9"), false)
    };
    table[R4000AllegrexV4D::C220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C220", 10, concat!("$", "10"), false)
    };
    table[R4000AllegrexV4D::C230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C230", 11, concat!("$", "11"), false)
    };
    table[R4000AllegrexV4D::C300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C300", 12, concat!("$", "12"), false)
    };
    table[R4000AllegrexV4D::C310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C310", 13, concat!("$", "13"), false)
    };
    table[R4000AllegrexV4D::C320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C320", 14, concat!("$", "14"), false)
    };
    table[R4000AllegrexV4D::C330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C330", 15, concat!("$", "15"), false)
    };
    table[R4000AllegrexV4D::C400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C400", 16, concat!("$", "16"), false)
    };
    table[R4000AllegrexV4D::C410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C410", 17, concat!("$", "17"), false)
    };
    table[R4000AllegrexV4D::C420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C420", 18, concat!("$", "18"), false)
    };
    table[R4000AllegrexV4D::C430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C430", 19, concat!("$", "19"), false)
    };
    table[R4000AllegrexV4D::C500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C500", 20, concat!("$", "20"), false)
    };
    table[R4000AllegrexV4D::C510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C510", 21, concat!("$", "21"), false)
    };
    table[R4000AllegrexV4D::C520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C520", 22, concat!("$", "22"), false)
    };
    table[R4000AllegrexV4D::C530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C530", 23, concat!("$", "23"), false)
    };
    table[R4000AllegrexV4D::C600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C600", 24, concat!("$", "24"), false)
    };
    table[R4000AllegrexV4D::C610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C610", 25, concat!("$", "25"), false)
    };
    table[R4000AllegrexV4D::C620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C620", 26, concat!("$", "26"), false)
    };
    table[R4000AllegrexV4D::C630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C630", 27, concat!("$", "27"), false)
    };
    table[R4000AllegrexV4D::C700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C700", 28, concat!("$", "28"), false)
    };
    table[R4000AllegrexV4D::C710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C710", 29, concat!("$", "29"), false)
    };
    table[R4000AllegrexV4D::C720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C720", 30, concat!("$", "30"), false)
    };
    table[R4000AllegrexV4D::C730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C730", 31, concat!("$", "31"), false)
    };
    table[R4000AllegrexV4D::R000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R000", 32, concat!("$", "32"), false)
    };
    table[R4000AllegrexV4D::R001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R001", 33, concat!("$", "33"), false)
    };
    table[R4000AllegrexV4D::R002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R002", 34, concat!("$", "34"), false)
    };
    table[R4000AllegrexV4D::R003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R003", 35, concat!("$", "35"), false)
    };
    table[R4000AllegrexV4D::R100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R100", 36, concat!("$", "36"), false)
    };
    table[R4000AllegrexV4D::R101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R101", 37, concat!("$", "37"), false)
    };
    table[R4000AllegrexV4D::R102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R102", 38, concat!("$", "38"), false)
    };
    table[R4000AllegrexV4D::R103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R103", 39, concat!("$", "39"), false)
    };
    table[R4000AllegrexV4D::R200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R200", 40, concat!("$", "40"), false)
    };
    table[R4000AllegrexV4D::R201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R201", 41, concat!("$", "41"), false)
    };
    table[R4000AllegrexV4D::R202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R202", 42, concat!("$", "42"), false)
    };
    table[R4000AllegrexV4D::R203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R203", 43, concat!("$", "43"), false)
    };
    table[R4000AllegrexV4D::R300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R300", 44, concat!("$", "44"), false)
    };
    table[R4000AllegrexV4D::R301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R301", 45, concat!("$", "45"), false)
    };
    table[R4000AllegrexV4D::R302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R302", 46, concat!("$", "46"), false)
    };
    table[R4000AllegrexV4D::R303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R303", 47, concat!("$", "47"), false)
    };
    table[R4000AllegrexV4D::R400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R400", 48, concat!("$", "48"), false)
    };
    table[R4000AllegrexV4D::R401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R401", 49, concat!("$", "49"), false)
    };
    table[R4000AllegrexV4D::R402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R402", 50, concat!("$", "50"), false)
    };
    table[R4000AllegrexV4D::R403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R403", 51, concat!("$", "51"), false)
    };
    table[R4000AllegrexV4D::R500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R500", 52, concat!("$", "52"), false)
    };
    table[R4000AllegrexV4D::R501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R501", 53, concat!("$", "53"), false)
    };
    table[R4000AllegrexV4D::R502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R502", 54, concat!("$", "54"), false)
    };
    table[R4000AllegrexV4D::R503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R503", 55, concat!("$", "55"), false)
    };
    table[R4000AllegrexV4D::R600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R600", 56, concat!("$", "56"), false)
    };
    table[R4000AllegrexV4D::R601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R601", 57, concat!("$", "57"), false)
    };
    table[R4000AllegrexV4D::R602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R602", 58, concat!("$", "58"), false)
    };
    table[R4000AllegrexV4D::R603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R603", 59, concat!("$", "59"), false)
    };
    table[R4000AllegrexV4D::R700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R700", 60, concat!("$", "60"), false)
    };
    table[R4000AllegrexV4D::R701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R701", 61, concat!("$", "61"), false)
    };
    table[R4000AllegrexV4D::R702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R702", 62, concat!("$", "62"), false)
    };
    table[R4000AllegrexV4D::R703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R703", 63, concat!("$", "63"), false)
    };
    table[R4000AllegrexV4D::C002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C002", 64, concat!("$", "64"), false)
    };
    table[R4000AllegrexV4D::C012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C012", 65, concat!("$", "65"), false)
    };
    table[R4000AllegrexV4D::C022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C022", 66, concat!("$", "66"), false)
    };
    table[R4000AllegrexV4D::C032 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C032", 67, concat!("$", "67"), false)
    };
    table[R4000AllegrexV4D::C102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C102", 68, concat!("$", "68"), false)
    };
    table[R4000AllegrexV4D::C112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C112", 69, concat!("$", "69"), false)
    };
    table[R4000AllegrexV4D::C122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C122", 70, concat!("$", "70"), false)
    };
    table[R4000AllegrexV4D::C132 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C132", 71, concat!("$", "71"), false)
    };
    table[R4000AllegrexV4D::C202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C202", 72, concat!("$", "72"), false)
    };
    table[R4000AllegrexV4D::C212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C212", 73, concat!("$", "73"), false)
    };
    table[R4000AllegrexV4D::C222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C222", 74, concat!("$", "74"), false)
    };
    table[R4000AllegrexV4D::C232 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C232", 75, concat!("$", "75"), false)
    };
    table[R4000AllegrexV4D::C302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C302", 76, concat!("$", "76"), false)
    };
    table[R4000AllegrexV4D::C312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C312", 77, concat!("$", "77"), false)
    };
    table[R4000AllegrexV4D::C322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C322", 78, concat!("$", "78"), false)
    };
    table[R4000AllegrexV4D::C332 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C332", 79, concat!("$", "79"), false)
    };
    table[R4000AllegrexV4D::C402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C402", 80, concat!("$", "80"), false)
    };
    table[R4000AllegrexV4D::C412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C412", 81, concat!("$", "81"), false)
    };
    table[R4000AllegrexV4D::C422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C422", 82, concat!("$", "82"), false)
    };
    table[R4000AllegrexV4D::C432 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C432", 83, concat!("$", "83"), false)
    };
    table[R4000AllegrexV4D::C502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C502", 84, concat!("$", "84"), false)
    };
    table[R4000AllegrexV4D::C512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C512", 85, concat!("$", "85"), false)
    };
    table[R4000AllegrexV4D::C522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C522", 86, concat!("$", "86"), false)
    };
    table[R4000AllegrexV4D::C532 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C532", 87, concat!("$", "87"), false)
    };
    table[R4000AllegrexV4D::C602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C602", 88, concat!("$", "88"), false)
    };
    table[R4000AllegrexV4D::C612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C612", 89, concat!("$", "89"), false)
    };
    table[R4000AllegrexV4D::C622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C622", 90, concat!("$", "90"), false)
    };
    table[R4000AllegrexV4D::C632 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C632", 91, concat!("$", "91"), false)
    };
    table[R4000AllegrexV4D::C702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C702", 92, concat!("$", "92"), false)
    };
    table[R4000AllegrexV4D::C712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C712", 93, concat!("$", "93"), false)
    };
    table[R4000AllegrexV4D::C722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C722", 94, concat!("$", "94"), false)
    };
    table[R4000AllegrexV4D::C732 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C732", 95, concat!("$", "95"), false)
    };
    table[R4000AllegrexV4D::R020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R020", 96, concat!("$", "96"), false)
    };
    table[R4000AllegrexV4D::R021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R021", 97, concat!("$", "97"), false)
    };
    table[R4000AllegrexV4D::R022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R022", 98, concat!("$", "98"), false)
    };
    table[R4000AllegrexV4D::R023 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R023", 99, concat!("$", "99"), false)
    };
    table[R4000AllegrexV4D::R120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R120", 100, concat!("$", "100"), false)
    };
    table[R4000AllegrexV4D::R121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R121", 101, concat!("$", "101"), false)
    };
    table[R4000AllegrexV4D::R122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R122", 102, concat!("$", "102"), false)
    };
    table[R4000AllegrexV4D::R123 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R123", 103, concat!("$", "103"), false)
    };
    table[R4000AllegrexV4D::R220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R220", 104, concat!("$", "104"), false)
    };
    table[R4000AllegrexV4D::R221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R221", 105, concat!("$", "105"), false)
    };
    table[R4000AllegrexV4D::R222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R222", 106, concat!("$", "106"), false)
    };
    table[R4000AllegrexV4D::R223 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R223", 107, concat!("$", "107"), false)
    };
    table[R4000AllegrexV4D::R320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R320", 108, concat!("$", "108"), false)
    };
    table[R4000AllegrexV4D::R321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R321", 109, concat!("$", "109"), false)
    };
    table[R4000AllegrexV4D::R322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R322", 110, concat!("$", "110"), false)
    };
    table[R4000AllegrexV4D::R323 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R323", 111, concat!("$", "111"), false)
    };
    table[R4000AllegrexV4D::R420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R420", 112, concat!("$", "112"), false)
    };
    table[R4000AllegrexV4D::R421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R421", 113, concat!("$", "113"), false)
    };
    table[R4000AllegrexV4D::R422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R422", 114, concat!("$", "114"), false)
    };
    table[R4000AllegrexV4D::R423 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R423", 115, concat!("$", "115"), false)
    };
    table[R4000AllegrexV4D::R520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R520", 116, concat!("$", "116"), false)
    };
    table[R4000AllegrexV4D::R521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R521", 117, concat!("$", "117"), false)
    };
    table[R4000AllegrexV4D::R522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R522", 118, concat!("$", "118"), false)
    };
    table[R4000AllegrexV4D::R523 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R523", 119, concat!("$", "119"), false)
    };
    table[R4000AllegrexV4D::R620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R620", 120, concat!("$", "120"), false)
    };
    table[R4000AllegrexV4D::R621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R621", 121, concat!("$", "121"), false)
    };
    table[R4000AllegrexV4D::R622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R622", 122, concat!("$", "122"), false)
    };
    table[R4000AllegrexV4D::R623 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R623", 123, concat!("$", "123"), false)
    };
    table[R4000AllegrexV4D::R720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R720", 124, concat!("$", "124"), false)
    };
    table[R4000AllegrexV4D::R721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R721", 125, concat!("$", "125"), false)
    };
    table[R4000AllegrexV4D::R722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R722", 126, concat!("$", "126"), false)
    };
    table[R4000AllegrexV4D::R723 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R723", 127, concat!("$", "127"), false)
    };
    table
};
impl R4000AllegrexV4D {
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
            64 => Ok(Self::C002),
            65 => Ok(Self::C012),
            66 => Ok(Self::C022),
            67 => Ok(Self::C032),
            68 => Ok(Self::C102),
            69 => Ok(Self::C112),
            70 => Ok(Self::C122),
            71 => Ok(Self::C132),
            72 => Ok(Self::C202),
            73 => Ok(Self::C212),
            74 => Ok(Self::C222),
            75 => Ok(Self::C232),
            76 => Ok(Self::C302),
            77 => Ok(Self::C312),
            78 => Ok(Self::C322),
            79 => Ok(Self::C332),
            80 => Ok(Self::C402),
            81 => Ok(Self::C412),
            82 => Ok(Self::C422),
            83 => Ok(Self::C432),
            84 => Ok(Self::C502),
            85 => Ok(Self::C512),
            86 => Ok(Self::C522),
            87 => Ok(Self::C532),
            88 => Ok(Self::C602),
            89 => Ok(Self::C612),
            90 => Ok(Self::C622),
            91 => Ok(Self::C632),
            92 => Ok(Self::C702),
            93 => Ok(Self::C712),
            94 => Ok(Self::C722),
            95 => Ok(Self::C732),
            96 => Ok(Self::R020),
            97 => Ok(Self::R021),
            98 => Ok(Self::R022),
            99 => Ok(Self::R023),
            100 => Ok(Self::R120),
            101 => Ok(Self::R121),
            102 => Ok(Self::R122),
            103 => Ok(Self::R123),
            104 => Ok(Self::R220),
            105 => Ok(Self::R221),
            106 => Ok(Self::R222),
            107 => Ok(Self::R223),
            108 => Ok(Self::R320),
            109 => Ok(Self::R321),
            110 => Ok(Self::R322),
            111 => Ok(Self::R323),
            112 => Ok(Self::R420),
            113 => Ok(Self::R421),
            114 => Ok(Self::R422),
            115 => Ok(Self::R423),
            116 => Ok(Self::R520),
            117 => Ok(Self::R521),
            118 => Ok(Self::R522),
            119 => Ok(Self::R523),
            120 => Ok(Self::R620),
            121 => Ok(Self::R621),
            122 => Ok(Self::R622),
            123 => Ok(Self::R623),
            124 => Ok(Self::R720),
            125 => Ok(Self::R721),
            126 => Ok(Self::R722),
            127 => Ok(Self::R723),
            x => Err(IntRegisterConversionError::new_out_of_range(
                x,
                128,
                "R4000AllegrexV4D",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        128
    }
}
impl TryFrom<u32> for R4000AllegrexV4D {
    type Error = IntRegisterConversionError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexV4D {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexV4D> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexV4D) -> &Self::Output {
        &self[index as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R4000ALLEGREX_V4D {
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
        for (i, x) in R4000ALLEGREX_V4D.iter().enumerate() {
            assert!(x.value() as usize == i, "Broken register index?");
            x.check_valid_entry();
        }
    }
}
