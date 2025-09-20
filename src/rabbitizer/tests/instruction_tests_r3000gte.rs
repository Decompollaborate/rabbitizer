/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "R3000GTE")]
mod common;

#[cfg(feature = "R3000GTE")]
use common::{check_test_entries, TestEntry};

#[cfg(feature = "R3000GTE")]
use rabbitizer::{instr::InstructionFlags, isa::IsaExtension, opcodes::Opcode};

#[cfg(feature = "R3000GTE")]
#[test]
fn check_r3000gte_instructions() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A180001,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "rtps",
                Opcode::r3000gte_rtps,
                "rtps",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A280030,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "rtpt",
                Opcode::r3000gte_rtpt,
                "rtpt",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A680029,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "dpcl",
                Opcode::r3000gte_dpcl,
                "dpcl",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A780010,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "dpcs",
                Opcode::r3000gte_dpcs,
                "dpcs",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4AF8002A,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "dpct",
                Opcode::r3000gte_dpct,
                "dpct",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A980011,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "intpl",
                Opcode::r3000gte_intpl,
                "intpl",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4AC8041E,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "ncs",
                Opcode::r3000gte_ncs,
                "ncs",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4AD80420,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "nct",
                Opcode::r3000gte_nct,
                "nct",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4AE80413,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "ncds",
                Opcode::r3000gte_ncds,
                "ncds",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4AF80416,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "ncdt",
                Opcode::r3000gte_ncdt,
                "ncdt",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B08041B,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "nccs",
                Opcode::r3000gte_nccs,
                "nccs",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B18043F,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "ncct",
                Opcode::r3000gte_ncct,
                "ncct",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B280414,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "cdp",
                Opcode::r3000gte_cdp,
                "cdp",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B38041C,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "cc",
                Opcode::r3000gte_cc,
                "cc",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B400006,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "nclip",
                Opcode::r3000gte_nclip,
                "nclip",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B58002D,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "avsz3",
                Opcode::r3000gte_avsz3,
                "avsz3",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B68002E,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "avsz4",
                Opcode::r3000gte_avsz4,
                "avsz4",
                [Some(""), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A400012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       0, 0, 0, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("0"), Some("0"), Some("0"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4AA00428,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "sqr         0",
                Opcode::r3000gte_sqr,
                "sqr",
                [Some("0"), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B70000C,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "op          0",
                Opcode::r3000gte_op,
                "op",
                [Some("0"), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B90003D,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "gpf         0",
                Opcode::r3000gte_gpf,
                "gpf",
                [Some("0"), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4BA0003E,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "gpl         0",
                Opcode::r3000gte_gpl,
                "gpl",
                [Some("0"), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A486012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 0, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("0"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A48E012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 1, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("1"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A496012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 2, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("2"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A49E012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 3, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("3"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A41E012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       0, 0, 3, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("0"), Some("0"), Some("3"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A480012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 0, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("0"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A488012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 1, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("1"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A490012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 2, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("2"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A498012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 3, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("3"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A482012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 0, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("0"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A48A012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 1, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("1"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A492012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 2, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("2"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A49A012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 0, 3, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("0"), Some("3"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4A6412,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 0, 3, 1",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("0"), Some("3"), Some("1")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4A6012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 0, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("0"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4AE012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 1, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("1"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4B6012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 2, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("2"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4BE012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 3, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("3"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4A0012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 0, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("0"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4A8012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 1, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("1"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4B0012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 2, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("2"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4B8012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 3, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("3"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4A2012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 0, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("0"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4AA012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 1, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("1"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4B2012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 2, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("2"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4BA012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 1, 3, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("1"), Some("3"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4DA412,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 3, 1, 1",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("3"), Some("1"), Some("1")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4C6012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 0, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("0"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4CE012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 1, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("1"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4D6012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 2, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("2"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4DE012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 3, 3, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("3"), Some("3"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4C0012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 0, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("0"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4C8012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 1, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("1"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4D0012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 2, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("2"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4D8012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 3, 0, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("3"), Some("0"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4C2012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 0, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("0"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4CA012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 1, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("1"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4D2012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 2, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("2"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4A4DA012,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "mvmva       1, 2, 3, 1, 0",
                Opcode::r3000gte_mvmva,
                "mvmva",
                [Some("1"), Some("2"), Some("3"), Some("1"), Some("0")],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4AA80428,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "sqr         1",
                Opcode::r3000gte_sqr,
                "sqr",
                [Some("1"), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B78000C,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "op          1",
                Opcode::r3000gte_op,
                "op",
                [Some("1"), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4B98003D,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "gpf         1",
                Opcode::r3000gte_gpf,
                "gpf",
                [Some("1"), None, None, None, None],
            )
        },
        TestEntry {
            test_encoder: false,
            ..TestEntry::new(
                0x4BA8003E,
                InstructionFlags::new_extension(IsaExtension::R3000GTE),
                "gpl         1",
                Opcode::r3000gte_gpl,
                "gpl",
                [Some("1"), None, None, None, None],
            )
        },
    ];

    // We have to disable checking the encoder for GTE instructions because
    // those instructions can have multiple valid encodings, but currently the
    // encoder picks one of them, and it doesn't match the value from the tests.
    // TODO: add more tests that actually test encoding GTE instructions
    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}
