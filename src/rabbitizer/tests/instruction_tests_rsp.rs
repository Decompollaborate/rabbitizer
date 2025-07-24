/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(feature = "RSP")]
mod common;

#[cfg(feature = "RSP")]
use common::{check_test_entries, TestEntry};

#[cfg(feature = "RSP")]
use rabbitizer::{
    display_flags::InstructionDisplayFlags,
    instr::{Instruction, InstructionFlags},
    isa::IsaExtension,
    opcodes::Opcode,
    vram::Vram,
};

#[cfg(feature = "RSP")]
#[test]
fn check_rsp_instructions() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry {
            instr: Instruction::new(
                0x09000419,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "j           func_A4001064",
            expected_opcode: Opcode::core_j,
            opcode_str: "j",
            operands_str: [Some("func_A4001064"), None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x21490000,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "addi        $9, $10, 0x0",
            expected_opcode: Opcode::core_addi,
            opcode_str: "addi",
            operands_str: [Some("$9"), Some("$10"), Some("0x0"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x8C060578,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lw          $6, 0x578($zero)",
            expected_opcode: Opcode::core_lw,
            opcode_str: "lw",
            operands_str: [Some("$6"), Some("0x578($zero)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x400B2800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false)
                .with_named_rsp_cop0(true),
            valid: true,
            expected: "mfc0        $11, SP_DMA_FULL",
            expected_opcode: Opcode::rsp_mfc0,
            opcode_str: "mfc0",
            operands_str: [Some("$11"), Some("SP_DMA_FULL"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x400B2800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false)
                .with_named_rsp_cop0(false),
            valid: true,
            expected: "mfc0        $11, $5",
            expected_opcode: Opcode::rsp_mfc0,
            opcode_str: "mfc0",
            operands_str: [Some("$11"), Some("$5"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x304203FF,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "andi        $2, $2, 0x3FF",
            expected_opcode: Opcode::core_andi,
            opcode_str: "andi",
            operands_str: [Some("$2"), Some("$2"), Some("0x3FF"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x10400003,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "beqz        $2, . + 4 + (0x3 << 2)",
            expected_opcode: Opcode::core_beqz,
            opcode_str: "beqz",
            operands_str: [Some("$2"), Some(". + 4 + (0x3 << 2)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x10400003,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B38E8"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "beqz        $2, .LA00B38E8",
            expected_opcode: Opcode::core_beqz,
            opcode_str: "beqz",
            operands_str: [Some("$2"), Some(".LA00B38E8"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00000000,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "nop",
            expected_opcode: Opcode::core_nop,
            opcode_str: "nop",
            operands_str: [None, None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x1C600033,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B3A74"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "bgtz        $3, .LA00B3A74",
            expected_opcode: Opcode::core_bgtz,
            opcode_str: "bgtz",
            operands_str: [Some("$3"), Some(".LA00B3A74"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x0D00077A,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "jal         func_A4001DE8",
            expected_opcode: Opcode::core_jal,
            opcode_str: "jal",
            operands_str: [Some("func_A4001DE8"), None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xAEEB000C,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sw          $11, 0xC($23)",
            expected_opcode: Opcode::core_sw,
            opcode_str: "sw",
            operands_str: [Some("$11"), Some("0xC($23)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x1560FB8D,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B2288"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "bnez        $11, .LA00B2288",
            expected_opcode: Opcode::core_bnez,
            opcode_str: "bnez",
            operands_str: [Some("$11"), Some(".LA00B2288"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x40921800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false)
                .with_named_rsp_cop0(true),
            valid: true,
            expected: "mtc0        $18, SP_WR_LEN",
            expected_opcode: Opcode::rsp_mtc0,
            opcode_str: "mtc0",
            operands_str: [Some("$18"), Some("SP_WR_LEN"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00026A82,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "srl         $13, $2, 10",
            expected_opcode: Opcode::core_srl,
            opcode_str: "srl",
            operands_str: [Some("$13"), Some("$2"), Some("10"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x004F1020,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "add         $2, $2, $15",
            expected_opcode: Opcode::core_add,
            opcode_str: "add",
            operands_str: [Some("$2"), Some("$2"), Some("$15"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x84040572,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lh          $4, 0x572($zero)",
            expected_opcode: Opcode::core_lh,
            opcode_str: "lh",
            operands_str: [Some("$4"), Some("0x572($zero)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x03E00008,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "jr          $ra",
            expected_opcode: Opcode::core_jr,
            opcode_str: "jr",
            operands_str: [Some("$ra"), None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x0000000D,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "break       0",
            expected_opcode: Opcode::core_break,
            opcode_str: "break",
            operands_str: [Some("0"), None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x19C0FA06,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B2288"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "blez        $14, .LA00B2288",
            expected_opcode: Opcode::core_blez,
            opcode_str: "blez",
            operands_str: [Some("$14"), Some(".LA00B2288"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x09000A19,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "j           func_A4002864",
            expected_opcode: Opcode::core_j,
            opcode_str: "j",
            operands_str: [Some("func_A4002864"), None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x37120000,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "ori         $18, $24, 0x0",
            expected_opcode: Opcode::core_ori,
            opcode_str: "ori",
            operands_str: [Some("$18"), Some("$24"), Some("0x0"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x1000FE72,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B1A78"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "b           .LA00B1A78",
            expected_opcode: Opcode::core_b,
            opcode_str: "b",
            operands_str: [Some(".LA00B1A78"), None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A0318EC,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vxor        $v3, $v3, $v3",
            expected_opcode: Opcode::rsp_vxor,
            opcode_str: "vxor",
            operands_str: [Some("$v3"), Some("$v3"), Some("$v3"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xEAF11B0B,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sdv         $v17[6], 0x58($23)",
            expected_opcode: Opcode::rsp_sdv,
            opcode_str: "sdv",
            operands_str: [Some("$v17[6]"), Some("0x58($23)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x940C0572,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lhu         $12, 0x572($zero)",
            expected_opcode: Opcode::core_lhu,
            opcode_str: "lhu",
            operands_str: [Some("$12"), Some("0x572($zero)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00095880,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sll         $11, $9, 2",
            expected_opcode: Opcode::core_sll,
            opcode_str: "sll",
            operands_str: [Some("$11"), Some("$9"), Some("2"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xA403057C,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sh          $3, 0x57C($zero)",
            expected_opcode: Opcode::core_sh,
            opcode_str: "sh",
            operands_str: [Some("$3"), Some("0x57C($zero)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xCA832000,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lqv         $v3[0], 0x0($20)",
            expected_opcode: Opcode::rsp_lqv,
            opcode_str: "lqv",
            operands_str: [Some("$v3[0]"), Some("0x0($20)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xEAE40C0B,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "ssv         $v4[8], 0x16($23)",
            expected_opcode: Opcode::rsp_ssv,
            opcode_str: "ssv",
            operands_str: [Some("$v4[8]"), Some("0x16($23)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xCBC41807,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "ldv         $v4[0], 0x38($30)",
            expected_opcode: Opcode::rsp_ldv,
            opcode_str: "ldv",
            operands_str: [Some("$v4[0]"), Some("0x38($30)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x3C07F510,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lui         $7, 0xF510",
            expected_opcode: Opcode::core_lui,
            opcode_str: "lui",
            operands_str: [Some("$7"), Some("0xF510"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00611824,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "and         $3, $3, $1",
            expected_opcode: Opcode::core_and,
            opcode_str: "and",
            operands_str: [Some("$3"), Some("$3"), Some("$1"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xEBC62000,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sqv         $v6[0], 0x0($30)",
            expected_opcode: Opcode::rsp_sqv,
            opcode_str: "sqv",
            operands_str: [Some("$v6[0]"), Some("0x0($30)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x1193FFFE,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B304C"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "beq         $12, $19, .LA00B304C",
            expected_opcode: Opcode::core_beq,
            opcode_str: "beq",
            operands_str: [Some("$12"), Some("$19"), Some(".LA00B304C"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x900B0539,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lbu         $11, 0x539($zero)",
            expected_opcode: Opcode::core_lbu,
            opcode_str: "lbu",
            operands_str: [Some("$11"), Some("0x539($zero)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B0C58A8,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vand        $v2, $v11, $v12[0]",
            expected_opcode: Opcode::rsp_vand,
            opcode_str: "vand",
            operands_str: [Some("$v2"), Some("$v11"), Some("$v12[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x008C5822,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sub         $11, $4, $12",
            expected_opcode: Opcode::core_sub,
            opcode_str: "sub",
            operands_str: [Some("$11"), Some("$4"), Some("$12"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00432006,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "srlv        $4, $3, $2",
            expected_opcode: Opcode::core_srlv,
            opcode_str: "srlv",
            operands_str: [Some("$4"), Some("$3"), Some("$2"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x488ED800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "mtc2        $14, $v27[0]",
            expected_opcode: Opcode::rsp_mtc2,
            opcode_str: "mtc2",
            operands_str: [Some("$14"), Some("$v27[0]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xC9B12802,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lrv         $v17[0], 0x20($13)",
            expected_opcode: Opcode::rsp_lrv,
            opcode_str: "lrv",
            operands_str: [Some("$v17[0]"), Some("0x20($13)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B1B21C6,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmudn       $v7, $v4, $v27[0]",
            expected_opcode: Opcode::rsp_vmudn,
            opcode_str: "vmudn",
            operands_str: [Some("$v7"), Some("$v4"), Some("$v27[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B7F488E,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmadn       $v2, $v9, $v31[3]",
            expected_opcode: Opcode::rsp_vmadn,
            opcode_str: "vmadn",
            operands_str: [Some("$v2"), Some("$v9"), Some("$v31[3]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B2A3A05,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmudm       $v8, $v7, $v10[1]",
            expected_opcode: Opcode::rsp_vmudm,
            opcode_str: "vmudm",
            operands_str: [Some("$v8"), Some("$v7"), Some("$v10[1]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B3D1EC7,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmudh       $v27, $v3, $v29[1]",
            expected_opcode: Opcode::rsp_vmudh,
            opcode_str: "vmudh",
            operands_str: [Some("$v27"), Some("$v3"), Some("$v29[1]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B7DFA0F,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmadh       $v8, $v31, $v29[3]",
            expected_opcode: Opcode::rsp_vmadh,
            opcode_str: "vmadh",
            operands_str: [Some("$v8"), Some("$v31"), Some("$v29[3]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B0A529D,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vsar        $v10, $v10, $v10[0]",
            expected_opcode: Opcode::rsp_vsar,
            opcode_str: "vsar",
            operands_str: [Some("$v10"), Some("$v10"), Some("$v10[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xEAFD1204,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "slv         $v29[4], 0x10($23)",
            expected_opcode: Opcode::rsp_slv,
            opcode_str: "slv",
            operands_str: [Some("$v29[4]"), Some("0x10($23)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xC827080F,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lsv         $v7[0], 0x1E($1)",
            expected_opcode: Opcode::rsp_lsv,
            opcode_str: "lsv",
            operands_str: [Some("$v7[0]"), Some("0x1E($1)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4BAA4351,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vsub        $v13, $v8, $v10[5]",
            expected_opcode: Opcode::rsp_vsub,
            opcode_str: "vsub",
            operands_str: [Some("$v13"), Some("$v8"), Some("$v10[5]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B1F8ECD,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmadm       $v27, $v17, $v31[0]",
            expected_opcode: Opcode::rsp_vmadm,
            opcode_str: "vmadm",
            operands_str: [Some("$v27"), Some("$v17"), Some("$v31[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B3F7384,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmudl       $v14, $v14, $v31[1]",
            expected_opcode: Opcode::rsp_vmudl,
            opcode_str: "vmudl",
            operands_str: [Some("$v14"), Some("$v14"), Some("$v31[1]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B9F2940,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmulf       $v5, $v5, $v31[4]",
            expected_opcode: Opcode::rsp_vmulf,
            opcode_str: "vmulf",
            operands_str: [Some("$v5"), Some("$v5"), Some("$v31[4]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4BAA4390,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vadd        $v14, $v8, $v10[5]",
            expected_opcode: Opcode::rsp_vadd,
            opcode_str: "vadd",
            operands_str: [Some("$v14"), Some("$v8"), Some("$v10[5]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4813D900,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "mfc2        $19, $v27[2]",
            expected_opcode: Opcode::rsp_mfc2,
            opcode_str: "mfc2",
            operands_str: [Some("$19"), Some("$v27[2]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A1174D5,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vsubc       $v19, $v14, $v17",
            expected_opcode: Opcode::rsp_vsubc,
            opcode_str: "vsubc",
            operands_str: [Some("$v19"), Some("$v14"), Some("$v17"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B7D6EE3,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vge         $v27, $v13, $v29[3]",
            expected_opcode: Opcode::rsp_vge,
            opcode_str: "vge",
            operands_str: [Some("$v27"), Some("$v13"), Some("$v29[3]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4AF4E0E4,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vcl         $v3, $v28, $v20[3h]",
            expected_opcode: Opcode::rsp_vcl,
            opcode_str: "vcl",
            operands_str: [Some("$v3"), Some("$v28"), Some("$v20[3h]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4AC550C8,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmacf       $v3, $v10, $v5[2h]",
            expected_opcode: Opcode::rsp_vmacf,
            opcode_str: "vmacf",
            operands_str: [Some("$v3"), Some("$v10"), Some("$v5[2h]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x15610003,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B2F4C"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "bne         $11, $1, .LA00B2F4C",
            expected_opcode: Opcode::core_bne,
            opcode_str: "bne",
            operands_str: [Some("$11"), Some("$1"), Some(".LA00B2F4C"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x05C1000F,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B3A2C"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "bgez        $14, .LA00B3A2C",
            expected_opcode: Opcode::core_bgez,
            opcode_str: "bgez",
            operands_str: [Some("$14"), Some(".LA00B3A2C"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xEAFD0688,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sbv         $v29[13], 0x8($23)",
            expected_opcode: Opcode::rsp_sbv,
            opcode_str: "sbv",
            operands_str: [Some("$v29[13]"), Some("0x8($23)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x01675825,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "or          $11, $11, $7",
            expected_opcode: Opcode::core_or,
            opcode_str: "or",
            operands_str: [Some("$11"), Some("$11"), Some("$7"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x03241804,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sllv        $3, $4, $25",
            expected_opcode: Opcode::core_sllv,
            opcode_str: "sllv",
            operands_str: [Some("$3"), Some("$4"), Some("$25"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00601827,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "not         $3, $3",
            expected_opcode: Opcode::core_not,
            opcode_str: "not",
            operands_str: [Some("$3"), Some("$3"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x83790500,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lb          $25, 0x500($27)",
            expected_opcode: Opcode::core_lb,
            opcode_str: "lb",
            operands_str: [Some("$25"), Some("0x500($27)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x05600002,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B3A2C"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "bltz        $11, .LA00B3A2C",
            expected_opcode: Opcode::core_bltz,
            opcode_str: "bltz",
            operands_str: [Some("$11"), Some(".LA00B3A2C"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xA2EB0009,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sb          $11, 0x9($23)",
            expected_opcode: Opcode::core_sb,
            opcode_str: "sb",
            operands_str: [Some("$11"), Some("0x9($23)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x000B5A83,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sra         $11, $11, 10",
            expected_opcode: Opcode::core_sra,
            opcode_str: "sra",
            operands_str: [Some("$11"), Some("$11"), Some("10"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xC98C1000,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "llv         $v12[0], 0x0($12)",
            expected_opcode: Opcode::rsp_llv,
            opcode_str: "llv",
            operands_str: [Some("$v12[0]"), Some("0x0($12)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4AF5E8E5,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vch         $v3, $v29, $v21[3h]",
            expected_opcode: Opcode::rsp_vch,
            opcode_str: "vch",
            operands_str: [Some("$v3"), Some("$v29"), Some("$v21[3h]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x484B0800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "cfc2        $11, $1",
            expected_opcode: Opcode::rsp_cfc2,
            opcode_str: "cfc2",
            operands_str: [Some("$11"), Some("$1"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A086A27,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmrg        $v8, $v13, $v8",
            expected_opcode: Opcode::rsp_vmrg,
            opcode_str: "vmrg",
            operands_str: [Some("$v8"), Some("$v13"), Some("$v8"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B7D7EE0,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vlt         $v27, $v15, $v29[3]",
            expected_opcode: Opcode::rsp_vlt,
            opcode_str: "vlt",
            operands_str: [Some("$v27"), Some("$v15"), Some("$v29[3]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x26F70018,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "addiu       $23, $23, 0x18",
            expected_opcode: Opcode::core_addiu,
            opcode_str: "addiu",
            operands_str: [Some("$23"), Some("$23"), Some("0x18"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00812021,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "addu        $4, $4, $1",
            expected_opcode: Opcode::core_addu,
            opcode_str: "addu",
            operands_str: [Some("$4"), Some("$4"), Some("$1"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B1F6A0C,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmadl       $v8, $v13, $v31[0]",
            expected_opcode: Opcode::rsp_vmadl,
            opcode_str: "vmadl",
            operands_str: [Some("$v8"), Some("$v13"), Some("$v31[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x0185602A,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "slt         $12, $12, $5",
            expected_opcode: Opcode::core_slt,
            opcode_str: "slt",
            operands_str: [Some("$12"), Some("$12"), Some("$5"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B224AF3,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vmov        $v11[1], $v2[1]",
            expected_opcode: Opcode::rsp_vmov,
            opcode_str: "vmov",
            operands_str: [Some("$v11[1]"), Some("$v2[1]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B0343F0,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vrcp        $v15[0], $v3[0]",
            expected_opcode: Opcode::rsp_vrcp,
            opcode_str: "vrcp",
            operands_str: [Some("$v15[0]"), Some("$v3[0]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B7D4232,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vrcph       $v8[0], $v29[3]",
            expected_opcode: Opcode::rsp_vrcph,
            opcode_str: "vrcph",
            operands_str: [Some("$v8[0]"), Some("$v29[3]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4BDE0026,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vcr         $v0, $v0, $v30[6]",
            expected_opcode: Opcode::rsp_vcr,
            opcode_str: "vcr",
            operands_str: [Some("$v0"), Some("$v0"), Some("$v30[6]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A0A56D3,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vabs        $v27, $v10, $v10",
            expected_opcode: Opcode::rsp_vabs,
            opcode_str: "vabs",
            operands_str: [Some("$v27"), Some("$v10"), Some("$v10"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xC9513800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "luv         $v17[0], 0x0($10)",
            expected_opcode: Opcode::rsp_luv,
            opcode_str: "luv",
            operands_str: [Some("$v17[0]"), Some("0x0($10)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B0D41F1,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vrcpl       $v7[0], $v13[0]",
            expected_opcode: Opcode::rsp_vrcpl,
            opcode_str: "vrcpl",
            operands_str: [Some("$v7[0]"), Some("$v13[0]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x398C0001,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "xori        $12, $12, 0x1",
            expected_opcode: Opcode::core_xori,
            opcode_str: "xori",
            operands_str: [Some("$12"), Some("$12"), Some("0x1"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00641826,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "xor         $3, $3, $4",
            expected_opcode: Opcode::core_xor,
            opcode_str: "xor",
            operands_str: [Some("$3"), Some("$3"), Some("$4"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4ACB36D4,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vaddc       $v27, $v6, $v11[2h]",
            expected_opcode: Opcode::rsp_vaddc,
            opcode_str: "vaddc",
            operands_str: [Some("$v27"), Some("$v6"), Some("$v11[2h]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B1F4221,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "veq         $v8, $v8, $v31[0]",
            expected_opcode: Opcode::rsp_veq,
            opcode_str: "veq",
            operands_str: [Some("$v8"), Some("$v8"), Some("$v31[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B1F7BE2,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vne         $v15, $v15, $v31[0]",
            expected_opcode: Opcode::rsp_vne,
            opcode_str: "vne",
            operands_str: [Some("$v15"), Some("$v15"), Some("$v31[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B1F7A2D,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vnxor       $v8, $v15, $v31[0]",
            expected_opcode: Opcode::rsp_vnxor,
            opcode_str: "vnxor",
            operands_str: [Some("$v8"), Some("$v15"), Some("$v31[0]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xE8FB3800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "suv         $v27[0], 0x0($7)",
            expected_opcode: Opcode::rsp_suv,
            opcode_str: "suv",
            operands_str: [Some("$v27[0]"), Some("0x0($7)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x03C0F809,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "jalr        $30",
            expected_opcode: Opcode::core_jalr,
            opcode_str: "jalr",
            operands_str: [Some("$30"), None, None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xC86F3000,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lpv         $v15[0], 0x0($3)",
            expected_opcode: Opcode::rsp_lpv,
            opcode_str: "lpv",
            operands_str: [Some("$v15[0]"), Some("0x0($3)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4A1FEF6A,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vor         $v29, $v29, $v31",
            expected_opcode: Opcode::rsp_vor,
            opcode_str: "vor",
            operands_str: [Some("$v29"), Some("$v29"), Some("$v31"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xE9085F04,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "stv         $v8[14], 0x40($8)",
            expected_opcode: Opcode::rsp_stv,
            opcode_str: "stv",
            operands_str: [Some("$v8[14]"), Some("0x40($8)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xC9085904,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "ltv         $v8[2], 0x40($8)",
            expected_opcode: Opcode::rsp_ltv,
            opcode_str: "ltv",
            operands_str: [Some("$v8[2]"), Some("0x40($8)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B1F42F6,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vrsqh       $v11[0], $v31[0]",
            expected_opcode: Opcode::rsp_vrsqh,
            opcode_str: "vrsqh",
            operands_str: [Some("$v11[0]"), Some("$v31[0]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B4743F5,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vrsql       $v15[0], $v7[2]",
            expected_opcode: Opcode::rsp_vrsql,
            opcode_str: "vrsql",
            operands_str: [Some("$v15[0]"), Some("$v7[2]"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xE8273038,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "spv         $v7[0], 0x1C0($1)",
            expected_opcode: Opcode::rsp_spv,
            opcode_str: "spv",
            operands_str: [Some("$v7[0]"), Some("0x1C0($1)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0xC94F0786,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "lbv         $v15[15], 0x6($10)",
            expected_opcode: Opcode::rsp_lbv,
            opcode_str: "lbv",
            operands_str: [Some("$v15[15]"), Some("0x6($10)"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x48CB0800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "ctc2        $11, $1",
            expected_opcode: Opcode::rsp_ctc2,
            opcode_str: "ctc2",
            operands_str: [Some("$11"), Some("$1"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x48CB0800,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(true)
                .with_named_fpr(true),
            valid: true,
            expected: "ctc2        $t3, $1",
            expected_opcode: Opcode::rsp_ctc2,
            opcode_str: "ctc2",
            operands_str: [Some("$t3"), Some("$1"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x4B3E01EB,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "vnor        $v7, $v0, $v30[1]",
            expected_opcode: Opcode::rsp_vnor,
            opcode_str: "vnor",
            operands_str: [Some("$v7"), Some("$v0"), Some("$v30[1]"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x04D1FF9D,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: Some(".LA00B3020"),
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "bgezal      $6, .LA00B3020",
            expected_opcode: Opcode::core_bgezal,
            opcode_str: "bgezal",
            operands_str: [Some("$6"), Some(".LA00B3020"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00035822,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "neg         $11, $3",
            expected_opcode: Opcode::core_neg,
            opcode_str: "neg",
            operands_str: [Some("$11"), Some("$3"), None, None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00042100,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sll         $4, $4, 4",
            expected_opcode: Opcode::core_sll,
            opcode_str: "sll",
            operands_str: [Some("$4"), Some("$4"), Some("4"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00021882,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "srl         $3, $2, 2",
            expected_opcode: Opcode::core_srl,
            opcode_str: "srl",
            operands_str: [Some("$3"), Some("$2"), Some("2"), None, None],
            test_encoder: true,
        },
        TestEntry {
            instr: Instruction::new(
                0x00017443,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default()
                .with_named_gpr(false)
                .with_named_fpr(false),
            valid: true,
            expected: "sra         $14, $1, 17",
            expected_opcode: Opcode::core_sra,
            opcode_str: "sra",
            operands_str: [Some("$14"), Some("$1"), Some("17"), None, None],
            test_encoder: true,
        },
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}

#[cfg(feature = "RSP")]
#[test]
fn check_rsp_instructions_vu() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry::new(
            0xC8000000,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "lbv         $v0[0], 0x0($zero)",
            Opcode::rsp_lbv,
            "lbv",
            [Some("$v0[0]"), Some("0x0($zero)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC8210888,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "lsv         $v1[1], 0x10($at)",
            Opcode::rsp_lsv,
            "lsv",
            [Some("$v1[1]"), Some("0x10($at)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC8421108,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "llv         $v2[2], 0x20($v0)",
            Opcode::rsp_llv,
            "llv",
            [Some("$v2[2]"), Some("0x20($v0)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC8631986,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "ldv         $v3[3], 0x30($v1)",
            Opcode::rsp_ldv,
            "ldv",
            [Some("$v3[3]"), Some("0x30($v1)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC8842204,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "lqv         $v4[4], 0x40($a0)",
            Opcode::rsp_lqv,
            "lqv",
            [Some("$v4[4]"), Some("0x40($a0)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC8A52A85,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "lrv         $v5[5], 0x50($a1)",
            Opcode::rsp_lrv,
            "lrv",
            [Some("$v5[5]"), Some("0x50($a1)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC8C6330C,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "lpv         $v6[6], 0x60($a2)",
            Opcode::rsp_lpv,
            "lpv",
            [Some("$v6[6]"), Some("0x60($a2)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC8E73B8E,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "luv         $v7[7], 0x70($a3)",
            Opcode::rsp_luv,
            "luv",
            [Some("$v7[7]"), Some("0x70($a3)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC9084408,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "lhv         $v8[8], 0x80($t0)",
            Opcode::rsp_lhv,
            "lhv",
            [Some("$v8[8]"), Some("0x80($t0)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC9294C89,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "lfv         $v9[9], 0x90($t1)",
            Opcode::rsp_lfv,
            "lfv",
            [Some("$v9[9]"), Some("0x90($t1)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xC96B5D91,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "ltv         $v11[11], 0x110($t3)",
            Opcode::rsp_ltv,
            "ltv",
            [Some("$v11[11]"), Some("0x110($t3)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xE98C0600,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "sbv         $v12[12], 0x0($t4)",
            Opcode::rsp_sbv,
            "sbv",
            [Some("$v12[12]"), Some("0x0($t4)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xE9AD0E88,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "ssv         $v13[13], 0x10($t5)",
            Opcode::rsp_ssv,
            "ssv",
            [Some("$v13[13]"), Some("0x10($t5)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xE9CE1708,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "slv         $v14[14], 0x20($t6)",
            Opcode::rsp_slv,
            "slv",
            [Some("$v14[14]"), Some("0x20($t6)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xE9EF1F86,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "sdv         $v15[15], 0x30($t7)",
            Opcode::rsp_sdv,
            "sdv",
            [Some("$v15[15]"), Some("0x30($t7)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEA102004,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "sqv         $v16[0], 0x40($s0)",
            Opcode::rsp_sqv,
            "sqv",
            [Some("$v16[0]"), Some("0x40($s0)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEA312885,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "srv         $v17[1], 0x50($s1)",
            Opcode::rsp_srv,
            "srv",
            [Some("$v17[1]"), Some("0x50($s1)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEA52310C,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "spv         $v18[2], 0x60($s2)",
            Opcode::rsp_spv,
            "spv",
            [Some("$v18[2]"), Some("0x60($s2)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEA73398E,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "suv         $v19[3], 0x70($s3)",
            Opcode::rsp_suv,
            "suv",
            [Some("$v19[3]"), Some("0x70($s3)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEA944208,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "shv         $v20[4], 0x80($s4)",
            Opcode::rsp_shv,
            "shv",
            [Some("$v20[4]"), Some("0x80($s4)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEAB54A89,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "sfv         $v21[5], 0x90($s5)",
            Opcode::rsp_sfv,
            "sfv",
            [Some("$v21[5]"), Some("0x90($s5)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEAD65310,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "swv         $v22[6], 0x100($s6)",
            Opcode::rsp_swv,
            "swv",
            [Some("$v22[6]"), Some("0x100($s6)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0xEAF75B91,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "stv         $v23[7], 0x110($s7)",
            Opcode::rsp_stv,
            "stv",
            [Some("$v23[7]"), Some("0x110($s7)"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A020800,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmulf       $v0, $v1, $v2",
            Opcode::rsp_vmulf,
            "vmulf",
            [Some("$v0"), Some("$v1"), Some("$v2"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0520C2,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrndp       $v3, $v4, $v5",
            Opcode::rsp_vrndp,
            "vrndp",
            [Some("$v3"), Some("$v4"), Some("$v5"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A083984,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudl       $v6, $v7, $v8",
            Opcode::rsp_vmudl,
            "vmudl",
            [Some("$v6"), Some("$v7"), Some("$v8"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0B5245,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudm       $v9, $v10, $v11",
            Opcode::rsp_vmudm,
            "vmudm",
            [Some("$v9"), Some("$v10"), Some("$v11"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0E6B06,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudn       $v12, $v13, $v14",
            Opcode::rsp_vmudn,
            "vmudn",
            [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1183C7,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudh       $v15, $v16, $v17",
            Opcode::rsp_vmudh,
            "vmudh",
            [Some("$v15"), Some("$v16"), Some("$v17"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A149C88,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmacf       $v18, $v19, $v20",
            Opcode::rsp_vmacf,
            "vmacf",
            [Some("$v18"), Some("$v19"), Some("$v20"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A17B549,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmacu       $v21, $v22, $v23",
            Opcode::rsp_vmacu,
            "vmacu",
            [Some("$v21"), Some("$v22"), Some("$v23"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1ACE0A,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrndn       $v24, $v25, $v26",
            Opcode::rsp_vrndn,
            "vrndn",
            [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1DE6CB,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmacq       $v27, $v28, $v29",
            Opcode::rsp_vmacq,
            "vmacq",
            [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A00FF8C,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadl       $v30, $v31, $v0",
            Opcode::rsp_vmadl,
            "vmadl",
            [Some("$v30"), Some("$v31"), Some("$v0"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A02080D,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadm       $v0, $v1, $v2",
            Opcode::rsp_vmadm,
            "vmadm",
            [Some("$v0"), Some("$v1"), Some("$v2"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0520CE,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadn       $v3, $v4, $v5",
            Opcode::rsp_vmadn,
            "vmadn",
            [Some("$v3"), Some("$v4"), Some("$v5"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A08398F,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadh       $v6, $v7, $v8",
            Opcode::rsp_vmadh,
            "vmadh",
            [Some("$v6"), Some("$v7"), Some("$v8"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A08399D,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vsar        $v6, $v7, $v8",
            Opcode::rsp_vsar,
            "vsar",
            [Some("$v6"), Some("$v7"), Some("$v8"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0E6B14,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vaddc       $v12, $v13, $v14",
            Opcode::rsp_vaddc,
            "vaddc",
            [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1183D5,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vsubc       $v15, $v16, $v17",
            Opcode::rsp_vsubc,
            "vsubc",
            [Some("$v15"), Some("$v16"), Some("$v17"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1183E0,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vlt         $v15, $v16, $v17",
            Opcode::rsp_vlt,
            "vlt",
            [Some("$v15"), Some("$v16"), Some("$v17"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A149CA1,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "veq         $v18, $v19, $v20",
            Opcode::rsp_veq,
            "veq",
            [Some("$v18"), Some("$v19"), Some("$v20"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A17B562,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vne         $v21, $v22, $v23",
            Opcode::rsp_vne,
            "vne",
            [Some("$v21"), Some("$v22"), Some("$v23"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1ACE23,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vge         $v24, $v25, $v26",
            Opcode::rsp_vge,
            "vge",
            [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1DE6E4,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vcl         $v27, $v28, $v29",
            Opcode::rsp_vcl,
            "vcl",
            [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A00FFA5,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vch         $v30, $v31, $v0",
            Opcode::rsp_vch,
            "vch",
            [Some("$v30"), Some("$v31"), Some("$v0"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A020826,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vcr         $v0, $v1, $v2",
            Opcode::rsp_vcr,
            "vcr",
            [Some("$v0"), Some("$v1"), Some("$v2"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0520E7,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmrg        $v3, $v4, $v5",
            Opcode::rsp_vmrg,
            "vmrg",
            [Some("$v3"), Some("$v4"), Some("$v5"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0839A8,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vand        $v6, $v7, $v8",
            Opcode::rsp_vand,
            "vand",
            [Some("$v6"), Some("$v7"), Some("$v8"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0B5269,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vnand       $v9, $v10, $v11",
            Opcode::rsp_vnand,
            "vnand",
            [Some("$v9"), Some("$v10"), Some("$v11"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0E6B2A,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vor         $v12, $v13, $v14",
            Opcode::rsp_vor,
            "vor",
            [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1183EB,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vnor        $v15, $v16, $v17",
            Opcode::rsp_vnor,
            "vnor",
            [Some("$v15"), Some("$v16"), Some("$v17"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A149CAC,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vxor        $v18, $v19, $v20",
            Opcode::rsp_vxor,
            "vxor",
            [Some("$v18"), Some("$v19"), Some("$v20"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A17B56D,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vnxor       $v21, $v22, $v23",
            Opcode::rsp_vnxor,
            "vnxor",
            [Some("$v21"), Some("$v22"), Some("$v23"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A1F07B0,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrcp        $v30, $v31",
            Opcode::rsp_vrcp,
            "vrcp",
            [Some("$v30"), Some("$v31"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A010031,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrcpl       $v0, $v1",
            Opcode::rsp_vrcpl,
            "vrcpl",
            [Some("$v0"), Some("$v1"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A0300B2,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrcph       $v2, $v3",
            Opcode::rsp_vrcph,
            "vrcph",
            [Some("$v2"), Some("$v3"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A050133,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmov        $v4, $v5",
            Opcode::rsp_vmov,
            "vmov",
            [Some("$v4"), Some("$v5"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A0701B4,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrsq        $v6, $v7",
            Opcode::rsp_vrsq,
            "vrsq",
            [Some("$v6"), Some("$v7"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A090235,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrsql       $v8, $v9",
            Opcode::rsp_vrsql,
            "vrsql",
            [Some("$v8"), Some("$v9"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A0B02B6,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrsqh       $v10, $v11",
            Opcode::rsp_vrsqh,
            "vrsqh",
            [Some("$v10"), Some("$v11"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4A020801,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmulu       $v0, $v1, $v2",
            Opcode::rsp_vmulu,
            "vmulu",
            [Some("$v0"), Some("$v1"), Some("$v2"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0520C3,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmulq       $v3, $v4, $v5",
            Opcode::rsp_vmulq,
            "vmulq",
            [Some("$v3"), Some("$v4"), Some("$v5"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A083990,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vadd        $v6, $v7, $v8",
            Opcode::rsp_vadd,
            "vadd",
            [Some("$v6"), Some("$v7"), Some("$v8"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0B5251,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vsub        $v9, $v10, $v11",
            Opcode::rsp_vsub,
            "vsub",
            [Some("$v9"), Some("$v10"), Some("$v11"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A0E6B13,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vabs        $v12, $v13, $v14",
            Opcode::rsp_vabs,
            "vabs",
            [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
        ),
        TestEntry::new_rsp(
            0x4A000037,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vnop",
            Opcode::rsp_vnop,
            "vnop",
            [None, None, None, None, None],
        ),
        TestEntry::new_rsp(
            0x4AF4E0E4,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vcl         $v3, $v28, $v20[3h]",
            Opcode::rsp_vcl,
            "vcl",
            [Some("$v3"), Some("$v28"), Some("$v20[3h]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4AC550C8,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmacf       $v3, $v10, $v5[2h]",
            Opcode::rsp_vmacf,
            "vmacf",
            [Some("$v3"), Some("$v10"), Some("$v5[2h]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4AF5E8E5,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vch         $v3, $v29, $v21[3h]",
            Opcode::rsp_vch,
            "vch",
            [Some("$v3"), Some("$v29"), Some("$v21[3h]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4ACB36D4,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vaddc       $v27, $v6, $v11[2h]",
            Opcode::rsp_vaddc,
            "vaddc",
            [Some("$v27"), Some("$v6"), Some("$v11[2h]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B0C58A8,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vand        $v2, $v11, $v12[0]",
            Opcode::rsp_vand,
            "vand",
            [Some("$v2"), Some("$v11"), Some("$v12[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x488ED800,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "mtc2        $t6, $v27[0]",
            Opcode::rsp_mtc2,
            "mtc2",
            [Some("$t6"), Some("$v27[0]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4B1B21C6,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudn       $v7, $v4, $v27[0]",
            Opcode::rsp_vmudn,
            "vmudn",
            [Some("$v7"), Some("$v4"), Some("$v27[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B7F488E,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadn       $v2, $v9, $v31[3]",
            Opcode::rsp_vmadn,
            "vmadn",
            [Some("$v2"), Some("$v9"), Some("$v31[3]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B2A3A05,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudm       $v8, $v7, $v10[1]",
            Opcode::rsp_vmudm,
            "vmudm",
            [Some("$v8"), Some("$v7"), Some("$v10[1]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B3D1EC7,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudh       $v27, $v3, $v29[1]",
            Opcode::rsp_vmudh,
            "vmudh",
            [Some("$v27"), Some("$v3"), Some("$v29[1]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B7DFA0F,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadh       $v8, $v31, $v29[3]",
            Opcode::rsp_vmadh,
            "vmadh",
            [Some("$v8"), Some("$v31"), Some("$v29[3]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B0A529D,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vsar        $v10, $v10, $v10[0]",
            Opcode::rsp_vsar,
            "vsar",
            [Some("$v10"), Some("$v10"), Some("$v10[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4BAA4351,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vsub        $v13, $v8, $v10[5]",
            Opcode::rsp_vsub,
            "vsub",
            [Some("$v13"), Some("$v8"), Some("$v10[5]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B1F8ECD,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadm       $v27, $v17, $v31[0]",
            Opcode::rsp_vmadm,
            "vmadm",
            [Some("$v27"), Some("$v17"), Some("$v31[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B3F7384,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmudl       $v14, $v14, $v31[1]",
            Opcode::rsp_vmudl,
            "vmudl",
            [Some("$v14"), Some("$v14"), Some("$v31[1]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B9F2940,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmulf       $v5, $v5, $v31[4]",
            Opcode::rsp_vmulf,
            "vmulf",
            [Some("$v5"), Some("$v5"), Some("$v31[4]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4BAA4390,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vadd        $v14, $v8, $v10[5]",
            Opcode::rsp_vadd,
            "vadd",
            [Some("$v14"), Some("$v8"), Some("$v10[5]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4813D900,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "mfc2        $s3, $v27[2]",
            Opcode::rsp_mfc2,
            "mfc2",
            [Some("$s3"), Some("$v27[2]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4B7D6EE3,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vge         $v27, $v13, $v29[3]",
            Opcode::rsp_vge,
            "vge",
            [Some("$v27"), Some("$v13"), Some("$v29[3]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B7D7EE0,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vlt         $v27, $v15, $v29[3]",
            Opcode::rsp_vlt,
            "vlt",
            [Some("$v27"), Some("$v15"), Some("$v29[3]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B1F6A0C,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmadl       $v8, $v13, $v31[0]",
            Opcode::rsp_vmadl,
            "vmadl",
            [Some("$v8"), Some("$v13"), Some("$v31[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B224AF3,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vmov        $v11[1], $v2[1]",
            Opcode::rsp_vmov,
            "vmov",
            [Some("$v11[1]"), Some("$v2[1]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4B0343F0,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrcp        $v15[0], $v3[0]",
            Opcode::rsp_vrcp,
            "vrcp",
            [Some("$v15[0]"), Some("$v3[0]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4B7D4232,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrcph       $v8[0], $v29[3]",
            Opcode::rsp_vrcph,
            "vrcph",
            [Some("$v8[0]"), Some("$v29[3]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4BDE0026,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vcr         $v0, $v0, $v30[6]",
            Opcode::rsp_vcr,
            "vcr",
            [Some("$v0"), Some("$v0"), Some("$v30[6]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B0D41F1,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrcpl       $v7[0], $v13[0]",
            Opcode::rsp_vrcpl,
            "vrcpl",
            [Some("$v7[0]"), Some("$v13[0]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4B1F4221,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "veq         $v8, $v8, $v31[0]",
            Opcode::rsp_veq,
            "veq",
            [Some("$v8"), Some("$v8"), Some("$v31[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B1F7BE2,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vne         $v15, $v15, $v31[0]",
            Opcode::rsp_vne,
            "vne",
            [Some("$v15"), Some("$v15"), Some("$v31[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B1F7A2D,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vnxor       $v8, $v15, $v31[0]",
            Opcode::rsp_vnxor,
            "vnxor",
            [Some("$v8"), Some("$v15"), Some("$v31[0]"), None, None],
        ),
        TestEntry::new_rsp(
            0x4B1F42F6,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrsqh       $v11[0], $v31[0]",
            Opcode::rsp_vrsqh,
            "vrsqh",
            [Some("$v11[0]"), Some("$v31[0]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4B4743F5,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vrsql       $v15[0], $v7[2]",
            Opcode::rsp_vrsql,
            "vrsql",
            [Some("$v15[0]"), Some("$v7[2]"), None, None, None],
        ),
        TestEntry::new_rsp(
            0x4B3E01EB,
            InstructionFlags::new_extension(IsaExtension::RSP),
            "vnor        $v7, $v0, $v30[1]",
            Opcode::rsp_vnor,
            "vnor",
            [Some("$v7"), Some("$v0"), Some("$v30[1]"), None, None],
        ),
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}

#[cfg(feature = "RspViceMsp")]
#[test]
fn check_rsp_instructions_vice_msp() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry::new_rsp(
            0xC94A5510,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "lwv         $v10[10], 0x100($t2)",
            Opcode::rsp_lwv,
            "lwv",
            [Some("$v10[10]"), Some("0x100($t2)"), None, None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0xC94A5510,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0xC94A5510                   /* lwv         $v10[10], 0x100($t2) / 00000000 <OpcodeCategory: RSP_NORMAL_LWC2> */",
            expected_opcode: Opcode::rsp_lwv,
            opcode_str: "lwv",
            operands_str: [Some("$v10[10]"), Some("0x100($t2)"), None, None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A0B5252,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vsut        $v9, $v10, $v11",
            Opcode::rsp_vsut,
            "vsut",
            [Some("$v9"), Some("$v10"), Some("$v11"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A0B5252,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A0B5252                   /* vsut        $v9, $v10, $v11 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vsut,
            opcode_str: "vsut",
            operands_str: [Some("$v9"), Some("$v10"), Some("$v11"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A149C96,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vaddb       $v18, $v19, $v20",
            Opcode::rsp_vaddb,
            "vaddb",
            [Some("$v18"), Some("$v19"), Some("$v20"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A149C96,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A149C96                   /* vaddb       $v18, $v19, $v20 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vaddb,
            opcode_str: "vaddb",
            operands_str: [Some("$v18"), Some("$v19"), Some("$v20"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A17B557,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vsubb       $v21, $v22, $v23",
            Opcode::rsp_vsubb,
            "vsubb",
            [Some("$v21"), Some("$v22"), Some("$v23"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A17B557,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A17B557                   /* vsubb       $v21, $v22, $v23 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vsubb,
            opcode_str: "vsubb",
            operands_str: [Some("$v21"), Some("$v22"), Some("$v23"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A1ACE18,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vaccb       $v24, $v25, $v26",
            Opcode::rsp_vaccb,
            "vaccb",
            [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A1ACE18,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A1ACE18                   /* vaccb       $v24, $v25, $v26 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vaccb,
            opcode_str: "vaccb",
            operands_str: [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A1DE6D9,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vsucb       $v27, $v28, $v29",
            Opcode::rsp_vsucb,
            "vsucb",
            [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A1DE6D9,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A1DE6D9                   /* vsucb       $v27, $v28, $v29 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vsucb,
            opcode_str: "vsucb",
            operands_str: [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A00FF9A,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vsad        $v30, $v31, $v0",
            Opcode::rsp_vsad,
            "vsad",
            [Some("$v30"), Some("$v31"), Some("$v0"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A00FF9A,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A00FF9A                   /* vsad        $v30, $v31, $v0 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vsad,
            opcode_str: "vsad",
            operands_str: [Some("$v30"), Some("$v31"), Some("$v0"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A02081B,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vsac        $v0, $v1, $v2",
            Opcode::rsp_vsac,
            "vsac",
            [Some("$v0"), Some("$v1"), Some("$v2"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A02081B,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A02081B                   /* vsac        $v0, $v1, $v2 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vsac,
            opcode_str: "vsac",
            operands_str: [Some("$v0"), Some("$v1"), Some("$v2"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A0520DC,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vsum        $v3, $v4, $v5",
            Opcode::rsp_vsum,
            "vsum",
            [Some("$v3"), Some("$v4"), Some("$v5"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A0520DC,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A0520DC                   /* vsum        $v3, $v4, $v5 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vsum,
            opcode_str: "vsum",
            operands_str: [Some("$v3"), Some("$v4"), Some("$v5"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A0B525E,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vacc        $v9, $v10, $v11",
            Opcode::rsp_vacc,
            "vacc",
            [Some("$v9"), Some("$v10"), Some("$v11"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A0B525E,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A0B525E                   /* vacc        $v9, $v10, $v11 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vacc,
            opcode_str: "vacc",
            operands_str: [Some("$v9"), Some("$v10"), Some("$v11"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A0E6B1F,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vsuc        $v12, $v13, $v14",
            Opcode::rsp_vsuc,
            "vsuc",
            [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A0E6B1F,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A0E6B1F                   /* vsuc        $v12, $v13, $v14 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vsuc,
            opcode_str: "vsuc",
            operands_str: [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A1ACE2E,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "v056        $v24, $v25, $v26",
            Opcode::rsp_v056,
            "v056",
            [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A1ACE2E,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A1ACE2E                   /* v056        $v24, $v25, $v26 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_v056,
            opcode_str: "v056",
            operands_str: [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A1DE6EF,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "v057        $v27, $v28, $v29",
            Opcode::rsp_v057,
            "v057",
            [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A1DE6EF,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A1DE6EF                   /* v057        $v27, $v28, $v29 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_v057,
            opcode_str: "v057",
            operands_str: [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A0E6B38,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vextt       $v12, $v13, $v14",
            Opcode::rsp_vextt,
            "vextt",
            [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A0E6B38,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A0E6B38                   /* vextt       $v12, $v13, $v14 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vextt,
            opcode_str: "vextt",
            operands_str: [Some("$v12"), Some("$v13"), Some("$v14"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A1183F9,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vextq       $v15, $v16, $v17",
            Opcode::rsp_vextq,
            "vextq",
            [Some("$v15"), Some("$v16"), Some("$v17"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A1183F9,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A1183F9                   /* vextq       $v15, $v16, $v17 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vextq,
            opcode_str: "vextq",
            operands_str: [Some("$v15"), Some("$v16"), Some("$v17"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A149CBA,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vextn       $v18, $v19, $v20",
            Opcode::rsp_vextn,
            "vextn",
            [Some("$v18"), Some("$v19"), Some("$v20"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A149CBA,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A149CBA                   /* vextn       $v18, $v19, $v20 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vextn,
            opcode_str: "vextn",
            operands_str: [Some("$v18"), Some("$v19"), Some("$v20"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A17B57B,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "v073        $v21, $v22, $v23",
            Opcode::rsp_v073,
            "v073",
            [Some("$v21"), Some("$v22"), Some("$v23"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A17B57B,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A17B57B                   /* v073        $v21, $v22, $v23 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_v073,
            opcode_str: "v073",
            operands_str: [Some("$v21"), Some("$v22"), Some("$v23"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A1ACE3C,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vinst       $v24, $v25, $v26",
            Opcode::rsp_vinst,
            "vinst",
            [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A1ACE3C,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A1ACE3C                   /* vinst       $v24, $v25, $v26 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vinst,
            opcode_str: "vinst",
            operands_str: [Some("$v24"), Some("$v25"), Some("$v26"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A1DE6FD,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vinsq       $v27, $v28, $v29",
            Opcode::rsp_vinsq,
            "vinsq",
            [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A1DE6FD,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A1DE6FD                   /* vinsq       $v27, $v28, $v29 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vinsq,
            opcode_str: "vinsq",
            operands_str: [Some("$v27"), Some("$v28"), Some("$v29"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A00FFBE,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vinsn       $v30, $v31, $v0",
            Opcode::rsp_vinsn,
            "vinsn",
            [Some("$v30"), Some("$v31"), Some("$v0"), None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A00FFBE,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A00FFBE                   /* vinsn       $v30, $v31, $v0 / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vinsn,
            opcode_str: "vinsn",
            operands_str: [Some("$v30"), Some("$v31"), Some("$v0"), None, None],
            test_encoder: true,
        },
        TestEntry::new_rsp(
            0x4A00003F,
            InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(true),
            "vnull",
            Opcode::rsp_vnull,
            "vnull",
            [None, None, None, None, None],
        ),
        TestEntry {
            instr: Instruction::new(
                0x4A00003F,
                Vram::new(0xA4000000),
                InstructionFlags::new_extension(IsaExtension::RSP).with_gated_rsp_vice_msp(false),
            ),
            imm_override: None,
            display_flags: InstructionDisplayFlags::default(),
            valid: false,
            expected: ".word       0x4A00003F                   /* vnull / 00000000 <OpcodeCategory: RSP_COP2_VU> */",
            expected_opcode: Opcode::rsp_vnull,
            opcode_str: "vnull",
            operands_str: [None, None, None, None, None],
            test_encoder: true,
        },
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}

#[cfg(feature = "RSP")]
#[test]
fn check_rsp_instructions_removed() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry::new_rsp_invalid(0x50740008, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x50740008                   /* INVALID / 00740008 <OpcodeCategory: RSP_NORMAL> */"), // beql
        TestEntry::new_rsp_invalid(0x56E1FFF8, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x56E1FFF8                   /* INVALID / 02E1FFF8 <OpcodeCategory: RSP_NORMAL> */"), // bnel
        TestEntry::new_rsp_invalid(0x59C00007, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x59C00007                   /* INVALID / 01C00007 <OpcodeCategory: RSP_NORMAL> */"), // blezl
        TestEntry::new_rsp_invalid(0x5D000001, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x5D000001                   /* INVALID / 01000001 <OpcodeCategory: RSP_NORMAL> */"), // bgtzl
        TestEntry::new_rsp_invalid(0x60840001, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x60840001                   /* INVALID / 00840001 <OpcodeCategory: RSP_NORMAL> */"), // daddi
        TestEntry::new_rsp_invalid(0x64840001, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x64840001                   /* INVALID / 00840001 <OpcodeCategory: RSP_NORMAL> */"), // daddiu
        TestEntry::new_rsp_invalid(0x69220007, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x69220007                   /* INVALID / 01220007 <OpcodeCategory: RSP_NORMAL> */"), // ldl
        TestEntry::new_rsp_invalid(0x6D240008, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x6D240008                   /* INVALID / 01240008 <OpcodeCategory: RSP_NORMAL> */"), // ldr
        TestEntry::new_rsp_invalid(0x88EE000D, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x88EE000D                   /* INVALID / 00EE000D <OpcodeCategory: RSP_NORMAL> */"), // lwl
        TestEntry::new_rsp_invalid(0x98EE0010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x98EE0010                   /* INVALID / 00EE0010 <OpcodeCategory: RSP_NORMAL> */"), // lwr
        TestEntry::new_rsp_invalid(0x9FA30010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x9FA30010                   /* INVALID / 03A30010 <OpcodeCategory: RSP_NORMAL> */"), // lwu
        TestEntry::new_rsp_invalid(0xA8C20000, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xA8C20000                   /* INVALID / 00C20000 <OpcodeCategory: RSP_NORMAL> */"), // swl
        TestEntry::new_rsp_invalid(0xB0C20007, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xB0C20007                   /* INVALID / 00C20007 <OpcodeCategory: RSP_NORMAL> */"), // sdl
        TestEntry::new_rsp_invalid(0xB4C20000, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xB4C20000                   /* INVALID / 00C20000 <OpcodeCategory: RSP_NORMAL> */"), // sdr
        TestEntry::new_rsp_invalid(0xB9C1000E, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xB9C1000E                   /* INVALID / 01C1000E <OpcodeCategory: RSP_NORMAL> */"), // swr
        TestEntry::new_rsp_invalid(0xC0850000, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xC0850000                   /* INVALID / 00850000 <OpcodeCategory: RSP_NORMAL> */"), // ll
        TestEntry::new_rsp_invalid(0xD0850000, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xD0850000                   /* INVALID / 00850000 <OpcodeCategory: RSP_NORMAL> */"), // lld
        TestEntry::new_rsp_invalid(0xDFBF0020, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xDFBF0020                   /* INVALID / 03BF0020 <OpcodeCategory: RSP_NORMAL> */"), // ld
        TestEntry::new_rsp_invalid(0xE0BF0020, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xE0BF0020                   /* INVALID / 00BF0020 <OpcodeCategory: RSP_NORMAL> */"), // sc
        TestEntry::new_rsp_invalid(0xF0BF0020, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xF0BF0020                   /* INVALID / 00BF0020 <OpcodeCategory: RSP_NORMAL> */"), // scd
        TestEntry::new_rsp_invalid(0xFFA00038, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xFFA00038                   /* INVALID / 03A00038 <OpcodeCategory: RSP_NORMAL> */"), // sd
        TestEntry::new_rsp_invalid(0xBCD00000, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xBCD00000                   /* INVALID / 00D00000 <OpcodeCategory: RSP_NORMAL> */"), // cache
        TestEntry::new_rsp_invalid(0xC4D00010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xC4D00010                   /* INVALID / 00D00010 <OpcodeCategory: RSP_NORMAL> */"), // lwc1
        TestEntry::new_rsp_invalid(0xD4D00010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xD4D00010                   /* INVALID / 00D00010 <OpcodeCategory: RSP_NORMAL> */"), // ldc1
        TestEntry::new_rsp_invalid(0xE4D00010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xE4D00010                   /* INVALID / 00D00010 <OpcodeCategory: RSP_NORMAL> */"), // swc1
        TestEntry::new_rsp_invalid(0xF4D00010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xF4D00010                   /* INVALID / 00D00010 <OpcodeCategory: RSP_NORMAL> */"), // sdc1
        // TestEntry::new_rsp_invalid(0xC8D00010, InstructionFlags::new_extension(IsaExtension::RSP), ""), // lwc2
        TestEntry::new_rsp_invalid(0xD8D00010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xD8D00010                   /* INVALID / 00D00010 <OpcodeCategory: RSP_NORMAL> */"), // ldc2
        // TestEntry::new_rsp_invalid(0xE8D00010, InstructionFlags::new_extension(IsaExtension::RSP), ""), // swc2
        TestEntry::new_rsp_invalid(0xF8D00010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xF8D00010                   /* INVALID / 00D00010 <OpcodeCategory: RSP_NORMAL> */"), // sdc2
        TestEntry::new_rsp_invalid(0x05020031, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x05020031                   /* INVALID / 01000031 <OpcodeCategory: RSP_REGIMM> */"), // bltzl
        TestEntry::new_rsp_invalid(0x04630005, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04630005                   /* INVALID / 00600005 <OpcodeCategory: RSP_REGIMM> */"), // bgezl
        TestEntry::new_rsp_invalid(0x04A80010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04A80010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // tgei
        TestEntry::new_rsp_invalid(0x04A90010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04A90010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // tgeiu
        TestEntry::new_rsp_invalid(0x04AA0010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04AA0010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // tlti
        TestEntry::new_rsp_invalid(0x04AB0010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04AB0010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // tltiu
        TestEntry::new_rsp_invalid(0x04AC0010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04AC0010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // teqi
        TestEntry::new_rsp_invalid(0x04AE0010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04AE0010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // tnei
        TestEntry::new_rsp_invalid(0x04B20010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04B20010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // bltzall
        TestEntry::new_rsp_invalid(0x04B30010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x04B30010                   /* INVALID / 00A00010 <OpcodeCategory: RSP_REGIMM> */"), // bgezall
        TestEntry::new_rsp_invalid(0x00042FF8, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00042FF8                   /* INVALID / 00042FC0 <OpcodeCategory: RSP_SPECIAL> */"), // dsll
        TestEntry::new_rsp_invalid(0x000637FA, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x000637FA                   /* INVALID / 000637C0 <OpcodeCategory: RSP_SPECIAL> */"), // dsrl
        TestEntry::new_rsp_invalid(0x0002137B, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0002137B                   /* INVALID / 00021340 <OpcodeCategory: RSP_SPECIAL> */"), // dsra
        TestEntry::new_rsp_invalid(0x000437FC, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x000437FC                   /* INVALID / 000437C0 <OpcodeCategory: RSP_SPECIAL> */"), // dsll32
        TestEntry::new_rsp_invalid(0x0005283E, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0005283E                   /* INVALID / 00052800 <OpcodeCategory: RSP_SPECIAL> */"), // dsrl32
        TestEntry::new_rsp_invalid(0x0002103F, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0002103F                   /* INVALID / 00021000 <OpcodeCategory: RSP_SPECIAL> */"), // dsra32
        TestEntry::new_rsp_invalid(0x01EE1014, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x01EE1014                   /* INVALID / 01EE1000 <OpcodeCategory: RSP_SPECIAL> */"), // dsllv
        TestEntry::new_rsp_invalid(0x01EE1016, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x01EE1016                   /* INVALID / 01EE1000 <OpcodeCategory: RSP_SPECIAL> */"), // dsrlv
        TestEntry::new_rsp_invalid(0x01EE1017, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x01EE1017                   /* INVALID / 01EE1000 <OpcodeCategory: RSP_SPECIAL> */"), // dsrav
        TestEntry::new_rsp_invalid(0x03600011, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x03600011                   /* INVALID / 03600000 <OpcodeCategory: RSP_SPECIAL> */"), // mthi
        TestEntry::new_rsp_invalid(0x03600013, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x03600013                   /* INVALID / 03600000 <OpcodeCategory: RSP_SPECIAL> */"), // mtlo
        TestEntry::new_rsp_invalid(0x00004010, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00004010                   /* INVALID / 00004000 <OpcodeCategory: RSP_SPECIAL> */"), // mfhi
        TestEntry::new_rsp_invalid(0x00004012, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00004012                   /* INVALID / 00004000 <OpcodeCategory: RSP_SPECIAL> */"), // mflo
        TestEntry::new_rsp_invalid(0x01AC001A, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x01AC001A                   /* INVALID / 01AC0000 <OpcodeCategory: RSP_SPECIAL> */"), // div
        TestEntry::new_rsp_invalid(0x0101001B, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0101001B                   /* INVALID / 01010000 <OpcodeCategory: RSP_SPECIAL> */"), // divu
        TestEntry::new_rsp_invalid(0x01CF001E, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x01CF001E                   /* INVALID / 01CF0000 <OpcodeCategory: RSP_SPECIAL> */"), // ddiv
        TestEntry::new_rsp_invalid(0x01CF001F, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x01CF001F                   /* INVALID / 01CF0000 <OpcodeCategory: RSP_SPECIAL> */"), // ddivu
        TestEntry::new_rsp_invalid(0x0162582C, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0162582C                   /* INVALID / 01625800 <OpcodeCategory: RSP_SPECIAL> */"), // dadd
        TestEntry::new_rsp_invalid(0x012A582D, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x012A582D                   /* INVALID / 012A5800 <OpcodeCategory: RSP_SPECIAL> */"), // daddu
        TestEntry::new_rsp_invalid(0x0162582E, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0162582E                   /* INVALID / 01625800 <OpcodeCategory: RSP_SPECIAL> */"), // dsub
        TestEntry::new_rsp_invalid(0x0162582F, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0162582F                   /* INVALID / 01625800 <OpcodeCategory: RSP_SPECIAL> */"), // dsubu
        TestEntry::new_rsp_invalid(0x0000000C, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0000000C                   /* INVALID / 00000000 <OpcodeCategory: RSP_SPECIAL> */"), // syscall
        TestEntry::new_rsp_invalid(0x0000000F, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0000000F                   /* INVALID / 00000000 <OpcodeCategory: RSP_SPECIAL> */"), // sync
        TestEntry::new_rsp_invalid(0x00870018, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00870018                   /* INVALID / 00870000 <OpcodeCategory: RSP_SPECIAL> */"), // mult
        TestEntry::new_rsp_invalid(0x00E20019, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E20019                   /* INVALID / 00E20000 <OpcodeCategory: RSP_SPECIAL> */"), // multu
        TestEntry::new_rsp_invalid(0x0087001C, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x0087001C                   /* INVALID / 00870000 <OpcodeCategory: RSP_SPECIAL> */"), // dmult
        TestEntry::new_rsp_invalid(0x00E2001D, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E2001D                   /* INVALID / 00E20000 <OpcodeCategory: RSP_SPECIAL> */"), // dmultu
        TestEntry::new_rsp_invalid(0x00E200B0, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E200B0                   /* INVALID / 00E20080 <OpcodeCategory: RSP_SPECIAL> */"), // tge
        TestEntry::new_rsp_invalid(0x00E200B1, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E200B1                   /* INVALID / 00E20080 <OpcodeCategory: RSP_SPECIAL> */"), // tgeu
        TestEntry::new_rsp_invalid(0x00E200B2, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E200B2                   /* INVALID / 00E20080 <OpcodeCategory: RSP_SPECIAL> */"), // tlt
        TestEntry::new_rsp_invalid(0x00E200B3, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E200B3                   /* INVALID / 00E20080 <OpcodeCategory: RSP_SPECIAL> */"), // tltu
        TestEntry::new_rsp_invalid(0x00E200B4, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E200B4                   /* INVALID / 00E20080 <OpcodeCategory: RSP_SPECIAL> */"), // teq
        TestEntry::new_rsp_invalid(0x00E200B6, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x00E200B6                   /* INVALID / 00E20080 <OpcodeCategory: RSP_SPECIAL> */"), // tne
        TestEntry::new_rsp_invalid(0x40220800, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x40220800                   /* INVALID / 00020800 <OpcodeCategory: RSP_COP0> */"), // dmfc0
        TestEntry::new_rsp_invalid(0x40420800, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x40420800                   /* INVALID / 00020800 <OpcodeCategory: RSP_COP0> */"), // cfc0
        TestEntry::new_rsp_invalid(0x40A20800, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x40A20800                   /* INVALID / 00020800 <OpcodeCategory: RSP_COP0> */"), // dmtc0
        TestEntry::new_rsp_invalid(0x40C20800, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x40C20800                   /* INVALID / 00020800 <OpcodeCategory: RSP_COP0> */"), // ctc0
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}

#[cfg(feature = "RSP")]
#[test]
fn check_rsp_instructions_invalid() {
    const ENTRIES: &[TestEntry] = &[
        TestEntry::new_rsp_invalid(0x44444444, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x44444444                   /* INVALID / 00444444 <OpcodeCategory: RSP_COP1> */"),
        TestEntry::new_rsp_invalid(0x77777777, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0x77777777                   /* INVALID / 03777777 <OpcodeCategory: CORE_NORMAL> */"),
        TestEntry::new_rsp_invalid(0xEEEEEEEE, InstructionFlags::new_extension(IsaExtension::RSP), ".word       0xEEEEEEEE                   /* INVALID / 02EEEEEE <OpcodeCategory: CORE_NORMAL> */"),
    ];

    assert_eq!(check_test_entries(ENTRIES), (0, 0));
}
