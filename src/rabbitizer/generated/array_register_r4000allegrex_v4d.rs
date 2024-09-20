/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{RegisterDescriptor, RegisterR4000AllegrexV4D};
pub static R4000ALLEGREX_V4D_REGISTERS: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[RegisterR4000AllegrexV4D::C000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C000", 0, concat!("$", "0"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C010 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C010", 1, concat!("$", "1"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C020", 2, concat!("$", "2"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C030 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C030", 3, concat!("$", "3"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C100", 4, concat!("$", "4"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C110 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C110", 5, concat!("$", "5"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C120", 6, concat!("$", "6"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C130 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C130", 7, concat!("$", "7"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C200", 8, concat!("$", "8"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C210", 9, concat!("$", "9"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C220", 10, concat!("$", "10"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C230", 11, concat!("$", "11"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C300", 12, concat!("$", "12"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C310 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C310", 13, concat!("$", "13"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C320", 14, concat!("$", "14"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C330 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C330", 15, concat!("$", "15"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C400", 16, concat!("$", "16"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C410 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C410", 17, concat!("$", "17"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C420", 18, concat!("$", "18"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C430 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C430", 19, concat!("$", "19"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C500", 20, concat!("$", "20"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C510 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C510", 21, concat!("$", "21"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C520", 22, concat!("$", "22"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C530 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C530", 23, concat!("$", "23"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C600", 24, concat!("$", "24"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C610 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C610", 25, concat!("$", "25"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C620", 26, concat!("$", "26"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C630 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C630", 27, concat!("$", "27"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C700", 28, concat!("$", "28"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C710 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C710", 29, concat!("$", "29"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C720", 30, concat!("$", "30"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C730 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C730", 31, concat!("$", "31"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R000 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R000", 32, concat!("$", "32"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R001 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R001", 33, concat!("$", "33"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R002", 34, concat!("$", "34"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R003 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R003", 35, concat!("$", "35"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R100 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R100", 36, concat!("$", "36"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R101 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R101", 37, concat!("$", "37"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R102", 38, concat!("$", "38"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R103 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R103", 39, concat!("$", "39"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R200", 40, concat!("$", "40"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R201", 41, concat!("$", "41"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R202", 42, concat!("$", "42"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R203", 43, concat!("$", "43"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R300 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R300", 44, concat!("$", "44"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R301 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R301", 45, concat!("$", "45"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R302", 46, concat!("$", "46"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R303 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R303", 47, concat!("$", "47"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R400 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R400", 48, concat!("$", "48"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R401 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R401", 49, concat!("$", "49"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R402", 50, concat!("$", "50"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R403 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R403", 51, concat!("$", "51"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R500 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R500", 52, concat!("$", "52"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R501 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R501", 53, concat!("$", "53"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R502", 54, concat!("$", "54"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R503 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R503", 55, concat!("$", "55"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R600 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R600", 56, concat!("$", "56"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R601 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R601", 57, concat!("$", "57"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R602", 58, concat!("$", "58"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R603 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R603", 59, concat!("$", "59"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R700 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R700", 60, concat!("$", "60"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R701 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R701", 61, concat!("$", "61"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R702", 62, concat!("$", "62"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R703 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R703", 63, concat!("$", "63"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C002 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C002", 64, concat!("$", "64"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C012 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C012", 65, concat!("$", "65"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C022", 66, concat!("$", "66"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C032 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C032", 67, concat!("$", "67"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C102 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C102", 68, concat!("$", "68"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C112 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C112", 69, concat!("$", "69"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C122", 70, concat!("$", "70"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C132 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C132", 71, concat!("$", "71"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C202", 72, concat!("$", "72"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C212", 73, concat!("$", "73"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C222", 74, concat!("$", "74"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C232 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C232", 75, concat!("$", "75"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C302 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C302", 76, concat!("$", "76"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C312 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C312", 77, concat!("$", "77"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C322", 78, concat!("$", "78"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C332 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C332", 79, concat!("$", "79"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C402 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C402", 80, concat!("$", "80"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C412 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C412", 81, concat!("$", "81"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C422", 82, concat!("$", "82"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C432 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C432", 83, concat!("$", "83"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C502 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C502", 84, concat!("$", "84"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C512 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C512", 85, concat!("$", "85"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C522", 86, concat!("$", "86"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C532 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C532", 87, concat!("$", "87"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C602 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C602", 88, concat!("$", "88"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C612 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C612", 89, concat!("$", "89"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C622", 90, concat!("$", "90"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C632 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C632", 91, concat!("$", "91"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C702 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C702", 92, concat!("$", "92"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C712 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C712", 93, concat!("$", "93"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C722", 94, concat!("$", "94"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::C732 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("C732", 95, concat!("$", "95"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R020 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R020", 96, concat!("$", "96"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R021 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R021", 97, concat!("$", "97"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R022 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R022", 98, concat!("$", "98"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R023 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R023", 99, concat!("$", "99"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R120 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R120", 100, concat!("$", "100"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R121 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R121", 101, concat!("$", "101"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R122 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R122", 102, concat!("$", "102"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R123 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R123", 103, concat!("$", "103"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R220", 104, concat!("$", "104"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R221", 105, concat!("$", "105"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R222", 106, concat!("$", "106"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R223 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R223", 107, concat!("$", "107"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R320 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R320", 108, concat!("$", "108"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R321 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R321", 109, concat!("$", "109"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R322 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R322", 110, concat!("$", "110"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R323 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R323", 111, concat!("$", "111"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R420 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R420", 112, concat!("$", "112"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R421 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R421", 113, concat!("$", "113"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R422 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R422", 114, concat!("$", "114"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R423 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R423", 115, concat!("$", "115"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R520 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R520", 116, concat!("$", "116"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R521 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R521", 117, concat!("$", "117"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R522 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R522", 118, concat!("$", "118"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R523 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R523", 119, concat!("$", "119"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R620 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R620", 120, concat!("$", "120"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R621 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R621", 121, concat!("$", "121"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R622 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R622", 122, concat!("$", "122"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R623 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R623", 123, concat!("$", "123"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R720 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R720", 124, concat!("$", "124"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R721 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R721", 125, concat!("$", "125"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R722 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R722", 126, concat!("$", "126"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexV4D::R723 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("R723", 127, concat!("$", "127"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize == i);
        i += 1;
    }
    table
};
