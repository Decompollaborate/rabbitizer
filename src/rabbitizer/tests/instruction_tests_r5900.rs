/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod common;

use common::{check_test_entries, TestEntry};
use rabbitizer::display_flags::InstructionDisplayFlags;
use rabbitizer::instr::{Instruction, InstructionFlags};
use rabbitizer::isa::IsaExtension;
use rabbitizer::opcodes::Opcode;
use rabbitizer::vram::Vram;

#[test]
fn check_r5900_instructions() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry {
            instr: Instruction::new(
                0x4A000038,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x0",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x0"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A004038,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x800",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x800"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A008038,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x1000",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x1000"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A008838,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x1100",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x1100"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A009038,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x1200",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x1200"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A009838,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x1300",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x1300"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00A038,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x1400",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x1400"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A07FFF8,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0xFFF8",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0xFFF8"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A080038,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x10000",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x10000"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A1F8038,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x3F000",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x3F000"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A1FFFB8,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x3FFF0",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x3FFF0"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A1FFFF8,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vcallms     0x3FFF8",
            expected_opcode: Opcode::r5900_vcallms,
            opcode_str: "vcallms",
            operands_str: [Some("0x3FFF8"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x70001030,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "pmfhl.lw    $v0",
            expected_opcode: Opcode::r5900_pmfhl_lw,
            opcode_str: "pmfhl.lw",
            operands_str: [Some("$v0"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x70001070,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "pmfhl.uw    $v0",
            expected_opcode: Opcode::r5900_pmfhl_uw,
            opcode_str: "pmfhl.uw",
            operands_str: [Some("$v0"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x700010B0,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "pmfhl.slw   $v0",
            expected_opcode: Opcode::r5900_pmfhl_slw,
            opcode_str: "pmfhl.slw",
            operands_str: [Some("$v0"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x700010F0,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "pmfhl.lh    $v0",
            expected_opcode: Opcode::r5900_pmfhl_lh,
            opcode_str: "pmfhl.lh",
            operands_str: [Some("$v0"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x70001130,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "pmfhl.sh    $v0",
            expected_opcode: Opcode::r5900_pmfhl_sh,
            opcode_str: "pmfhl.sh",
            operands_str: [Some("$v0"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x70000031,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "pmthl.lw    $zero",
            expected_opcode: Opcode::r5900_pmthl_lw,
            opcode_str: "pmthl.lw",
            operands_str: [Some("$zero"), None, None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4B020BFE,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vilwr.x     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_vilwr_x,
            opcode_str: "vilwr.x",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A820BFE,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vilwr.y     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_vilwr_y,
            opcode_str: "vilwr.y",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A420BFE,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vilwr.z     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_vilwr_z,
            opcode_str: "vilwr.z",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A220BFE,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vilwr.w     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_vilwr_w,
            opcode_str: "vilwr.w",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4B020BFF,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viswr.x     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_viswr_x,
            opcode_str: "viswr.x",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A820BFF,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viswr.y     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_viswr_y,
            opcode_str: "viswr.y",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A420BFF,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viswr.z     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_viswr_z,
            opcode_str: "viswr.z",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A220BFF,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viswr.w     $vi2, ($vi1)",
            expected_opcode: Opcode::r5900_viswr_w,
            opcode_str: "viswr.w",
            operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x70111334,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "psllh       $v0, $s1, 12",
            expected_opcode: Opcode::r5900_psllh,
            opcode_str: "psllh",
            operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x70111336,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "psrlh       $v0, $s1, 12",
            expected_opcode: Opcode::r5900_psrlh,
            opcode_str: "psrlh",
            operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x70111337,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "psrah       $v0, $s1, 12",
            expected_opcode: Opcode::r5900_psrah,
            opcode_str: "psrah",
            operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x7011133C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "psllw       $v0, $s1, 12",
            expected_opcode: Opcode::r5900_psllw,
            opcode_str: "psllw",
            operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x7011133E,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "psrlw       $v0, $s1, 12",
            expected_opcode: Opcode::r5900_psrlw,
            opcode_str: "psrlw",
            operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x7011133F,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "psraw       $v0, $s1, 12",
            expected_opcode: Opcode::r5900_psraw,
            opcode_str: "psraw",
            operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00551D,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vmaxi       $vf20, $vf10, $I",
            expected_opcode: Opcode::r5900_vmaxi,
            opcode_str: "vmaxi",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("$I"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00551D,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vmaxi       $vf20, $vf10, I",
            expected_opcode: Opcode::r5900_vmaxi,
            opcode_str: "vmaxi",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("I"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00551C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vmulq       $vf20, $vf10, $Q",
            expected_opcode: Opcode::r5900_vmulq,
            opcode_str: "vmulq",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("$Q"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A00551C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vmulq       $vf20, $vf10, Q",
            expected_opcode: Opcode::r5900_vmulq,
            opcode_str: "vmulq",
            operands_str: [Some("$vf20"), Some("$vf10"), Some("Q"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06043C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vrnext      $vf6, $R",
            expected_opcode: Opcode::r5900_vrnext,
            opcode_str: "vrnext",
            operands_str: [Some("$vf6"), Some("$R"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06043C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vrnext      $vf6, R",
            expected_opcode: Opcode::r5900_vrnext,
            opcode_str: "vrnext",
            operands_str: [Some("$vf6"), Some("R"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06003C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: "vaddax      $ACC, $vf0, $vf6x",
            expected_opcode: Opcode::r5900_vaddax,
            opcode_str: "vaddax",
            operands_str: [Some("$ACC"), Some("$vf0"), Some("$vf6x"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06003C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "vaddax      ACC, $vf0, $vf6x",
            expected_opcode: Opcode::r5900_vaddax,
            opcode_str: "vaddax",
            operands_str: [Some("ACC"), Some("$vf0"), Some("$vf6x"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A0663BC,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vdiv        Q, $vf12x, $vf6x",
            expected_opcode: Opcode::r5900_vdiv,
            opcode_str: "vdiv",
            operands_str: [Some("Q"), Some("$vf12x"), Some("$vf6x"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A066630,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viadd       $vi24, $vi12, $vi6",
            expected_opcode: Opcode::r5900_viadd,
            opcode_str: "viadd",
            operands_str: [Some("$vi24"), Some("$vi12"), Some("$vi6"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A066632,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viaddi      $vi6, $vi12, -0x8",
            expected_opcode: Opcode::r5900_viaddi,
            opcode_str: "viaddi",
            operands_str: [Some("$vi6"), Some("$vi12"), Some("-0x8"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06637E,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vlqd        $vf6, (--$vi12)",
            expected_opcode: Opcode::r5900_vlqd,
            opcode_str: "vlqd",
            operands_str: [Some("$vf6"), Some("(--$vi12)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06637F,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vsqd        $vf12, (--$vi6)",
            expected_opcode: Opcode::r5900_vsqd,
            opcode_str: "vsqd",
            operands_str: [Some("$vf12"), Some("(--$vi6)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06637C,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vlqi        $vf6, ($vi12++)",
            expected_opcode: Opcode::r5900_vlqi,
            opcode_str: "vlqi",
            operands_str: [Some("$vf6"), Some("($vi12++)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A06637D,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "vsqi        $vf12, ($vi6++)",
            expected_opcode: Opcode::r5900_vsqi,
            opcode_str: "vsqi",
            operands_str: [Some("$vf12"), Some("($vi6++)"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A0307B2,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viaddi      $vi3, $vi0, -0x2",
            expected_opcode: Opcode::r5900_viaddi,
            opcode_str: "viaddi",
            operands_str: [Some("$vi3"), Some("$vi0"), Some("-0x2"), None, None],
        },
        TestEntry {
            instr: Instruction::new(
                0x4A0303B2,
                Vram::new(0x80000000),
                InstructionFlags::new_extension(IsaExtension::R5900),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: true,
            expected: "viaddi      $vi3, $vi0, 0xE",
            expected_opcode: Opcode::r5900_viaddi,
            opcode_str: "viaddi",
            operands_str: [Some("$vi3"), Some("$vi0"), Some("0xE"), None, None],
        },
    ];

    assert_eq!(check_test_entries(ENTRIES, true), (0, 0));
}

#[test]
fn check_r5900_trunc_cvt_instructions() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry {
            instr: Instruction::new(0x4600600D, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: ".word       0x4600600D                   /* trunc.w.s   $f0, $f12 / 00000000 <OpcodeCategory: CORE_COP1_FPUS> */",
            expected_opcode: Opcode::core_trunc_w_s,
            opcode_str: "trunc.w.s",
            operands_str: [Some("$f0"), Some("$f12"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(0x46006024, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_gnu_as(),
            valid: true,
            expected: ".word       0x46006024                   /* cvt.w.s     $f0, $f12 / 00000000 <OpcodeCategory: CORE_COP1_FPUS> */",
            expected_opcode: Opcode::core_cvt_w_s,
            opcode_str: "cvt.w.s",
            operands_str: [Some("$f0"), Some("$f12"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(0x4600600D, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "trunc.w.s   $f0, $f12",
            expected_opcode: Opcode::core_trunc_w_s,
            opcode_str: "trunc.w.s",
            operands_str: [Some("$f0"), Some("$f12"), None, None, None],
        },
        TestEntry {
            instr: Instruction::new(0x46006024, Vram::new(0x80000000), InstructionFlags::new_extension(IsaExtension::R5900)),
            imm_override: None,
            display_flags: InstructionDisplayFlags::new_legacy_as(),
            valid: true,
            expected: "cvt.w.s     $f0, $f12",
            expected_opcode: Opcode::core_cvt_w_s,
            opcode_str: "cvt.w.s",
            operands_str: [Some("$f0"), Some("$f12"), None, None, None],
        },
    ];

    assert_eq!(check_test_entries(ENTRIES, true), (0, 0));
}
