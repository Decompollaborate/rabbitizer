/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "R5900EE")]
mod common;

#[cfg(feature = "R5900EE")]
use common::{check_test_entries, TestEntry};

#[cfg(feature = "R5900EE")]
use address_space::Vram;

#[cfg(feature = "R5900EE")]
use rabbitizer::{
    abi::Abi,
    display_flags::InstructionDisplayFlags,
    instr::{Instruction, InstructionFlags},
    isa::IsaExtension,
    opcodes::Opcode,
};

#[cfg(feature = "R5900EE")]
#[test]
fn check_r5900ee_instructions() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry::new(
            0x4A000038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x0",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x0"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A004038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x800",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x800"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A008038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x1000",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x1000"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A008838,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x1100",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x1100"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A009038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x1200",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x1200"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A009838,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x1300",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x1300"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A00A038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x1400",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x1400"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A07FFF8,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0xFFF8",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0xFFF8"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A080038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x10000",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x10000"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A1F8038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x3F000",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x3F000"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A1FFFB8,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x3FFF0",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x3FFF0"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A1FFFF8,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x3FFF8",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x3FFF8"), None, None, None, None],
        ),
        TestEntry::new(
            0x70001030,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "pmfhl.lw    $v0",
            Opcode::r5900ee_pmfhl_lw,
            "pmfhl.lw",
            [Some("$v0"), None, None, None, None],
        ),
        TestEntry::new(
            0x70001070,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "pmfhl.uw    $v0",
            Opcode::r5900ee_pmfhl_uw,
            "pmfhl.uw",
            [Some("$v0"), None, None, None, None],
        ),
        TestEntry::new(
            0x700010B0,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "pmfhl.slw   $v0",
            Opcode::r5900ee_pmfhl_slw,
            "pmfhl.slw",
            [Some("$v0"), None, None, None, None],
        ),
        TestEntry::new(
            0x700010F0,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "pmfhl.lh    $v0",
            Opcode::r5900ee_pmfhl_lh,
            "pmfhl.lh",
            [Some("$v0"), None, None, None, None],
        ),
        TestEntry::new(
            0x70001130,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "pmfhl.sh    $v0",
            Opcode::r5900ee_pmfhl_sh,
            "pmfhl.sh",
            [Some("$v0"), None, None, None, None],
        ),
        TestEntry::new(
            0x70000031,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "pmthl.lw    $zero",
            Opcode::r5900ee_pmthl_lw,
            "pmthl.lw",
            [Some("$zero"), None, None, None, None],
        ),
        TestEntry::new(
            0x4B020BFE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vilwr.x     $vi2, ($vi1)",
            Opcode::r5900ee_vilwr_x,
            "vilwr.x",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x4A820BFE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vilwr.y     $vi2, ($vi1)",
            Opcode::r5900ee_vilwr_y,
            "vilwr.y",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x4A420BFE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vilwr.z     $vi2, ($vi1)",
            Opcode::r5900ee_vilwr_z,
            "vilwr.z",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x4A220BFE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vilwr.w     $vi2, ($vi1)",
            Opcode::r5900ee_vilwr_w,
            "vilwr.w",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x4B020BFF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viswr.x     $vi2, ($vi1)",
            Opcode::r5900ee_viswr_x,
            "viswr.x",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x4A820BFF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viswr.y     $vi2, ($vi1)",
            Opcode::r5900ee_viswr_y,
            "viswr.y",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x4A420BFF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viswr.z     $vi2, ($vi1)",
            Opcode::r5900ee_viswr_z,
            "viswr.z",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x4A220BFF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viswr.w     $vi2, ($vi1)",
            Opcode::r5900ee_viswr_w,
            "viswr.w",
            [Some("$vi2"), Some("($vi1)"), None, None, None],
        ),
        TestEntry::new(
            0x70111334,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "psllh       $v0, $s1, 12",
            Opcode::r5900ee_psllh,
            "psllh",
            [Some("$v0"), Some("$s1"), Some("12"), None, None],
        ),
        TestEntry::new(
            0x70111336,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "psrlh       $v0, $s1, 12",
            Opcode::r5900ee_psrlh,
            "psrlh",
            [Some("$v0"), Some("$s1"), Some("12"), None, None],
        ),
        TestEntry::new(
            0x70111337,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "psrah       $v0, $s1, 12",
            Opcode::r5900ee_psrah,
            "psrah",
            [Some("$v0"), Some("$s1"), Some("12"), None, None],
        ),
        TestEntry::new(
            0x7011133C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "psllw       $v0, $s1, 12",
            Opcode::r5900ee_psllw,
            "psllw",
            [Some("$v0"), Some("$s1"), Some("12"), None, None],
        ),
        TestEntry::new(
            0x7011133E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "psrlw       $v0, $s1, 12",
            Opcode::r5900ee_psrlw,
            "psrlw",
            [Some("$v0"), Some("$s1"), Some("12"), None, None],
        ),
        TestEntry::new(
            0x7011133F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "psraw       $v0, $s1, 12",
            Opcode::r5900ee_psraw,
            "psraw",
            [Some("$v0"), Some("$s1"), Some("12"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A00551D,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vmaxi       $vf20, $vf10, $I",
            expected_opcode: Opcode::r5900ee_vmaxi,
            opcode_str: "vmaxi",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("$I"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00551D,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vmaxi       $vf20, $vf10, I",
            expected_opcode: Opcode::r5900ee_vmaxi,
            opcode_str: "vmaxi",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("I"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00551C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vmulq       $vf20, $vf10, $Q",
            expected_opcode: Opcode::r5900ee_vmulq,
            opcode_str: "vmulq",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("$Q"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00551C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vmulq       $vf20, $vf10, Q",
            expected_opcode: Opcode::r5900ee_vmulq,
            opcode_str: "vmulq",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("Q"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06043C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vrnext      $vf6, $R",
            expected_opcode: Opcode::r5900ee_vrnext,
            opcode_str: "vrnext",
            operands_str: [Some("$vf6"), Some("$R"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06043C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vrnext      $vf6, R",
            expected_opcode: Opcode::r5900ee_vrnext,
            opcode_str: "vrnext",
            operands_str: [Some("$vf6"), Some("R"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06003C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vaddax      $ACC, $vf0, $vf6x",
            expected_opcode: Opcode::r5900ee_vaddax,
            opcode_str: "vaddax",
            operands_str: [Some("$ACC"), Some("$vf0"), Some("$vf6x"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06003C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vaddax      ACC, $vf0, $vf6x",
            expected_opcode: Opcode::r5900ee_vaddax,
            opcode_str: "vaddax",
            operands_str: [Some("ACC"), Some("$vf0"), Some("$vf6x"), None, None],
            test_encoder: true,
        },
        TestEntry::new(
            0x4A0663BC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vdiv        Q, $vf12x, $vf6x",
            Opcode::r5900ee_vdiv,
            "vdiv",
            [Some("Q"), Some("$vf12x"), Some("$vf6x"), None, None],
        ),
        TestEntry::new(
            0x4A066630,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viadd       $vi24, $vi12, $vi6",
            Opcode::r5900ee_viadd,
            "viadd",
            [Some("$vi24"), Some("$vi12"), Some("$vi6"), None, None],
        ),
        TestEntry::new(
            0x4A066632,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viaddi      $vi6, $vi12, -0x8",
            Opcode::r5900ee_viaddi,
            "viaddi",
            [Some("$vi6"), Some("$vi12"), Some("-0x8"), None, None],
        ),
        TestEntry::new(
            0x4A06637E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vlqd        $vf6, (--$vi12)",
            Opcode::r5900ee_vlqd,
            "vlqd",
            [Some("$vf6"), Some("(--$vi12)"), None, None, None],
        ),
        TestEntry::new(
            0x4A06637F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqd        $vf12, (--$vi6)",
            Opcode::r5900ee_vsqd,
            "vsqd",
            [Some("$vf12"), Some("(--$vi6)"), None, None, None],
        ),
        TestEntry::new(
            0x4A06637C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vlqi        $vf6, ($vi12++)",
            Opcode::r5900ee_vlqi,
            "vlqi",
            [Some("$vf6"), Some("($vi12++)"), None, None, None],
        ),
        TestEntry::new(
            0x4A06637D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqi        $vf12, ($vi6++)",
            Opcode::r5900ee_vsqi,
            "vsqi",
            [Some("$vf12"), Some("($vi6++)"), None, None, None],
        ),
        TestEntry::new(
            0x4A0307B2,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viaddi      $vi3, $vi0, -0x2",
            Opcode::r5900ee_viaddi,
            "viaddi",
            [Some("$vi3"), Some("$vi0"), Some("-0x2"), None, None],
        ),
        TestEntry::new(
            0x4A0303B2,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viaddi      $vi3, $vi0, 0xE",
            Opcode::r5900ee_viaddi,
            "viaddi",
            [Some("$vi3"), Some("$vi0"), Some("0xE"), None, None],
        ),
        TestEntry::new(
            0x48300800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmfc2.ni    $s0, $vf1",
            Opcode::r5900ee_qmfc2_ni,
            "qmfc2.ni",
            [Some("$s0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0x48300801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmfc2.i     $s0, $vf1",
            Opcode::r5900ee_qmfc2_i,
            "qmfc2.i",
            [Some("$s0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0x48500800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "cfc2.ni     $s0, $vi1",
            Opcode::r5900ee_cfc2_ni,
            "cfc2.ni",
            [Some("$s0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0x48500801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "cfc2.i      $s0, $vi1",
            Opcode::r5900ee_cfc2_i,
            "cfc2.i",
            [Some("$s0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0x48B00800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmtc2.ni    $s0, $vf1",
            Opcode::r5900ee_qmtc2_ni,
            "qmtc2.ni",
            [Some("$s0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0x48B00801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmtc2.i     $s0, $vf1",
            Opcode::r5900ee_qmtc2_i,
            "qmtc2.i",
            [Some("$s0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0x48D00800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "ctc2.ni     $s0, $vi1",
            Opcode::r5900ee_ctc2_ni,
            "ctc2.ni",
            [Some("$s0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0x48D00801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "ctc2.i      $s0, $vi1",
            Opcode::r5900ee_ctc2_i,
            "ctc2.i",
            [Some("$s0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0xD8771234,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "lqc2        $vf23, 0x1234($v1)",
            Opcode::r5900ee_lqc2,
            "lqc2",
            [Some("$vf23"), Some("0x1234($v1)"), None, None, None],
        ),
        TestEntry::new(
            0xF8661234,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "sqc2        $vf6, 0x1234($v1)",
            Opcode::r5900ee_sqc2,
            "sqc2",
            [Some("$vf6"), Some("0x1234($v1)"), None, None, None],
        ),
        TestEntry::new(
            0x78771234,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "lq          $s7, 0x1234($v1)",
            Opcode::r5900ee_lq,
            "lq",
            [Some("$s7"), Some("0x1234($v1)"), None, None, None],
        ),
        TestEntry::new(
            0x7C661234,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "sq          $a2, 0x1234($v1)",
            Opcode::r5900ee_sq,
            "sq",
            [Some("$a2"), Some("0x1234($v1)"), None, None, None],
        ),
        TestEntry::new(
            0x4A003ABC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vadda       ACC, $vf0, $vf7",
            Opcode::r5900ee_vadda,
            "vadda",
            [Some("ACC"), Some("$vf0"), Some("$vf7"), None, None],
        ),
        TestEntry::new(
            0x4BC03ABC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vadda.xyz   ACC, $vf0, $vf7",
            Opcode::r5900ee_vadda,
            "vadda.xyz",
            [Some("ACC"), Some("$vf0"), Some("$vf7"), None, None],
        ),
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}

#[cfg(feature = "R5900EE")]
#[test]
fn check_r5900ee_trunc_cvt_instructions() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry {
            instr: Instruction::new(0x4600600D, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: ".word       0x4600600D                   /* trunc.w.s   $fv0, $fa0 / 00000000 <OpcodeCategory: CORE_COP1_FPUS> */",
            expected_opcode: Opcode::core_trunc_w_s,
            opcode_str: "trunc.w.s",
            operands_str: [Some("$fv0"), Some("$fa0"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(0x46006024, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: ".word       0x46006024                   /* cvt.w.s     $fv0, $fa0 / 00000000 <OpcodeCategory: CORE_COP1_FPUS> */",
            expected_opcode: Opcode::core_cvt_w_s,
            opcode_str: "cvt.w.s",
            operands_str: [Some("$fv0"), Some("$fa0"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(0x4600600D, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "trunc.w.s   $f0, $f12",
            expected_opcode: Opcode::core_trunc_w_s,
            opcode_str: "trunc.w.s",
            operands_str: [Some("$f0"), Some("$f12"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(0x46006024, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "cvt.w.s     $f0, $f12",
            expected_opcode: Opcode::core_cvt_w_s,
            opcode_str: "cvt.w.s",
            operands_str: [Some("$f0"), Some("$f12"), None, None, None],
            test_encoder: true,
        },
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}

#[cfg(feature = "R5900EE")]
#[test]
fn check_r5900ee_more() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry::new(
            0x49000007,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2f        . + 4 + (0x7 << 2)",
            Opcode::r5900ee_bc2f,
            "bc2f",
            [Some(". + 4 + (0x7 << 2)"), None, None, None, None],
        ),
        TestEntry::new(
            0x49020005,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2fl       . + 4 + (0x5 << 2)",
            Opcode::r5900ee_bc2fl,
            "bc2fl",
            [Some(". + 4 + (0x5 << 2)"), None, None, None, None],
        ),
        TestEntry::new(
            0x49010003,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2t        . + 4 + (0x3 << 2)",
            Opcode::r5900ee_bc2t,
            "bc2t",
            [Some(". + 4 + (0x3 << 2)"), None, None, None, None],
        ),
        TestEntry::new(
            0x49030001,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2tl       . + 4 + (0x1 << 2)",
            Opcode::r5900ee_bc2tl,
            "bc2tl",
            [Some(". + 4 + (0x1 << 2)"), None, None, None, None],
        ),
        TestEntry::new(
            0x48440800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "cfc2.ni     $a0, $vi1",
            Opcode::r5900ee_cfc2_ni,
            "cfc2.ni",
            [Some("$a0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0x48440801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "cfc2.i      $a0, $vi1",
            Opcode::r5900ee_cfc2_i,
            "cfc2.i",
            [Some("$a0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0x48C40800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "ctc2.ni     $a0, $vi1",
            Opcode::r5900ee_ctc2_ni,
            "ctc2.ni",
            [Some("$a0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0x48C40801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "ctc2.i      $a0, $vi1",
            Opcode::r5900ee_ctc2_i,
            "ctc2.i",
            [Some("$a0"), Some("$vi1"), None, None, None],
        ),
        TestEntry::new(
            0xD8810000,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "lqc2        $vf1, 0x0($a0)",
            Opcode::r5900ee_lqc2,
            "lqc2",
            [Some("$vf1"), Some("0x0($a0)"), None, None, None],
        ),
        TestEntry::new(
            0x48240800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmfc2.ni    $a0, $vf1",
            Opcode::r5900ee_qmfc2_ni,
            "qmfc2.ni",
            [Some("$a0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0x48240801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmfc2.i     $a0, $vf1",
            Opcode::r5900ee_qmfc2_i,
            "qmfc2.i",
            [Some("$a0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0x48A40800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmtc2.ni    $a0, $vf1",
            Opcode::r5900ee_qmtc2_ni,
            "qmtc2.ni",
            [Some("$a0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0x48A40801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmtc2.i     $a0, $vf1",
            Opcode::r5900ee_qmtc2_i,
            "qmtc2.i",
            [Some("$a0"), Some("$vf1"), None, None, None],
        ),
        TestEntry::new(
            0xF8810000,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "sqc2        $vf1, 0x0($a0)",
            Opcode::r5900ee_sqc2,
            "sqc2",
            [Some("$vf1"), Some("0x0($a0)"), None, None, None],
        ),
        TestEntry::new(
            0x4BE111FD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vabs.xyzw   $vf1, $vf2",
            Opcode::r5900ee_vabs,
            "vabs.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE31068,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vadd.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vadd,
            "vadd.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE01062,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddi.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vaddi,
            "vaddi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE01060,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddq.xyzw  $vf1, $vf2, Q",
            Opcode::r5900ee_vaddq,
            "vaddq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE31040,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vaddx,
            "vaddx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE21ABC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vadda.xyzw  ACC, $vf2, $vf3",
            Opcode::r5900ee_vadda,
            "vadda.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE0123E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vaddai,
            "vaddai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE0123C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vaddaq,
            "vaddaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE3103C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vaddax,
            "vaddax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4A000038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x0",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x0"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A00D839,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallmsr    $vi27",
            Opcode::r5900ee_vcallmsr,
            "vcallmsr",
            [Some("$vi27"), None, None, None, None],
        ),
        TestEntry::new(
            0x4A0313BC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vdiv        Q, $vf2x, $vf3x",
            Opcode::r5900ee_vdiv,
            "vdiv",
            [Some("Q"), Some("$vf2x"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE1117C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi0.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi0,
            "vftoi0.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1117D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi4.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi4,
            "vftoi4.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1117E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi12.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi12,
            "vftoi12.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1117F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi15.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi15,
            "vftoi15.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4A031070,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viadd       $vi1, $vi2, $vi3",
            Opcode::r5900ee_viadd,
            "viadd",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        ),
        TestEntry::new(
            0x4A0110F2,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viaddi      $vi1, $vi2, 0x3",
            Opcode::r5900ee_viaddi,
            "viaddi",
            [Some("$vi1"), Some("$vi2"), Some("0x3"), None, None],
        ),
        TestEntry::new(
            0x4A031074,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viand       $vi1, $vi2, $vi3",
            Opcode::r5900ee_viand,
            "viand",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        ),
        TestEntry::new(
            0x4B0113FE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vilwr.x     $vi1, ($vi2)",
            Opcode::r5900ee_vilwr_x,
            "vilwr.x",
            [Some("$vi1"), Some("($vi2)"), None, None, None],
        ),
        TestEntry::new(
            0x4A031075,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vior        $vi1, $vi2, $vi3",
            Opcode::r5900ee_vior,
            "vior",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        ),
        TestEntry::new(
            0x4A031071,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "visub       $vi1, $vi2, $vi3",
            Opcode::r5900ee_visub,
            "visub",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        ),
        TestEntry::new(
            0x4B0113FF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viswr.x     $vi1, ($vi2)",
            Opcode::r5900ee_viswr_x,
            "viswr.x",
            [Some("$vi1"), Some("($vi2)"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1113C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof0.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof0,
            "vitof0.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1113D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof4.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof4,
            "vitof4.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1113E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof12.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof12,
            "vitof12.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1113F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof15.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof15,
            "vitof15.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1137E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vlqd.xyzw   $vf1, (--$vi2)",
            Opcode::r5900ee_vlqd,
            "vlqd.xyzw",
            [Some("$vf1"), Some("(--$vi2)"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1137C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vlqi.xyzw   $vf1, ($vi2++)",
            Opcode::r5900ee_vlqi,
            "vlqi.xyzw",
            [Some("$vf1"), Some("($vi2++)"), None, None, None],
        ),
        TestEntry::new(
            0x4BE31069,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmadd.xyzw  $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmadd,
            "vmadd.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE01063,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddi.xyzw $vf1, $vf2, I",
            Opcode::r5900ee_vmaddi,
            "vmaddi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE01061,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddq.xyzw $vf1, $vf2, Q",
            Opcode::r5900ee_vmaddq,
            "vmaddq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE31048,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddx.xyzw $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmaddx,
            "vmaddx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE312BD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmadda.xyzw ACC, $vf2, $vf3",
            Opcode::r5900ee_vmadda,
            "vmadda.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE0123F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vmaddai,
            "vmaddai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE0123D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vmaddaq,
            "vmaddaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE310BC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vmaddax,
            "vmaddax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE3106B,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmax.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmax,
            "vmax.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE0105D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaxi.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vmaxi,
            "vmaxi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE31050,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaxx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmaxx,
            "vmaxx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE113FD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmfir.xyzw  $vf1, $vi2",
            Opcode::r5900ee_vmfir,
            "vmfir.xyzw",
            [Some("$vf1"), Some("$vi2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE3106F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmini.xyzw  $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmini,
            "vmini.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE0105F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vminii.xyzw $vf1, $vf2, I",
            Opcode::r5900ee_vminii,
            "vminii.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE31054,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vminix.xyzw $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vminix,
            "vminix.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE1133C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmove.xyzw  $vf1, $vf2",
            Opcode::r5900ee_vmove,
            "vmove.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1133D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmr32.xyzw  $vf1, $vf2",
            Opcode::r5900ee_vmr32,
            "vmr32.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        ),
        TestEntry::new(
            0x4BE3106D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsub.xyzw  $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmsub,
            "vmsub.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE01067,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubi.xyzw $vf1, $vf2, I",
            Opcode::r5900ee_vmsubi,
            "vmsubi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE01065,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubq.xyzw $vf1, $vf2, Q",
            Opcode::r5900ee_vmsubq,
            "vmsubq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE3104C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubx.xyzw $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmsubx,
            "vmsubx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE21AFD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsuba.xyzw ACC, $vf2, $vf3",
            Opcode::r5900ee_vmsuba,
            "vmsuba.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE0127F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vmsubai,
            "vmsubai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE0127D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vmsubaq,
            "vmsubaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE310FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vmsubax,
            "vmsubax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4A0113FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmtir       $vi1, $vf2x",
            Opcode::r5900ee_vmtir,
            "vmtir",
            [Some("$vi1"), Some("$vf2x"), None, None, None],
        ),
        TestEntry::new(
            0x4BE3106A,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmul.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmul,
            "vmul.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE0105E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmuli.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vmuli,
            "vmuli.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE0105C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulq.xyzw  $vf1, $vf2, Q",
            Opcode::r5900ee_vmulq,
            "vmulq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE31058,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmulx,
            "vmulx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE312BE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmula.xyzw  ACC, $vf2, $vf3",
            Opcode::r5900ee_vmula,
            "vmula.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE011FE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vmulai,
            "vmulai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE011FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vmulaq,
            "vmulaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE311BC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vmulax,
            "vmulax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4A0002FF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vnop",
            Opcode::r5900ee_vnop,
            "vnop",
            [None, None, None, None, None],
        ),
        TestEntry::new(
            0x4BC312FE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vopmula.xyz ACC, $vf2, $vf3",
            Opcode::r5900ee_vopmula,
            "vopmula.xyz",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BC3106E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vopmsub.xyz $vf1, $vf2, $vf3",
            Opcode::r5900ee_vopmsub,
            "vopmsub.xyz",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE1043D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrget.xyzw  $vf1, R",
            Opcode::r5900ee_vrget,
            "vrget.xyzw",
            [Some("$vf1"), Some("R"), None, None, None],
        ),
        TestEntry::new(
            0x4A00143E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrinit      R, $vf2x",
            Opcode::r5900ee_vrinit,
            "vrinit",
            [Some("R"), Some("$vf2x"), None, None, None],
        ),
        TestEntry::new(
            0x4BE1043C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrnext.xyzw $vf1, R",
            Opcode::r5900ee_vrnext,
            "vrnext.xyzw",
            [Some("$vf1"), Some("R"), None, None, None],
        ),
        TestEntry::new(
            0x4A0313BE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrsqrt      Q, $vf2x, $vf3x",
            Opcode::r5900ee_vrsqrt,
            "vrsqrt",
            [Some("Q"), Some("$vf2x"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4A00143F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrxor       R, $vf2x",
            Opcode::r5900ee_vrxor,
            "vrxor",
            [Some("R"), Some("$vf2x"), None, None, None],
        ),
        TestEntry::new(
            0x4BE20B7F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqd.xyzw   $vf1, (--$vi2)",
            Opcode::r5900ee_vsqd,
            "vsqd.xyzw",
            [Some("$vf1"), Some("(--$vi2)"), None, None, None],
        ),
        TestEntry::new(
            0x4BE20B7D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqi.xyzw   $vf1, ($vi2++)",
            Opcode::r5900ee_vsqi,
            "vsqi.xyzw",
            [Some("$vf1"), Some("($vi2++)"), None, None, None],
        ),
        TestEntry::new(
            0x4A0203BD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqrt       Q, $vf2x",
            Opcode::r5900ee_vsqrt,
            "vsqrt",
            [Some("Q"), Some("$vf2x"), None, None, None],
        ),
        TestEntry::new(
            0x4BE3106C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsub.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vsub,
            "vsub.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE01066,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubi.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vsubi,
            "vsubi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE01064,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubq.xyzw  $vf1, $vf2, Q",
            Opcode::r5900ee_vsubq,
            "vsubq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE31044,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vsubx,
            "vsubx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4BE312FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsuba.xyzw  ACC, $vf2, $vf3",
            Opcode::r5900ee_vsuba,
            "vsuba.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        ),
        TestEntry::new(
            0x4BE0127E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vsubai,
            "vsubai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        ),
        TestEntry::new(
            0x4BE0127C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vsubaq,
            "vsubaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        ),
        TestEntry::new(
            0x4BE3107C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vsubax,
            "vsubax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        ),
        TestEntry::new(
            0x4A0003BF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vwaitq",
            Opcode::r5900ee_vwaitq,
            "vwaitq",
            [None, None, None, None, None],
        ),
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}

#[cfg(feature = "R5900EE")]
#[test]
fn check_r5900ee_prodg_sn_as_inverted_regs() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry::new(
            0x49000007,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2f        . + 4 + (0x7 << 2)",
            Opcode::r5900ee_bc2f,
            "bc2f",
            [Some(". + 4 + (0x7 << 2)"), None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x49020005,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2fl       . + 4 + (0x5 << 2)",
            Opcode::r5900ee_bc2fl,
            "bc2fl",
            [Some(". + 4 + (0x5 << 2)"), None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x49010003,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2t        . + 4 + (0x3 << 2)",
            Opcode::r5900ee_bc2t,
            "bc2t",
            [Some(". + 4 + (0x3 << 2)"), None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x49030001,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "bc2tl       . + 4 + (0x1 << 2)",
            Opcode::r5900ee_bc2tl,
            "bc2tl",
            [Some(". + 4 + (0x1 << 2)"), None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48440800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "cfc2.ni     $a0, $vi1",
            Opcode::r5900ee_cfc2_ni,
            "cfc2.ni",
            [Some("$a0"), Some("$vi1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48440801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "cfc2.i      $a0, $vi1",
            Opcode::r5900ee_cfc2_i,
            "cfc2.i",
            [Some("$a0"), Some("$vi1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48C40800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "ctc2.ni     $a0, $vi1",
            Opcode::r5900ee_ctc2_ni,
            "ctc2.ni",
            [Some("$a0"), Some("$vi1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48C40801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "ctc2.i      $a0, $vi1",
            Opcode::r5900ee_ctc2_i,
            "ctc2.i",
            [Some("$a0"), Some("$vi1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0xD8810000,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "lqc2        $vf1, 0x0($a0)",
            Opcode::r5900ee_lqc2,
            "lqc2",
            [Some("$vf1"), Some("0x0($a0)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48240800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmfc2.ni    $a0, $vf1",
            Opcode::r5900ee_qmfc2_ni,
            "qmfc2.ni",
            [Some("$a0"), Some("$vf1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48240801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmfc2.i     $a0, $vf1",
            Opcode::r5900ee_qmfc2_i,
            "qmfc2.i",
            [Some("$a0"), Some("$vf1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48A40800,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmtc2.ni    $a0, $vf1",
            Opcode::r5900ee_qmtc2_ni,
            "qmtc2.ni",
            [Some("$a0"), Some("$vf1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x48A40801,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "qmtc2.i     $a0, $vf1",
            Opcode::r5900ee_qmtc2_i,
            "qmtc2.i",
            [Some("$a0"), Some("$vf1"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0xF8810000,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "sqc2        $vf1, 0x0($a0)",
            Opcode::r5900ee_sqc2,
            "sqc2",
            [Some("$vf1"), Some("0x0($a0)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE111FD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vabs.xyzw   $vf1, $vf2",
            Opcode::r5900ee_vabs,
            "vabs.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31068,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vadd.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vadd,
            "vadd.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01062,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddi.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vaddi,
            "vaddi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01060,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddq.xyzw  $vf1, $vf2, Q",
            Opcode::r5900ee_vaddq,
            "vaddq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31040,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vaddx,
            "vaddx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE21ABC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vadda.xyzw  ACC, $vf3, $vf2",
            Opcode::r5900ee_vadda,
            "vadda.xyzw",
            [Some("ACC"), Some("$vf3"), Some("$vf2"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0123E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vaddai,
            "vaddai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0123C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vaddaq,
            "vaddaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3103C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vaddax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vaddax,
            "vaddax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A000038,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallms     0x0",
            Opcode::r5900ee_vcallms,
            "vcallms",
            [Some("0x0"), None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A00D839,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vcallmsr    $vi27",
            Opcode::r5900ee_vcallmsr,
            "vcallmsr",
            [Some("$vi27"), None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A0313BC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vdiv        Q, $vf2x, $vf3x",
            Opcode::r5900ee_vdiv,
            "vdiv",
            [Some("Q"), Some("$vf2x"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1117C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi0.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi0,
            "vftoi0.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1117D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi4.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi4,
            "vftoi4.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1117E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi12.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi12,
            "vftoi12.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1117F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vftoi15.xyzw $vf1, $vf2",
            Opcode::r5900ee_vftoi15,
            "vftoi15.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A031070,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viadd       $vi1, $vi2, $vi3",
            Opcode::r5900ee_viadd,
            "viadd",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A0110F2,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viaddi      $vi1, $vi2, 0x3",
            Opcode::r5900ee_viaddi,
            "viaddi",
            [Some("$vi1"), Some("$vi2"), Some("0x3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A031074,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viand       $vi1, $vi2, $vi3",
            Opcode::r5900ee_viand,
            "viand",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4B0113FE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vilwr.x     $vi1, ($vi2)",
            Opcode::r5900ee_vilwr_x,
            "vilwr.x",
            [Some("$vi1"), Some("($vi2)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A031075,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vior        $vi1, $vi2, $vi3",
            Opcode::r5900ee_vior,
            "vior",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A031071,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "visub       $vi1, $vi2, $vi3",
            Opcode::r5900ee_visub,
            "visub",
            [Some("$vi1"), Some("$vi2"), Some("$vi3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4B0113FF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "viswr.x     $vi1, ($vi2)",
            Opcode::r5900ee_viswr_x,
            "viswr.x",
            [Some("$vi1"), Some("($vi2)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1113C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof0.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof0,
            "vitof0.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1113D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof4.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof4,
            "vitof4.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1113E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof12.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof12,
            "vitof12.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1113F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vitof15.xyzw $vf1, $vf2",
            Opcode::r5900ee_vitof15,
            "vitof15.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1137E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vlqd.xyzw   $vf1, (--$vi2)",
            Opcode::r5900ee_vlqd,
            "vlqd.xyzw",
            [Some("$vf1"), Some("(--$vi2)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1137C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vlqi.xyzw   $vf1, ($vi2++)",
            Opcode::r5900ee_vlqi,
            "vlqi.xyzw",
            [Some("$vf1"), Some("($vi2++)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31069,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmadd.xyzw  $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmadd,
            "vmadd.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01063,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddi.xyzw $vf1, $vf2, I",
            Opcode::r5900ee_vmaddi,
            "vmaddi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01061,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddq.xyzw $vf1, $vf2, Q",
            Opcode::r5900ee_vmaddq,
            "vmaddq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31048,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddx.xyzw $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmaddx,
            "vmaddx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE312BD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmadda.xyzw ACC, $vf2, $vf3",
            Opcode::r5900ee_vmadda,
            "vmadda.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0123F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vmaddai,
            "vmaddai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0123D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vmaddaq,
            "vmaddaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE310BC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaddax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vmaddax,
            "vmaddax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3106B,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmax.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmax,
            "vmax.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0105D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaxi.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vmaxi,
            "vmaxi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31050,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmaxx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmaxx,
            "vmaxx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE113FD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmfir.xyzw  $vf1, $vi2",
            Opcode::r5900ee_vmfir,
            "vmfir.xyzw",
            [Some("$vf1"), Some("$vi2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3106F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmini.xyzw  $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmini,
            "vmini.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0105F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vminii.xyzw $vf1, $vf2, I",
            Opcode::r5900ee_vminii,
            "vminii.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31054,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vminix.xyzw $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vminix,
            "vminix.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1133C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmove.xyzw  $vf1, $vf2",
            Opcode::r5900ee_vmove,
            "vmove.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1133D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmr32.xyzw  $vf1, $vf2",
            Opcode::r5900ee_vmr32,
            "vmr32.xyzw",
            [Some("$vf1"), Some("$vf2"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3106D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsub.xyzw  $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmsub,
            "vmsub.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01067,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubi.xyzw $vf1, $vf2, I",
            Opcode::r5900ee_vmsubi,
            "vmsubi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01065,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubq.xyzw $vf1, $vf2, Q",
            Opcode::r5900ee_vmsubq,
            "vmsubq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3104C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubx.xyzw $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmsubx,
            "vmsubx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE21AFD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsuba.xyzw ACC, $vf3, $vf2",
            Opcode::r5900ee_vmsuba,
            "vmsuba.xyzw",
            [Some("ACC"), Some("$vf3"), Some("$vf2"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0127F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vmsubai,
            "vmsubai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0127D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vmsubaq,
            "vmsubaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE310FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmsubax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vmsubax,
            "vmsubax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A0113FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmtir       $vi1, $vf2x",
            Opcode::r5900ee_vmtir,
            "vmtir",
            [Some("$vi1"), Some("$vf2x"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3106A,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmul.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vmul,
            "vmul.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0105E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmuli.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vmuli,
            "vmuli.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0105C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulq.xyzw  $vf1, $vf2, Q",
            Opcode::r5900ee_vmulq,
            "vmulq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31058,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vmulx,
            "vmulx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE312BE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmula.xyzw  ACC, $vf2, $vf3",
            Opcode::r5900ee_vmula,
            "vmula.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE011FE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vmulai,
            "vmulai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE011FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vmulaq,
            "vmulaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE311BC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vmulax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vmulax,
            "vmulax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A0002FF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vnop",
            Opcode::r5900ee_vnop,
            "vnop",
            [None, None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BC312FE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vopmula.xyz ACC, $vf2, $vf3",
            Opcode::r5900ee_vopmula,
            "vopmula.xyz",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BC3106E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vopmsub.xyz $vf1, $vf2, $vf3",
            Opcode::r5900ee_vopmsub,
            "vopmsub.xyz",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1043D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrget.xyzw  $vf1, R",
            Opcode::r5900ee_vrget,
            "vrget.xyzw",
            [Some("$vf1"), Some("R"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A00143E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrinit      R, $vf2x",
            Opcode::r5900ee_vrinit,
            "vrinit",
            [Some("R"), Some("$vf2x"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE1043C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrnext.xyzw $vf1, R",
            Opcode::r5900ee_vrnext,
            "vrnext.xyzw",
            [Some("$vf1"), Some("R"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A0313BE,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrsqrt      Q, $vf2x, $vf3x",
            Opcode::r5900ee_vrsqrt,
            "vrsqrt",
            [Some("Q"), Some("$vf2x"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A00143F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vrxor       R, $vf2x",
            Opcode::r5900ee_vrxor,
            "vrxor",
            [Some("R"), Some("$vf2x"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE20B7F,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqd.xyzw   $vf1, (--$vi2)",
            Opcode::r5900ee_vsqd,
            "vsqd.xyzw",
            [Some("$vf1"), Some("(--$vi2)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE20B7D,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqi.xyzw   $vf1, ($vi2++)",
            Opcode::r5900ee_vsqi,
            "vsqi.xyzw",
            [Some("$vf1"), Some("($vi2++)"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A0203BD,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsqrt       Q, $vf2x",
            Opcode::r5900ee_vsqrt,
            "vsqrt",
            [Some("Q"), Some("$vf2x"), None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3106C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsub.xyzw   $vf1, $vf2, $vf3",
            Opcode::r5900ee_vsub,
            "vsub.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01066,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubi.xyzw  $vf1, $vf2, I",
            Opcode::r5900ee_vsubi,
            "vsubi.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE01064,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubq.xyzw  $vf1, $vf2, Q",
            Opcode::r5900ee_vsubq,
            "vsubq.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE31044,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubx.xyzw  $vf1, $vf2, $vf3x",
            Opcode::r5900ee_vsubx,
            "vsubx.xyzw",
            [Some("$vf1"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE312FC,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsuba.xyzw  ACC, $vf2, $vf3",
            Opcode::r5900ee_vsuba,
            "vsuba.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0127E,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubai.xyzw ACC, $vf2, I",
            Opcode::r5900ee_vsubai,
            "vsubai.xyzw",
            [Some("ACC"), Some("$vf2"), Some("I"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE0127C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubaq.xyzw ACC, $vf2, Q",
            Opcode::r5900ee_vsubaq,
            "vsubaq.xyzw",
            [Some("ACC"), Some("$vf2"), Some("Q"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4BE3107C,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vsubax.xyzw ACC, $vf2, $vf3x",
            Opcode::r5900ee_vsubax,
            "vsubax.xyzw",
            [Some("ACC"), Some("$vf2"), Some("$vf3x"), None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
        TestEntry::new(
            0x4A0003BF,
            InstructionFlags::new_extension(IsaExtension::R5900EE).with_abi(Abi::EABI64),
            "vwaitq",
            Opcode::r5900ee_vwaitq,
            "vwaitq",
            [None, None, None, None, None],
        )
        .with_display_flags(
            InstructionDisplayFlags::default().with_r5900ee_prodg_sn_as_inverted_regs(true),
        ),
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}
