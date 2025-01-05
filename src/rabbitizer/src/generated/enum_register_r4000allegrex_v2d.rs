/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexV2D {
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
pub static R4000ALLEGREX_V2D: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[R4000AllegrexV2D::C000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C000", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C010", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C020", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C030", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C100", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C110", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C120", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C130", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C200", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C210", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C220", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C230", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C300", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C310", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C320", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C330", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C400", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C410", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C420", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C430", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C500", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C510", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C520", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C530", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C600", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C610", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C620", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C630", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C700", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C710", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C720", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C730", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R000", 32, concat!("$", "32"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R001", 33, concat!("$", "33"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R002", 34, concat!("$", "34"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R003", 35, concat!("$", "35"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R100", 36, concat!("$", "36"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R101", 37, concat!("$", "37"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R102", 38, concat!("$", "38"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R103", 39, concat!("$", "39"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R200", 40, concat!("$", "40"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R201", 41, concat!("$", "41"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R202", 42, concat!("$", "42"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R203", 43, concat!("$", "43"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R300", 44, concat!("$", "44"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R301", 45, concat!("$", "45"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R302", 46, concat!("$", "46"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R303", 47, concat!("$", "47"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R400", 48, concat!("$", "48"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R401", 49, concat!("$", "49"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R402", 50, concat!("$", "50"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R403", 51, concat!("$", "51"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R500", 52, concat!("$", "52"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R501", 53, concat!("$", "53"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R502", 54, concat!("$", "54"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R503", 55, concat!("$", "55"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R600", 56, concat!("$", "56"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R601", 57, concat!("$", "57"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R602", 58, concat!("$", "58"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R603", 59, concat!("$", "59"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R700", 60, concat!("$", "60"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R701", 61, concat!("$", "61"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R702", 62, concat!("$", "62"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R703", 63, concat!("$", "63"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C002", 64, concat!("$", "64"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C012", 65, concat!("$", "65"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C022", 66, concat!("$", "66"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C032 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C032", 67, concat!("$", "67"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C102", 68, concat!("$", "68"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C112", 69, concat!("$", "69"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C122", 70, concat!("$", "70"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C132 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C132", 71, concat!("$", "71"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C202", 72, concat!("$", "72"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C212", 73, concat!("$", "73"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C222", 74, concat!("$", "74"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C232 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C232", 75, concat!("$", "75"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C302", 76, concat!("$", "76"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C312", 77, concat!("$", "77"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C322", 78, concat!("$", "78"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C332 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C332", 79, concat!("$", "79"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C402", 80, concat!("$", "80"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C412", 81, concat!("$", "81"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C422", 82, concat!("$", "82"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C432 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C432", 83, concat!("$", "83"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C502", 84, concat!("$", "84"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C512", 85, concat!("$", "85"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C522", 86, concat!("$", "86"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C532 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C532", 87, concat!("$", "87"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C602", 88, concat!("$", "88"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C612", 89, concat!("$", "89"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C622", 90, concat!("$", "90"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C632 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C632", 91, concat!("$", "91"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C702", 92, concat!("$", "92"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C712", 93, concat!("$", "93"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C722", 94, concat!("$", "94"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::C732 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C732", 95, concat!("$", "95"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R020", 96, concat!("$", "96"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R021", 97, concat!("$", "97"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R022", 98, concat!("$", "98"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R023 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R023", 99, concat!("$", "99"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R120", 100, concat!("$", "100"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R121", 101, concat!("$", "101"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R122", 102, concat!("$", "102"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R123 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R123", 103, concat!("$", "103"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R220", 104, concat!("$", "104"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R221", 105, concat!("$", "105"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R222", 106, concat!("$", "106"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R223 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R223", 107, concat!("$", "107"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R320", 108, concat!("$", "108"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R321", 109, concat!("$", "109"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R322", 110, concat!("$", "110"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R323 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R323", 111, concat!("$", "111"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R420", 112, concat!("$", "112"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R421", 113, concat!("$", "113"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R422", 114, concat!("$", "114"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R423 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R423", 115, concat!("$", "115"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R520", 116, concat!("$", "116"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R521", 117, concat!("$", "117"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R522", 118, concat!("$", "118"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R523 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R523", 119, concat!("$", "119"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R620", 120, concat!("$", "120"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R621", 121, concat!("$", "121"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R622", 122, concat!("$", "122"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R623 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R623", 123, concat!("$", "123"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R720", 124, concat!("$", "124"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R721", 125, concat!("$", "125"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R722", 126, concat!("$", "126"))
    }
    .check_panic_chain();
    table[R4000AllegrexV2D::R723 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R723", 127, concat!("$", "127"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexV2D {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
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
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 128,
                register_kind: "R4000AllegrexV2D",
            }),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        128
    }
}
impl TryFrom<u32> for R4000AllegrexV2D {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexV2D {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexV2D> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexV2D) -> &Self::Output {
        &self[index as usize]
    }
}
