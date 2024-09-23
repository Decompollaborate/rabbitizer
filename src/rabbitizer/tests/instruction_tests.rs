/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(test)]
mod tests {
    use crate::{operand, Instruction, InstructionFlags, Opcode};

    struct TestEntry {
        pub instr: Instruction,
        pub valid: bool,
        pub expected: &'static str,
        pub expected_opcode: Opcode,
        pub opcode_str: &'static str,
        pub operands_str: [Option<&'static str>; operand::OPERAND_COUNT_MAX],
    }

    impl TestEntry {
        pub const fn new_rsp_invalid(word: u32, flags: Option<InstructionFlags>) -> Self {
            Self {
                instr: Instruction::new_rsp(word, 0xA4000000, flags),
                valid: false,
                expected: "INVALID",
                expected_opcode: Opcode::ALL_INVALID,
                opcode_str: "INVALID",
                operands_str: [None, None, None, None, None],
            }
        }
    }

    fn entries_sanity_check(entries: &[TestEntry]) {
        for (i, x) in entries.iter().enumerate() {
            for (_j, y) in entries[i + 1..].iter().enumerate() {
                assert_ne!(x.instr.word(), y.instr.word());
            }
        }
    }

    fn check_test_entries(entries: &[TestEntry]) -> u32 {
        let mut errors = 0;

        entries_sanity_check(entries);

        for entry in entries {
            if entry.instr.is_valid() != entry.valid {
                println!(
                    "'{}' ({:08X}) has incorrect validity. Expected '{}', got '{}'",
                    entry.opcode_str,
                    entry.instr.word(),
                    entry.valid,
                    entry.instr.is_valid()
                );
                println!("  {}", entry.expected);
                println!("  {:?}", entry.operands_str);
                errors += 1;
            }
            if entry.instr.opcode() != entry.expected_opcode {
                println!(
                    "'{}' ({:08X}) has incorrect decoded opcode. Expected '{:?}', got '{:?}'",
                    entry.opcode_str,
                    entry.instr.word(),
                    entry.expected_opcode,
                    entry.instr.opcode()
                );
                errors += 1;
            }
            if entry.instr.opcode().name() != entry.opcode_str {
                println!(
                    "'{}' ({:08X}) has incorrect opcode name. Expected '{}', got '{}'",
                    entry.opcode_str,
                    entry.instr.word(),
                    entry.opcode_str,
                    entry.instr.opcode().name()
                );
                errors += 1;
            }
            // TODO: expected
            // TODO: operands_str
        }

        errors
    }

    #[test]
    fn check_rsp_instructions() {
        const ENTRIES: &[TestEntry] = &[
            TestEntry {
                instr: Instruction::new_rsp(0x09000419, 0xA4000000, None),
                valid: true,
                expected: "j           func_A4001064",
                expected_opcode: Opcode::rsp_j,
                opcode_str: "j",
                operands_str: [Some("func_A4001064"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x21490000, 0xA4000000, None),
                valid: true,
                expected: "addi        $9, $10, 0x0",
                expected_opcode: Opcode::rsp_addi,
                opcode_str: "addi",
                operands_str: [Some("$9"), Some("$10"), Some("0x0"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x8C060578, 0xA4000000, None),
                valid: true,
                expected: "lw          $6, 0x578($zero)",
                expected_opcode: Opcode::rsp_lw,
                opcode_str: "lw",
                operands_str: [Some("$6"), Some("0x578($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x400B2800, 0xA4000000, None),
                valid: true,
                expected: "mfc0        $11, SP_DMA_FULL",
                expected_opcode: Opcode::rsp_mfc0,
                opcode_str: "mfc0",
                operands_str: [Some("$11"), Some("SP_DMA_FULL"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x304203FF, 0xA4000000, None),
                valid: true,
                expected: "andi        $2, $2, 0x3FF",
                expected_opcode: Opcode::rsp_andi,
                opcode_str: "andi",
                operands_str: [Some("$2"), Some("$2"), Some("0x3FF"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x10400003, 0xA4000000, None),
                valid: true,
                expected: "beqz        $2, .L800B38E8",
                expected_opcode: Opcode::rsp_beqz,
                opcode_str: "beqz",
                operands_str: [Some("$2"), Some(".L800B38E8"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00000000, 0xA4000000, None),
                valid: true,
                expected: "nop",
                expected_opcode: Opcode::rsp_nop,
                opcode_str: "nop",
                operands_str: [None, None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1C600033, 0xA4000000, None),
                valid: true,
                expected: "bgtz        $3, .L800B3A74",
                expected_opcode: Opcode::rsp_bgtz,
                opcode_str: "bgtz",
                operands_str: [Some("$3"), Some(".L800B3A74"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x0D00077A, 0xA4000000, None),
                valid: true,
                expected: "jal         func_84001DE8",
                expected_opcode: Opcode::rsp_jal,
                opcode_str: "jal",
                operands_str: [Some("func_84001DE8"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xAEEB000C, 0xA4000000, None),
                valid: true,
                expected: "sw          $11, 0xC($23)",
                expected_opcode: Opcode::rsp_sw,
                opcode_str: "sw",
                operands_str: [Some("$11"), Some("0xC($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1560FB8D, 0xA4000000, None),
                valid: true,
                expected: "bnez        $11, .L800B2288",
                expected_opcode: Opcode::rsp_bnez,
                opcode_str: "bnez",
                operands_str: [Some("$11"), Some(".L800B2288"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x40921800, 0xA4000000, None),
                valid: true,
                expected: "mtc0        $18, SP_WR_LEN",
                expected_opcode: Opcode::rsp_mtc0,
                opcode_str: "mtc0",
                operands_str: [Some("$18"), Some("SP_WR_LEN"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00026A82, 0xA4000000, None),
                valid: true,
                expected: "srl         $13, $2, 10",
                expected_opcode: Opcode::rsp_srl,
                opcode_str: "srl",
                operands_str: [Some("$13"), Some("$2"), Some("10"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x004F1020, 0xA4000000, None),
                valid: true,
                expected: "add         $2, $2, $15",
                expected_opcode: Opcode::rsp_add,
                opcode_str: "add",
                operands_str: [Some("$2"), Some("$2"), Some("$15"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x84040572, 0xA4000000, None),
                valid: true,
                expected: "lh          $4, 0x572($zero)",
                expected_opcode: Opcode::rsp_lh,
                opcode_str: "lh",
                operands_str: [Some("$4"), Some("0x572($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x03E00008, 0xA4000000, None),
                valid: true,
                expected: "jr          $ra",
                expected_opcode: Opcode::rsp_jr,
                opcode_str: "jr",
                operands_str: [Some("$ra"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x0000000D, 0xA4000000, None),
                valid: true,
                expected: "break       0",
                expected_opcode: Opcode::rsp_break,
                opcode_str: "break",
                operands_str: [Some("0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x19C0FA06, 0xA4000000, None),
                valid: true,
                expected: "blez        $14, .L800B2288",
                expected_opcode: Opcode::rsp_blez,
                opcode_str: "blez",
                operands_str: [Some("$14"), Some(".L800B2288"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x09000A19, 0xA4000000, None),
                valid: true,
                expected: "j           func_84002864",
                expected_opcode: Opcode::rsp_j,
                opcode_str: "j",
                operands_str: [Some("func_84002864"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x37120000, 0xA4000000, None),
                valid: true,
                expected: "ori         $18, $24, 0x0",
                expected_opcode: Opcode::rsp_ori,
                opcode_str: "ori",
                operands_str: [Some("$18"), Some("$24"), Some("0x0"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1000FE72, 0xA4000000, None),
                valid: true,
                expected: "b           .L800B1A78",
                expected_opcode: Opcode::rsp_b,
                opcode_str: "b",
                operands_str: [Some(".L800B1A78"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A0318EC, 0xA4000000, None),
                valid: true,
                expected: "vxor        $v3, $v3, $v3",
                expected_opcode: Opcode::rsp_vxor,
                opcode_str: "vxor",
                operands_str: [Some("$v3"), Some("$v3"), Some("$v3"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAF11B0B, 0xA4000000, None),
                valid: true,
                expected: "sdv         $v17[6], 0x58($23)",
                expected_opcode: Opcode::rsp_sdv,
                opcode_str: "sdv",
                operands_str: [Some("$v17[6]"), Some("0x58($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x940C0572, 0xA4000000, None),
                valid: true,
                expected: "lhu         $12, 0x572($zero)",
                expected_opcode: Opcode::rsp_lhu,
                opcode_str: "lhu",
                operands_str: [Some("$12"), Some("0x572($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00095880, 0xA4000000, None),
                valid: true,
                expected: "sll         $11, $9, 2",
                expected_opcode: Opcode::rsp_sll,
                opcode_str: "sll",
                operands_str: [Some("$11"), Some("$9"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xA403057C, 0xA4000000, None),
                valid: true,
                expected: "sh          $3, 0x57C($zero)",
                expected_opcode: Opcode::rsp_sh,
                opcode_str: "sh",
                operands_str: [Some("$3"), Some("0x57C($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xCA832000, 0xA4000000, None),
                valid: true,
                expected: "lqv         $v3[0], 0x0($20)",
                expected_opcode: Opcode::rsp_lqv,
                opcode_str: "lqv",
                operands_str: [Some("$v3[0]"), Some("0x0($20)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAE40C0B, 0xA4000000, None),
                valid: true,
                expected: "ssv         $v4[8], 0x16($23)",
                expected_opcode: Opcode::rsp_ssv,
                opcode_str: "ssv",
                operands_str: [Some("$v4[8]"), Some("0x16($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xCBC41807, 0xA4000000, None),
                valid: true,
                expected: "ldv         $v4[0], 0x38($30)",
                expected_opcode: Opcode::rsp_ldv,
                opcode_str: "ldv",
                operands_str: [Some("$v4[0]"), Some("0x38($30)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x3C07F510, 0xA4000000, None),
                valid: true,
                expected: "lui         $7, (0xF5100000 >> 16)",
                expected_opcode: Opcode::rsp_lui,
                opcode_str: "lui",
                operands_str: [Some("$7"), Some("(0xF5100000 >> 16)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00611824, 0xA4000000, None),
                valid: true,
                expected: "and         $3, $3, $1",
                expected_opcode: Opcode::rsp_and,
                opcode_str: "and",
                operands_str: [Some("$3"), Some("$3"), Some("$1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEBC62000, 0xA4000000, None),
                valid: true,
                expected: "sqv         $v6[0], 0x0($30)",
                expected_opcode: Opcode::rsp_sqv,
                opcode_str: "sqv",
                operands_str: [Some("$v6[0]"), Some("0x0($30)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1193FFFE, 0xA4000000, None),
                valid: true,
                expected: "beq         $12, $19, .L800B304C",
                expected_opcode: Opcode::rsp_beq,
                opcode_str: "beq",
                operands_str: [Some("$12"), Some("$19"), Some(".L800B304C"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x900B0539, 0xA4000000, None),
                valid: true,
                expected: "lbu         $11, 0x539($zero)",
                expected_opcode: Opcode::rsp_lbu,
                opcode_str: "lbu",
                operands_str: [Some("$11"), Some("0x539($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0C58A8, 0xA4000000, None),
                valid: true,
                expected: "vand        $v2, $v11, $v12[0]",
                expected_opcode: Opcode::rsp_vand,
                opcode_str: "vand",
                operands_str: [Some("$v2"), Some("$v11"), Some("$v12[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x008C5822, 0xA4000000, None),
                valid: true,
                expected: "sub         $11, $4, $12",
                expected_opcode: Opcode::rsp_sub,
                opcode_str: "sub",
                operands_str: [Some("$11"), Some("$4"), Some("$12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00432006, 0xA4000000, None),
                valid: true,
                expected: "srlv        $4, $3, $2",
                expected_opcode: Opcode::rsp_srlv,
                opcode_str: "srlv",
                operands_str: [Some("$4"), Some("$3"), Some("$2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x488ED800, 0xA4000000, None),
                valid: true,
                expected: "mtc2        $14, $v27[0]",
                expected_opcode: Opcode::rsp_mtc2,
                opcode_str: "mtc2",
                operands_str: [Some("$14"), Some("$v27[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC9B12802, 0xA4000000, None),
                valid: true,
                expected: "lrv         $v17[0], 0x20($13)",
                expected_opcode: Opcode::rsp_lrv,
                opcode_str: "lrv",
                operands_str: [Some("$v17[0]"), Some("0x20($13)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1B21C6, 0xA4000000, None),
                valid: true,
                expected: "vmudn       $v7, $v4, $v27[0]",
                expected_opcode: Opcode::rsp_vmudn,
                opcode_str: "vmudn",
                operands_str: [Some("$v7"), Some("$v4"), Some("$v27[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7F488E, 0xA4000000, None),
                valid: true,
                expected: "vmadn       $v2, $v9, $v31[3]",
                expected_opcode: Opcode::rsp_vmadn,
                opcode_str: "vmadn",
                operands_str: [Some("$v2"), Some("$v9"), Some("$v31[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B2A3A05, 0xA4000000, None),
                valid: true,
                expected: "vmudm       $v8, $v7, $v10[1]",
                expected_opcode: Opcode::rsp_vmudm,
                opcode_str: "vmudm",
                operands_str: [Some("$v8"), Some("$v7"), Some("$v10[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B3D1EC7, 0xA4000000, None),
                valid: true,
                expected: "vmudh       $v27, $v3, $v29[1]",
                expected_opcode: Opcode::rsp_vmudh,
                opcode_str: "vmudh",
                operands_str: [Some("$v27"), Some("$v3"), Some("$v29[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7DFA0F, 0xA4000000, None),
                valid: true,
                expected: "vmadh       $v8, $v31, $v29[3]",
                expected_opcode: Opcode::rsp_vmadh,
                opcode_str: "vmadh",
                operands_str: [Some("$v8"), Some("$v31"), Some("$v29[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0A529D, 0xA4000000, None),
                valid: true,
                expected: "vsar        $v10, $v10, $v10[0]",
                expected_opcode: Opcode::rsp_vsar,
                opcode_str: "vsar",
                operands_str: [Some("$v10"), Some("$v10"), Some("$v10[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAFD1204, 0xA4000000, None),
                valid: true,
                expected: "slv         $v29[4], 0x10($23)",
                expected_opcode: Opcode::rsp_slv,
                opcode_str: "slv",
                operands_str: [Some("$v29[4]"), Some("0x10($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC827080F, 0xA4000000, None),
                valid: true,
                expected: "lsv         $v7[0], 0x1E($1)",
                expected_opcode: Opcode::rsp_lsv,
                opcode_str: "lsv",
                operands_str: [Some("$v7[0]"), Some("0x1E($1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4BAA4351, 0xA4000000, None),
                valid: true,
                expected: "vsub        $v13, $v8, $v10[5]",
                expected_opcode: Opcode::rsp_vsub,
                opcode_str: "vsub",
                operands_str: [Some("$v13"), Some("$v8"), Some("$v10[5]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F8ECD, 0xA4000000, None),
                valid: true,
                expected: "vmadm       $v27, $v17, $v31[0]",
                expected_opcode: Opcode::rsp_vmadm,
                opcode_str: "vmadm",
                operands_str: [Some("$v27"), Some("$v17"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B3F7384, 0xA4000000, None),
                valid: true,
                expected: "vmudl       $v14, $v14, $v31[1]",
                expected_opcode: Opcode::rsp_vmudl,
                opcode_str: "vmudl",
                operands_str: [Some("$v14"), Some("$v14"), Some("$v31[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B9F2940, 0xA4000000, None),
                valid: true,
                expected: "vmulf       $v5, $v5, $v31[4]",
                expected_opcode: Opcode::rsp_vmulf,
                opcode_str: "vmulf",
                operands_str: [Some("$v5"), Some("$v5"), Some("$v31[4]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4BAA4390, 0xA4000000, None),
                valid: true,
                expected: "vadd        $v14, $v8, $v10[5]",
                expected_opcode: Opcode::rsp_vadd,
                opcode_str: "vadd",
                operands_str: [Some("$v14"), Some("$v8"), Some("$v10[5]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4813D900, 0xA4000000, None),
                valid: true,
                expected: "mfc2        $19, $v27[2]",
                expected_opcode: Opcode::rsp_mfc2,
                opcode_str: "mfc2",
                operands_str: [Some("$19"), Some("$v27[2]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A1174D5, 0xA4000000, None),
                valid: true,
                expected: "vsubc       $v19, $v14, $v17",
                expected_opcode: Opcode::rsp_vsubc,
                opcode_str: "vsubc",
                operands_str: [Some("$v19"), Some("$v14"), Some("$v17"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7D6EE3, 0xA4000000, None),
                valid: true,
                expected: "vge         $v27, $v13, $v29[3]",
                expected_opcode: Opcode::rsp_vge,
                opcode_str: "vge",
                operands_str: [Some("$v27"), Some("$v13"), Some("$v29[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4AF4E0E4, 0xA4000000, None),
                valid: true,
                expected: "vcl         $v3, $v28, $v20[3h]",
                expected_opcode: Opcode::rsp_vcl,
                opcode_str: "vcl",
                operands_str: [Some("$v3"), Some("$v28"), Some("$v20[3h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4AC550C8, 0xA4000000, None),
                valid: true,
                expected: "vmacf       $v3, $v10, $v5[2h]",
                expected_opcode: Opcode::rsp_vmacf,
                opcode_str: "vmacf",
                operands_str: [Some("$v3"), Some("$v10"), Some("$v5[2h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x15610003, 0xA4000000, None),
                valid: true,
                expected: "bne         $11, $1, .L800B2F4C",
                expected_opcode: Opcode::rsp_bne,
                opcode_str: "bne",
                operands_str: [Some("$11"), Some("$1"), Some(".L800B2F4C"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x05C1000F, 0xA4000000, None),
                valid: true,
                expected: "bgez        $14, .L800B3A2C",
                expected_opcode: Opcode::rsp_bgez,
                opcode_str: "bgez",
                operands_str: [Some("$14"), Some(".L800B3A2C"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAFD0688, 0xA4000000, None),
                valid: true,
                expected: "sbv         $v29[13], 0x8($23)",
                expected_opcode: Opcode::rsp_sbv,
                opcode_str: "sbv",
                operands_str: [Some("$v29[13]"), Some("0x8($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x01675825, 0xA4000000, None),
                valid: true,
                expected: "or          $11, $11, $7",
                expected_opcode: Opcode::rsp_or,
                opcode_str: "or",
                operands_str: [Some("$11"), Some("$11"), Some("$7"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x03241804, 0xA4000000, None),
                valid: true,
                expected: "sllv        $3, $4, $25",
                expected_opcode: Opcode::rsp_sllv,
                opcode_str: "sllv",
                operands_str: [Some("$3"), Some("$4"), Some("$25"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00601827, 0xA4000000, None),
                valid: true,
                expected: "not         $3, $3",
                expected_opcode: Opcode::rsp_not,
                opcode_str: "not",
                operands_str: [Some("$3"), Some("$3"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x83790500, 0xA4000000, None),
                valid: true,
                expected: "lb          $25, 0x500($27)",
                expected_opcode: Opcode::rsp_lb,
                opcode_str: "lb",
                operands_str: [Some("$25"), Some("0x500($27)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x05600002, 0xA4000000, None),
                valid: true,
                expected: "bltz        $11, .L800B3A2C",
                expected_opcode: Opcode::rsp_bltz,
                opcode_str: "bltz",
                operands_str: [Some("$11"), Some(".L800B3A2C"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xA2EB0009, 0xA4000000, None),
                valid: true,
                expected: "sb          $11, 0x9($23)",
                expected_opcode: Opcode::rsp_sb,
                opcode_str: "sb",
                operands_str: [Some("$11"), Some("0x9($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x000B5A83, 0xA4000000, None),
                valid: true,
                expected: "sra         $11, $11, 10",
                expected_opcode: Opcode::rsp_sra,
                opcode_str: "sra",
                operands_str: [Some("$11"), Some("$11"), Some("10"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC98C1000, 0xA4000000, None),
                valid: true,
                expected: "llv         $v12[0], 0x0($12)",
                expected_opcode: Opcode::rsp_llv,
                opcode_str: "llv",
                operands_str: [Some("$v12[0]"), Some("0x0($12)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4AF5E8E5, 0xA4000000, None),
                valid: true,
                expected: "vch         $v3, $v29, $v21[3h]",
                expected_opcode: Opcode::rsp_vch,
                opcode_str: "vch",
                operands_str: [Some("$v3"), Some("$v29"), Some("$v21[3h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x484B0800, 0xA4000000, None),
                valid: true,
                expected: "cfc2        $t3, $1",
                expected_opcode: Opcode::rsp_cfc2,
                opcode_str: "cfc2",
                operands_str: [Some("$t3"), Some("$1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A086A27, 0xA4000000, None),
                valid: true,
                expected: "vmrg        $v8, $v13, $v8",
                expected_opcode: Opcode::rsp_vmrg,
                opcode_str: "vmrg",
                operands_str: [Some("$v8"), Some("$v13"), Some("$v8"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7D7EE0, 0xA4000000, None),
                valid: true,
                expected: "vlt         $v27, $v15, $v29[3]",
                expected_opcode: Opcode::rsp_vlt,
                opcode_str: "vlt",
                operands_str: [Some("$v27"), Some("$v15"), Some("$v29[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x26F70018, 0xA4000000, None),
                valid: true,
                expected: "addiu       $23, $23, 0x18",
                expected_opcode: Opcode::rsp_addiu,
                opcode_str: "addiu",
                operands_str: [Some("$23"), Some("$23"), Some("0x18"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00812021, 0xA4000000, None),
                valid: true,
                expected: "addu        $4, $4, $1",
                expected_opcode: Opcode::rsp_addu,
                opcode_str: "addu",
                operands_str: [Some("$4"), Some("$4"), Some("$1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F6A0C, 0xA4000000, None),
                valid: true,
                expected: "vmadl       $v8, $v13, $v31[0]",
                expected_opcode: Opcode::rsp_vmadl,
                opcode_str: "vmadl",
                operands_str: [Some("$v8"), Some("$v13"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x0185602A, 0xA4000000, None),
                valid: true,
                expected: "slt         $12, $12, $5",
                expected_opcode: Opcode::rsp_slt,
                opcode_str: "slt",
                operands_str: [Some("$12"), Some("$12"), Some("$5"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B224AF3, 0xA4000000, None),
                valid: true,
                expected: "vmov        $v11[1], $v2[1]",
                expected_opcode: Opcode::rsp_vmov,
                opcode_str: "vmov",
                operands_str: [Some("$v11[1]"), Some("$v2[1]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0343F0, 0xA4000000, None),
                valid: true,
                expected: "vrcp        $v15[0], $v3[0]",
                expected_opcode: Opcode::rsp_vrcp,
                opcode_str: "vrcp",
                operands_str: [Some("$v15[0]"), Some("$v3[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7D4232, 0xA4000000, None),
                valid: true,
                expected: "vrcph       $v8[0], $v29[3]",
                expected_opcode: Opcode::rsp_vrcph,
                opcode_str: "vrcph",
                operands_str: [Some("$v8[0]"), Some("$v29[3]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4BDE0026, 0xA4000000, None),
                valid: true,
                expected: "vcr         $v0, $v0, $v30[6]",
                expected_opcode: Opcode::rsp_vcr,
                opcode_str: "vcr",
                operands_str: [Some("$v0"), Some("$v0"), Some("$v30[6]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A0A56D3, 0xA4000000, None),
                valid: true,
                expected: "vabs        $v27, $v10, $v10",
                expected_opcode: Opcode::rsp_vabs,
                opcode_str: "vabs",
                operands_str: [Some("$v27"), Some("$v10"), Some("$v10"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC9513800, 0xA4000000, None),
                valid: true,
                expected: "luv         $v17[0], 0x0($10)",
                expected_opcode: Opcode::rsp_luv,
                opcode_str: "luv",
                operands_str: [Some("$v17[0]"), Some("0x0($10)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0D41F1, 0xA4000000, None),
                valid: true,
                expected: "vrcpl       $v7[0], $v13[0]",
                expected_opcode: Opcode::rsp_vrcpl,
                opcode_str: "vrcpl",
                operands_str: [Some("$v7[0]"), Some("$v13[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x398C0001, 0xA4000000, None),
                valid: true,
                expected: "xori        $12, $12, 0x1",
                expected_opcode: Opcode::rsp_xori,
                opcode_str: "xori",
                operands_str: [Some("$12"), Some("$12"), Some("0x1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00641826, 0xA4000000, None),
                valid: true,
                expected: "xor         $3, $3, $4",
                expected_opcode: Opcode::rsp_xor,
                opcode_str: "xor",
                operands_str: [Some("$3"), Some("$3"), Some("$4"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4ACB36D4, 0xA4000000, None),
                valid: true,
                expected: "vaddc       $v27, $v6, $v11[2h]",
                expected_opcode: Opcode::rsp_vaddc,
                opcode_str: "vaddc",
                operands_str: [Some("$v27"), Some("$v6"), Some("$v11[2h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F4221, 0xA4000000, None),
                valid: true,
                expected: "veq         $v8, $v8, $v31[0]",
                expected_opcode: Opcode::rsp_veq,
                opcode_str: "veq",
                operands_str: [Some("$v8"), Some("$v8"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F7BE2, 0xA4000000, None),
                valid: true,
                expected: "vne         $v15, $v15, $v31[0]",
                expected_opcode: Opcode::rsp_vne,
                opcode_str: "vne",
                operands_str: [Some("$v15"), Some("$v15"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F7A2D, 0xA4000000, None),
                valid: true,
                expected: "vnxor       $v8, $v15, $v31[0]",
                expected_opcode: Opcode::rsp_vnxor,
                opcode_str: "vnxor",
                operands_str: [Some("$v8"), Some("$v15"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xE8FB3800, 0xA4000000, None),
                valid: true,
                expected: "suv         $v27[0], 0x0($7)",
                expected_opcode: Opcode::rsp_suv,
                opcode_str: "suv",
                operands_str: [Some("$v27[0]"), Some("0x0($7)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x03C0F809, 0xA4000000, None),
                valid: true,
                expected: "jalr        $30",
                expected_opcode: Opcode::rsp_jalr,
                opcode_str: "jalr",
                operands_str: [Some("$30"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC86F3000, 0xA4000000, None),
                valid: true,
                expected: "lpv         $v15[0], 0x0($3)",
                expected_opcode: Opcode::rsp_lpv,
                opcode_str: "lpv",
                operands_str: [Some("$v15[0]"), Some("0x0($3)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A1FEF6A, 0xA4000000, None),
                valid: true,
                expected: "vor         $v29, $v29, $v31",
                expected_opcode: Opcode::rsp_vor,
                opcode_str: "vor",
                operands_str: [Some("$v29"), Some("$v29"), Some("$v31"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xE9085F04, 0xA4000000, None),
                valid: true,
                expected: "stv         $v8[14], 0x40($8)",
                expected_opcode: Opcode::rsp_stv,
                opcode_str: "stv",
                operands_str: [Some("$v8[14]"), Some("0x40($8)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC9085904, 0xA4000000, None),
                valid: true,
                expected: "ltv         $v8[2], 0x40($8)",
                expected_opcode: Opcode::rsp_ltv,
                opcode_str: "ltv",
                operands_str: [Some("$v8[2]"), Some("0x40($8)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F42F6, 0xA4000000, None),
                valid: true,
                expected: "vrsqh       $v11[0], $v31[0]",
                expected_opcode: Opcode::rsp_vrsqh,
                opcode_str: "vrsqh",
                operands_str: [Some("$v11[0]"), Some("$v31[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B4743F5, 0xA4000000, None),
                valid: true,
                expected: "vrsql       $v15[0], $v7[2]",
                expected_opcode: Opcode::rsp_vrsql,
                opcode_str: "vrsql",
                operands_str: [Some("$v15[0]"), Some("$v7[2]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xE8273038, 0xA4000000, None),
                valid: true,
                expected: "spv         $v7[0], 0x1C0($1)",
                expected_opcode: Opcode::rsp_spv,
                opcode_str: "spv",
                operands_str: [Some("$v7[0]"), Some("0x1C0($1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC94F0786, 0xA4000000, None),
                valid: true,
                expected: "lbv         $v15[15], 0x6($10)",
                expected_opcode: Opcode::rsp_lbv,
                opcode_str: "lbv",
                operands_str: [Some("$v15[15]"), Some("0x6($10)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x48CB0800, 0xA4000000, None),
                valid: true,
                expected: "ctc2        $t3, $1",
                expected_opcode: Opcode::rsp_ctc2,
                opcode_str: "ctc2",
                operands_str: [Some("$t3"), Some("$1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B3E01EB, 0xA4000000, None),
                valid: true,
                expected: "vnor        $v7, $v0, $v30[1]",
                expected_opcode: Opcode::rsp_vnor,
                opcode_str: "vnor",
                operands_str: [Some("$v7"), Some("$v0"), Some("$v30[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x04D1FF9D, 0xA4000000, None),
                valid: true,
                expected: "bgezal      $6, .L800B3020",
                expected_opcode: Opcode::rsp_bgezal,
                opcode_str: "bgezal",
                operands_str: [Some("$6"), Some(".L800B3020"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00035822, 0xA4000000, None),
                valid: true,
                expected: "neg         $11, $3",
                expected_opcode: Opcode::rsp_neg,
                opcode_str: "neg",
                operands_str: [Some("$11"), Some("$3"), None, None, None],
            },
            // removed rsp instructions
            TestEntry::new_rsp_invalid(0x50740008, None), // beql
            TestEntry::new_rsp_invalid(0x56E1FFF8, None), // bnel
            TestEntry::new_rsp_invalid(0x59C00007, None), // blezl
            TestEntry::new_rsp_invalid(0x5D000001, None), // bgtzl
            TestEntry::new_rsp_invalid(0x60840001, None), // daddi
            TestEntry::new_rsp_invalid(0x64840001, None), // daddiu
            TestEntry::new_rsp_invalid(0x69220007, None), // ldl
            TestEntry::new_rsp_invalid(0x6D240008, None), // ldr
            TestEntry::new_rsp_invalid(0x88EE000D, None), // lwl
            TestEntry::new_rsp_invalid(0x98EE0010, None), // lwr
            TestEntry::new_rsp_invalid(0x9FA30010, None), // lwu
            TestEntry::new_rsp_invalid(0xA8C20000, None), // swl
            TestEntry::new_rsp_invalid(0xB0C20007, None), // sdl
            TestEntry::new_rsp_invalid(0xB4C20000, None), // sdr
            TestEntry::new_rsp_invalid(0xB9C1000E, None), // swr
            TestEntry::new_rsp_invalid(0xC0850000, None), // ll
            TestEntry::new_rsp_invalid(0xD0850000, None), // lld
            TestEntry::new_rsp_invalid(0xDFBF0020, None), // ld
            TestEntry::new_rsp_invalid(0xE0BF0020, None), // sc
            TestEntry::new_rsp_invalid(0xF0BF0020, None), // scd
            TestEntry::new_rsp_invalid(0xFFA00038, None), // sd
            TestEntry::new_rsp_invalid(0xBCD00000, None), // cache
            TestEntry::new_rsp_invalid(0xC4D00010, None), // lwc1
            TestEntry::new_rsp_invalid(0xD4D00010, None), // ldc1
            TestEntry::new_rsp_invalid(0xE4D00010, None), // swc1
            TestEntry::new_rsp_invalid(0xF4D00010, None), // sdc1
            // TestEntry::new_rsp_invalid(0xC8D00010, None), // lwc2
            TestEntry::new_rsp_invalid(0xD8D00010, None), // ldc2
            // TestEntry::new_rsp_invalid(0xE8D00010, None), // swc2
            TestEntry::new_rsp_invalid(0xF8D00010, None), // sdc2
            TestEntry::new_rsp_invalid(0x05020031, None), // bltzl
            TestEntry::new_rsp_invalid(0x04630005, None), // bgezl
            TestEntry::new_rsp_invalid(0x04A80010, None), // tgei
            TestEntry::new_rsp_invalid(0x04A90010, None), // tgeiu
            TestEntry::new_rsp_invalid(0x04AA0010, None), // tlti
            TestEntry::new_rsp_invalid(0x04AB0010, None), // tltiu
            TestEntry::new_rsp_invalid(0x04AC0010, None), // teqi
            TestEntry::new_rsp_invalid(0x04AE0010, None), // tnei
            TestEntry::new_rsp_invalid(0x04B20010, None), // bltzall
            TestEntry::new_rsp_invalid(0x04B30010, None), // bgezall
            TestEntry::new_rsp_invalid(0x00042FF8, None), // dsll
            TestEntry::new_rsp_invalid(0x000637FA, None), // dsrl
            TestEntry::new_rsp_invalid(0x0002137B, None), // dsra
            TestEntry::new_rsp_invalid(0x000437FC, None), // dsll32
            TestEntry::new_rsp_invalid(0x0005283E, None), // dsrl32
            TestEntry::new_rsp_invalid(0x0002103F, None), // dsra32
            TestEntry::new_rsp_invalid(0x01EE1014, None), // dsllv
            TestEntry::new_rsp_invalid(0x01EE1016, None), // dsrlv
            TestEntry::new_rsp_invalid(0x01EE1017, None), // dsrav
            TestEntry::new_rsp_invalid(0x03600011, None), // mthi
            TestEntry::new_rsp_invalid(0x03600013, None), // mtlo
            TestEntry::new_rsp_invalid(0x00004010, None), // mfhi
            TestEntry::new_rsp_invalid(0x00004012, None), // mflo
            TestEntry::new_rsp_invalid(0x01AC001A, None), // div
            TestEntry::new_rsp_invalid(0x0101001B, None), // divu
            TestEntry::new_rsp_invalid(0x01CF001E, None), // ddiv
            TestEntry::new_rsp_invalid(0x01CF001F, None), // ddivu
            TestEntry::new_rsp_invalid(0x0162582C, None), // dadd
            TestEntry::new_rsp_invalid(0x012A582D, None), // daddu
            TestEntry::new_rsp_invalid(0x0162582E, None), // dsub
            TestEntry::new_rsp_invalid(0x0162582F, None), // dsubu
            TestEntry::new_rsp_invalid(0x0000000C, None), // syscall
            TestEntry::new_rsp_invalid(0x0000000F, None), // sync
            TestEntry::new_rsp_invalid(0x00870018, None), // mult
            TestEntry::new_rsp_invalid(0x00E20019, None), // multu
            TestEntry::new_rsp_invalid(0x0087001C, None), // dmult
            TestEntry::new_rsp_invalid(0x00E2001D, None), // dmultu
            TestEntry::new_rsp_invalid(0x00E200B0, None), // tge
            TestEntry::new_rsp_invalid(0x00E200B1, None), // tgeu
            TestEntry::new_rsp_invalid(0x00E200B2, None), // tlt
            TestEntry::new_rsp_invalid(0x00E200B3, None), // tltu
            TestEntry::new_rsp_invalid(0x00E200B4, None), // teq
            TestEntry::new_rsp_invalid(0x00E200B6, None), // tne
            TestEntry::new_rsp_invalid(0x40220800, None), // dmfc0
            TestEntry::new_rsp_invalid(0x40420800, None), // cfc0
            TestEntry::new_rsp_invalid(0x40A20800, None), // dmtc0
            TestEntry::new_rsp_invalid(0x40C20800, None), // ctc0
            // Invalid instructions
            TestEntry::new_rsp_invalid(0x44444444, None),
            TestEntry::new_rsp_invalid(0x77777777, None),
            TestEntry::new_rsp_invalid(0xEEEEEEEE, None),
        ];

        assert_eq!(check_test_entries(ENTRIES), 0);
    }
}
