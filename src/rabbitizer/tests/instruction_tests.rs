/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(test)]
pub(crate) mod tests {
    use std::string::ToString;

    use crate::{operand, DisplayFlags, Instruction, InstructionFlags, IsaVersion, Opcode};

    pub(crate) struct TestEntry {
        pub instr: Instruction,
        pub imm_override: Option<&'static str>,
        pub display_flags: DisplayFlags,

        pub valid: bool,

        pub expected: &'static str,
        pub expected_opcode: Opcode,
        pub opcode_str: &'static str,
        pub operands_str: [Option<&'static str>; operand::OPERAND_COUNT_MAX],
    }

    impl TestEntry {
        #[allow(dead_code)]
        pub const fn new_none_invalid(word: u32, flags: InstructionFlags) -> Self {
            Self {
                instr: Instruction::new_no_extension(word, 0x80000000, flags, IsaVersion::MIPS_III),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: false,
                expected: "INVALID",
                expected_opcode: Opcode::ALL_INVALID,
                opcode_str: "INVALID",
                operands_str: [None, None, None, None, None],
            }
        }

        #[allow(dead_code)]
        pub const fn new_rsp_invalid(word: u32, flags: InstructionFlags) -> Self {
            Self {
                instr: Instruction::new_rsp(word, 0xA4000000, flags),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: false,
                expected: "INVALID",
                expected_opcode: Opcode::ALL_INVALID,
                opcode_str: "INVALID",
                operands_str: [None, None, None, None, None],
            }
        }

        pub const fn new_r4000allegrex(
            word: u32,
            expected: &'static str,
            expected_opcode: Opcode,
            opcode_str: &'static str,
            operands_str: [Option<&'static str>; operand::OPERAND_COUNT_MAX],
        ) -> Self {
            Self {
                instr: Instruction::new_r4000allegrex(
                    word,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected,
                expected_opcode,
                opcode_str,
                operands_str,
            }
        }

        pub fn compare_source_info(&self, other: &Self) -> bool {
            self.instr.word() == other.instr.word()
                && self.instr.flags() == other.instr.flags()
                && self.imm_override == other.imm_override
                && self.display_flags == other.display_flags
                && self.valid == other.valid
        }

        pub fn check_validity(&self) -> u32 {
            let mut errors = 0;

            if self.instr.is_valid() != self.valid {
                println!(
                    "'{}' ({:08X}) has incorrect validity. Expected '{}', got '{}'",
                    self.opcode_str,
                    self.instr.word(),
                    self.valid,
                    self.instr.is_valid()
                );
                errors += 1;
            }
            if self.instr.opcode() != self.expected_opcode {
                println!(
                    "'{}' ({:08X}) has incorrect decoded opcode. Expected '{:?}', got '{:?}'",
                    self.opcode_str,
                    self.instr.word(),
                    self.expected_opcode,
                    self.instr.opcode()
                );
                errors += 1;
            }
            if self.instr.opcode().name() != self.opcode_str {
                println!(
                    "'{}' ({:08X}) has incorrect opcode name. Expected '{}', got '{}'",
                    self.opcode_str,
                    self.instr.word(),
                    self.opcode_str,
                    self.instr.opcode().name()
                );
                errors += 1;
            }

            errors
        }

        pub fn check_disassembly(&self) -> u32 {
            let mut errors = 0;

            let disasm = self
                .instr
                .display(self.imm_override, &self.display_flags)
                .to_string();
            // println!("    {}", disasm);
            if disasm != self.expected {
                println!(
                    "'{}' ({:08X}) did not match the expected string.",
                    self.opcode_str,
                    self.instr.word(),
                );
                println!("    Expected: '{}'", self.expected,);
                println!("    Got:      '{}'", disasm,);
                errors += 1;
            }

            {
                let mut j = 0;
                for (i, operand) in self.instr.operands_iter().enumerate() {
                    let operand_str = operand
                        .display(&self.instr, self.imm_override, &self.display_flags)
                        .to_string();
                    let maybe_expected_str = self.operands_str[i];

                    if let Some(expected_str) = maybe_expected_str {
                        if operand_str != expected_str {
                            println!(
                                "'{}' ({:08X}) has incorrect disassembled operand. Expected '{}', got '{}'",
                                self.opcode_str,
                                self.instr.word(),
                                expected_str,
                                operand_str
                            );
                            errors += 1;
                        }
                    } else {
                        println!(
                            "'{}' ({:08X}) has an unexpected operand at index {}. Got: '{}'",
                            self.opcode_str,
                            self.instr.word(),
                            i,
                            operand_str,
                        );
                        errors += 1;
                    }
                    j = i;
                }

                if !self.operands_str[j + 1..].iter().all(|x| x.is_none()) {
                    println!(
                        "'{}' ({:08X}) has unhandled expected operands. Values: '{:?}'",
                        self.opcode_str,
                        self.instr.word(),
                        &self.operands_str[j..]
                    );
                    errors += 1;
                }
            }

            errors
        }
    }

    pub(crate) fn entries_sanity_check(entries: &[TestEntry]) {
        for (i, x) in entries.iter().enumerate() {
            for y in entries[i + 1..].iter() {
                assert!(
                    !x.compare_source_info(y),
                    "Duplicated entry. Word: '0x{:08X}'. imm_override: '{:?}'",
                    x.instr.word(),
                    x.imm_override
                );
            }
        }
    }

    pub(crate) fn check_test_entries(entries: &[TestEntry], thingy: bool) -> u32 {
        let mut errors = 0;

        entries_sanity_check(entries);

        for entry in entries {
            errors += entry.check_validity();
            if thingy {
                errors += entry.check_disassembly();
            }
        }

        errors
    }

    #[test]
    fn check_none_instructions() {
        const ENTRIES: &[TestEntry] = &[
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x3C088001,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lui         $t0, 0x8001",
                expected_opcode: Opcode::core_lui,
                opcode_str: "lui",
                operands_str: [Some("$t0"), Some("0x8001"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x25080E60,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "addiu       $t0, $t0, 0xE60",
                expected_opcode: Opcode::core_addiu,
                opcode_str: "addiu",
                operands_str: [Some("$t0"), Some("$t0"), Some("0xE60"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x3C090002,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lui         $t1, 0x2",
                expected_opcode: Opcode::core_lui,
                opcode_str: "lui",
                operands_str: [Some("$t1"), Some("0x2"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x25298DE0,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "addiu       $t1, $t1, -0x7220",
                expected_opcode: Opcode::core_addiu,
                opcode_str: "addiu",
                operands_str: [Some("$t1"), Some("$t1"), Some("-0x7220"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xAD000000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sw          $zero, 0x0($t0)",
                expected_opcode: Opcode::core_sw,
                opcode_str: "sw",
                operands_str: [Some("$zero"), Some("0x0($t0)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xAD000004,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sw          $zero, 0x4($t0)",
                expected_opcode: Opcode::core_sw,
                opcode_str: "sw",
                operands_str: [Some("$zero"), Some("0x4($t0)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x21080008,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "addi        $t0, $t0, 0x8",
                expected_opcode: Opcode::core_addi,
                opcode_str: "addi",
                operands_str: [Some("$t0"), Some("$t0"), Some("0x8"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x2129FFF8,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "addi        $t1, $t1, -0x8",
                expected_opcode: Opcode::core_addi,
                opcode_str: "addi",
                operands_str: [Some("$t1"), Some("$t1"), Some("-0x8"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x1520FFFB,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "bnez        $t1, . + 4 + (-0x5 << 2)",
                expected_opcode: Opcode::core_bnez,
                opcode_str: "bnez",
                operands_str: [Some("$t1"), Some(". + 4 + (-0x5 << 2)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00000000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "nop",
                expected_opcode: Opcode::core_nop,
                opcode_str: "nop",
                operands_str: [None, None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x3C0A8000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lui         $t2, 0x8000",
                expected_opcode: Opcode::core_lui,
                opcode_str: "lui",
                operands_str: [Some("$t2"), Some("0x8000"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x254A0494,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "addiu       $t2, $t2, 0x494",
                expected_opcode: Opcode::core_addiu,
                opcode_str: "addiu",
                operands_str: [Some("$t2"), Some("$t2"), Some("0x494"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x3C1D8002,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lui         $sp, 0x8002",
                expected_opcode: Opcode::core_lui,
                opcode_str: "lui",
                operands_str: [Some("$sp"), Some("0x8002"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x01400008,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "jr          $t2",
                expected_opcode: Opcode::core_jr,
                opcode_str: "jr",
                operands_str: [Some("$t2"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x27BDF8C0,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "addiu       $sp, $sp, -0x740",
                expected_opcode: Opcode::core_addiu,
                opcode_str: "addiu",
                operands_str: [Some("$sp"), Some("$sp"), Some("-0x740"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x3C018001,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lui         $at, 0x8001",
                expected_opcode: Opcode::core_lui,
                opcode_str: "lui",
                operands_str: [Some("$at"), Some("0x8001"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x03E00008,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "jr          $ra",
                expected_opcode: Opcode::core_jr,
                opcode_str: "jr",
                operands_str: [Some("$ra"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xAC24E190,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sw          $a0, -0x1E70($at)",
                expected_opcode: Opcode::core_sw,
                opcode_str: "sw",
                operands_str: [Some("$a0"), Some("-0x1E70($at)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x3C018001,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: Some("%hi(D_8000E190)"),
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lui         $at, %hi(D_8000E190)",
                expected_opcode: Opcode::core_lui,
                opcode_str: "lui",
                operands_str: [Some("$at"), Some("%hi(D_8000E190)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xAC24E190,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: Some("%lo(D_8000E190)"),
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sw          $a0, %lo(D_8000E190)($at)",
                expected_opcode: Opcode::core_sw,
                opcode_str: "sw",
                operands_str: [Some("$a0"), Some("%lo(D_8000E190)($at)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0C001F24,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "jal         func_80007C90",
                expected_opcode: Opcode::core_jal,
                opcode_str: "jal",
                operands_str: [Some("func_80007C90"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0C001F24,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: Some("some_func"),
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "jal         some_func",
                expected_opcode: Opcode::core_jal,
                opcode_str: "jal",
                operands_str: [Some("some_func"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x8F99805C,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lw          $t9, -0x7FA4($gp)",
                expected_opcode: Opcode::core_lw,
                opcode_str: "lw",
                operands_str: [Some("$t9"), Some("-0x7FA4($gp)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x8F99805C,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: Some("%call16(strcmp)"),
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lw          $t9, %call16(strcmp)($gp)",
                expected_opcode: Opcode::core_lw,
                opcode_str: "lw",
                operands_str: [Some("$t9"), Some("%call16(strcmp)($gp)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x8F858028,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lw          $a1, -0x7FD8($gp)",
                expected_opcode: Opcode::core_lw,
                opcode_str: "lw",
                operands_str: [Some("$a1"), Some("-0x7FD8($gp)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x8F858028,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: Some("%got(STR_10007C90)"),
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lw          $a1, %got(STR_10007C90)($gp)",
                expected_opcode: Opcode::core_lw,
                opcode_str: "lw",
                operands_str: [
                    Some("$a1"),
                    Some("%got(STR_10007C90)($gp)"),
                    None,
                    None,
                    None,
                ],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00435022,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sub         $t2, $v0, $v1",
                expected_opcode: Opcode::core_sub,
                opcode_str: "sub",
                operands_str: [Some("$t2"), Some("$v0"), Some("$v1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00025022,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "neg         $t2, $v0",
                expected_opcode: Opcode::core_neg,
                opcode_str: "neg",
                operands_str: [Some("$t2"), Some("$v0"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00E41823,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "subu        $v1, $a3, $a0",
                expected_opcode: Opcode::core_subu,
                opcode_str: "subu",
                operands_str: [Some("$v1"), Some("$a3"), Some("$a0"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00041823,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "negu        $v1, $a0",
                expected_opcode: Opcode::core_negu,
                opcode_str: "negu",
                operands_str: [Some("$v1"), Some("$a0"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x42000010,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "rfe",
                expected_opcode: Opcode::core_rfe,
                opcode_str: "rfe",
                operands_str: [None, None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0260F809,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "jalr        $s3",
                expected_opcode: Opcode::core_jalr,
                opcode_str: "jalr",
                operands_str: [Some("$s3"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0260F809,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_expand_jalr(true),
                valid: true,
                expected: "jalr        $ra, $s3",
                expected_opcode: Opcode::core_jalr,
                opcode_str: "jalr",
                operands_str: [Some("$ra, $s3"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x02602009,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "jalr        $a0, $s3",
                expected_opcode: Opcode::core_jalr,
                opcode_str: "jalr",
                operands_str: [Some("$a0, $s3"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00042100,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sll         $a0, $a0, 4",
                expected_opcode: Opcode::core_sll,
                opcode_str: "sll",
                operands_str: [Some("$a0"), Some("$a0"), Some("4"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00021882,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "srl         $v1, $v0, 2",
                expected_opcode: Opcode::core_srl,
                opcode_str: "srl",
                operands_str: [Some("$v1"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00017443,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sra         $t6, $at, 17",
                expected_opcode: Opcode::core_sra,
                opcode_str: "sra",
                operands_str: [Some("$t6"), Some("$at"), Some("17"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00042FF8,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dsll        $a1, $a0, 31",
                expected_opcode: Opcode::core_dsll,
                opcode_str: "dsll",
                operands_str: [Some("$a1"), Some("$a0"), Some("31"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x000637FA,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dsrl        $a2, $a2, 31",
                expected_opcode: Opcode::core_dsrl,
                opcode_str: "dsrl",
                operands_str: [Some("$a2"), Some("$a2"), Some("31"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0002137B,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dsra        $v0, $v0, 13",
                expected_opcode: Opcode::core_dsra,
                opcode_str: "dsra",
                operands_str: [Some("$v0"), Some("$v0"), Some("13"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x000437FC,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dsll32      $a2, $a0, 31",
                expected_opcode: Opcode::core_dsll32,
                opcode_str: "dsll32",
                operands_str: [Some("$a2"), Some("$a0"), Some("31"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0005283E,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dsrl32      $a1, $a1, 0",
                expected_opcode: Opcode::core_dsrl32,
                opcode_str: "dsrl32",
                operands_str: [Some("$a1"), Some("$a1"), Some("0"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0002103F,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dsra32      $v0, $v0, 0",
                expected_opcode: Opcode::core_dsra32,
                opcode_str: "dsra32",
                operands_str: [Some("$v0"), Some("$v0"), Some("0"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x40086800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "mfc0        $t0, $13",
                expected_opcode: Opcode::core_mfc0,
                opcode_str: "mfc0",
                operands_str: [Some("$t0"), Some("$13"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x40286800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dmfc0       $t0, $13",
                expected_opcode: Opcode::core_dmfc0,
                opcode_str: "dmfc0",
                operands_str: [Some("$t0"), Some("$13"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x40486800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "cfc0        $t0, $13",
                expected_opcode: Opcode::core_cfc0,
                opcode_str: "cfc0",
                operands_str: [Some("$t0"), Some("$13"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x40886800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "mtc0        $t0, $13",
                expected_opcode: Opcode::core_mtc0,
                opcode_str: "mtc0",
                operands_str: [Some("$t0"), Some("$13"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x40A86800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "dmtc0       $t0, $13",
                expected_opcode: Opcode::core_dmtc0,
                opcode_str: "dmtc0",
                operands_str: [Some("$t0"), Some("$13"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x40C86800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "ctc0        $t0, $13",
                expected_opcode: Opcode::core_ctc0,
                opcode_str: "ctc0",
                operands_str: [Some("$t0"), Some("$13"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x46168200,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_named_fpr(false),
                valid: true,
                expected: "add.s       $f8, $f16, $f22",
                expected_opcode: Opcode::core_add_s,
                opcode_str: "add.s",
                operands_str: [Some("$f8"), Some("$f16"), Some("$f22"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x46168200,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_named_fpr(true),
                valid: true,
                expected: "add.s       $ft2, $ft4, $fs1",
                expected_opcode: Opcode::core_add_s,
                opcode_str: "add.s",
                operands_str: [Some("$ft2"), Some("$ft4"), Some("$fs1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x44C2F800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "ctc1        $v0, $31",
                expected_opcode: Opcode::core_ctc1,
                opcode_str: "ctc1",
                operands_str: [Some("$v0"), Some("$31"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xBD150000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "cache       0x15, 0x0($t0)",
                expected_opcode: Opcode::core_cache,
                opcode_str: "cache",
                operands_str: [Some("0x15"), Some("0x0($t0)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xCD150018,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: false,
                expected: ".word       0xCD150018                   /* INVALID / 01150018 <OpcodeCategory: CORE_NORMAL> */",
                expected_opcode: Opcode::ALL_INVALID,
                opcode_str: "INVALID",
                operands_str: [None, None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xCD150008,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_IV,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "pref        0x15, 0x8($t0)",
                expected_opcode:   Opcode::core_pref,
                opcode_str: "pref",
                operands_str: [Some("0x15"), Some("0x8($t0)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0001008D,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "break       1, 2",
                expected_opcode: Opcode::core_break,
                opcode_str: "break",
                operands_str: [Some("1, 2"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0001008D,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_sn64_break_fix(true),
                valid: true,
                expected: ".word       0x0001008D                   /* break       1, 2 / 00000000 <OpcodeCategory: CORE_SPECIAL> */",
                expected_opcode: Opcode::core_break,
                opcode_str: "break",
                operands_str: [Some("1, 2"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0007000D,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "break       7",
                expected_opcode: Opcode::core_break,
                opcode_str: "break",
                operands_str: [Some("7"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0007000D,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_sn64_break_fix(true),
                valid: true,
                expected: ".word       0x0007000D                   /* break       7 / 00000000 <OpcodeCategory: CORE_SPECIAL> */",
                expected_opcode: Opcode::core_break,
                opcode_str: "break",
                operands_str: [Some("7"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0000000C,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "syscall     0",
                expected_opcode: Opcode::core_syscall,
                opcode_str: "syscall",
                operands_str: [Some("0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00E200B0,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "tge         $a3, $v0, 2",
                expected_opcode: Opcode::core_tge,
                opcode_str: "tge",
                operands_str: [Some("$a3"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00E200B1,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "tgeu        $a3, $v0, 2",
                expected_opcode: Opcode::core_tgeu,
                opcode_str: "tgeu",
                operands_str: [Some("$a3"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00E200B2,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "tlt         $a3, $v0, 2",
                expected_opcode: Opcode::core_tlt,
                opcode_str: "tlt",
                operands_str: [Some("$a3"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00E200B3,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "tltu        $a3, $v0, 2",
                expected_opcode: Opcode::core_tltu,
                opcode_str: "tltu",
                operands_str: [Some("$a3"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00E200B4,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "teq         $a3, $v0, 2",
                expected_opcode: Opcode::core_teq,
                opcode_str: "teq",
                operands_str: [Some("$a3"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x00E200B6,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "tne         $a3, $v0, 2",
                expected_opcode: Opcode::core_tne,
                opcode_str: "tne",
                operands_str: [Some("$a3"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xC8621800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "lwc2        $2, 0x1800($v1)",
                expected_opcode: Opcode::core_lwc2,
                opcode_str: "lwc2",
                operands_str: [Some("$2"), Some("0x1800($v1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xD8621800,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "ldc2        $2, 0x1800($v1)",
                expected_opcode: Opcode::core_ldc2,
                opcode_str: "ldc2",
                operands_str: [Some("$2"), Some("0x1800($v1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xE8810878,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "swc2        $1, 0x878($a0)",
                expected_opcode: Opcode::core_swc2,
                opcode_str: "swc2",
                operands_str: [Some("$1"), Some("0x878($a0)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xF8810878,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "sdc2        $1, 0x878($a0)",
                expected_opcode: Opcode::core_sdc2,
                opcode_str: "sdc2",
                operands_str: [Some("$1"), Some("0x878($a0)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x4802E000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "mfc2        $v0, $28",
                expected_opcode: Opcode::core_mfc2,
                opcode_str: "mfc2",
                operands_str: [Some("$v0"), Some("$28"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x4882E000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "mtc2        $v0, $28",
                expected_opcode: Opcode::core_mtc2,
                opcode_str: "mtc2",
                operands_str: [Some("$v0"), Some("$28"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x4842E000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "cfc2        $v0, $28",
                expected_opcode: Opcode::core_cfc2,
                opcode_str: "cfc2",
                operands_str: [Some("$v0"), Some("$28"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x48C2E000,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "ctc2        $v0, $28",
                expected_opcode: Opcode::core_ctc2,
                opcode_str: "ctc2",
                operands_str: [Some("$v0"), Some("$28"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001A,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(true),
                valid: true,
                expected: "div         $zero, $a0, $a1",
                expected_opcode: Opcode::core_div,
                opcode_str: "div",
                operands_str: [Some("$zero, $a0"), Some("$a1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001A,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(false),
                valid: true,
                expected: "div         $a0, $a1",
                expected_opcode: Opcode::core_div,
                opcode_str: "div",
                operands_str: [Some("$a0"), Some("$a1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001B,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(true),
                valid: true,
                expected: "divu        $zero, $a0, $a1",
                expected_opcode: Opcode::core_divu,
                opcode_str: "divu",
                operands_str: [Some("$zero, $a0"), Some("$a1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001B,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(false),
                valid: true,
                expected: "divu        $a0, $a1",
                expected_opcode: Opcode::core_divu,
                opcode_str: "divu",
                operands_str: [Some("$a0"), Some("$a1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001E,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(true),
                valid: true,
                expected: "ddiv        $zero, $a0, $a1",
                expected_opcode: Opcode::core_ddiv,
                opcode_str: "ddiv",
                operands_str: [Some("$zero, $a0"), Some("$a1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001E,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(false),
                valid: true,
                expected: "ddiv        $a0, $a1",
                expected_opcode: Opcode::core_ddiv,
                opcode_str: "ddiv",
                operands_str: [Some("$a0"), Some("$a1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001F,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(true),
                valid: true,
                expected: "ddivu       $zero, $a0, $a1",
                expected_opcode: Opcode::core_ddivu,
                opcode_str: "ddivu",
                operands_str: [Some("$zero, $a0"), Some("$a1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x0085001F,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default().with_gnu_div(false),
                valid: true,
                expected: "ddivu       $a0, $a1",
                expected_opcode: Opcode::core_ddivu,
                opcode_str: "ddivu",
                operands_str: [Some("$a0"), Some("$a1"), None, None, None],
            },


            // Invalid instructions

            TestEntry {
                instr: Instruction::new_no_extension(
                    0x44444444,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: false,
                expected: ".word       0x44444444                   /* cfc1        $a0, $8 / 00000444 <OpcodeCategory: CORE_COP1> */",
                expected_opcode: Opcode::core_cfc1,
                opcode_str: "cfc1",
                operands_str: [Some("$a0"), Some("$8"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0x77777777,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: false,
                expected: ".word       0x77777777                   /* INVALID / 03777777 <OpcodeCategory: CORE_NORMAL> */",
                expected_opcode: Opcode::ALL_INVALID,
                opcode_str: "INVALID",
                operands_str: [None, None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_no_extension(
                    0xEEEEEEEE,
                    0x80000000,
                    InstructionFlags::default(),
                    IsaVersion::MIPS_III,
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: false,
                expected: ".word       0xEEEEEEEE                   /* INVALID / 02EEEEEE <OpcodeCategory: CORE_NORMAL> */",
                expected_opcode: Opcode::ALL_INVALID,
                opcode_str: "INVALID",
                operands_str: [None, None, None, None, None],
            },
        ];

        assert_eq!(check_test_entries(ENTRIES, true), 0);
    }

    #[test]
    fn check_rsp_instructions() {
        const ENTRIES: &[TestEntry] = &[
            TestEntry {
                instr: Instruction::new_rsp(0x09000419, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "j           func_A4001064",
                expected_opcode: Opcode::core_j,
                opcode_str: "j",
                operands_str: [Some("func_A4001064"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x21490000, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "addi        $9, $10, 0x0",
                expected_opcode: Opcode::core_addi,
                opcode_str: "addi",
                operands_str: [Some("$9"), Some("$10"), Some("0x0"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x8C060578, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lw          $6, 0x578($zero)",
                expected_opcode: Opcode::core_lw,
                opcode_str: "lw",
                operands_str: [Some("$6"), Some("0x578($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x400B2800, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "mfc0        $11, SP_DMA_FULL",
                expected_opcode: Opcode::rsp_mfc0,
                opcode_str: "mfc0",
                operands_str: [Some("$11"), Some("SP_DMA_FULL"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x304203FF, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "andi        $2, $2, 0x3FF",
                expected_opcode: Opcode::core_andi,
                opcode_str: "andi",
                operands_str: [Some("$2"), Some("$2"), Some("0x3FF"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x10400003, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "beqz        $2, .L800B38E8",
                expected_opcode: Opcode::core_beqz,
                opcode_str: "beqz",
                operands_str: [Some("$2"), Some(".L800B38E8"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00000000, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "nop",
                expected_opcode: Opcode::core_nop,
                opcode_str: "nop",
                operands_str: [None, None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1C600033, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "bgtz        $3, .L800B3A74",
                expected_opcode: Opcode::core_bgtz,
                opcode_str: "bgtz",
                operands_str: [Some("$3"), Some(".L800B3A74"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x0D00077A, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "jal         func_84001DE8",
                expected_opcode: Opcode::core_jal,
                opcode_str: "jal",
                operands_str: [Some("func_84001DE8"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xAEEB000C, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sw          $11, 0xC($23)",
                expected_opcode: Opcode::core_sw,
                opcode_str: "sw",
                operands_str: [Some("$11"), Some("0xC($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1560FB8D, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "bnez        $11, .L800B2288",
                expected_opcode: Opcode::core_bnez,
                opcode_str: "bnez",
                operands_str: [Some("$11"), Some(".L800B2288"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x40921800, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "mtc0        $18, SP_WR_LEN",
                expected_opcode: Opcode::rsp_mtc0,
                opcode_str: "mtc0",
                operands_str: [Some("$18"), Some("SP_WR_LEN"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00026A82, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "srl         $13, $2, 10",
                expected_opcode: Opcode::core_srl,
                opcode_str: "srl",
                operands_str: [Some("$13"), Some("$2"), Some("10"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x004F1020, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "add         $2, $2, $15",
                expected_opcode: Opcode::core_add,
                opcode_str: "add",
                operands_str: [Some("$2"), Some("$2"), Some("$15"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x84040572, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lh          $4, 0x572($zero)",
                expected_opcode: Opcode::core_lh,
                opcode_str: "lh",
                operands_str: [Some("$4"), Some("0x572($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x03E00008, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "jr          $ra",
                expected_opcode: Opcode::core_jr,
                opcode_str: "jr",
                operands_str: [Some("$ra"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x0000000D, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "break       0",
                expected_opcode: Opcode::core_break,
                opcode_str: "break",
                operands_str: [Some("0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x19C0FA06, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "blez        $14, .L800B2288",
                expected_opcode: Opcode::core_blez,
                opcode_str: "blez",
                operands_str: [Some("$14"), Some(".L800B2288"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x09000A19, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "j           func_84002864",
                expected_opcode: Opcode::core_j,
                opcode_str: "j",
                operands_str: [Some("func_84002864"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x37120000, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "ori         $18, $24, 0x0",
                expected_opcode: Opcode::core_ori,
                opcode_str: "ori",
                operands_str: [Some("$18"), Some("$24"), Some("0x0"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1000FE72, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "b           .L800B1A78",
                expected_opcode: Opcode::core_b,
                opcode_str: "b",
                operands_str: [Some(".L800B1A78"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A0318EC, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vxor        $v3, $v3, $v3",
                expected_opcode: Opcode::rsp_vxor,
                opcode_str: "vxor",
                operands_str: [Some("$v3"), Some("$v3"), Some("$v3"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAF11B0B, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sdv         $v17[6], 0x58($23)",
                expected_opcode: Opcode::rsp_sdv,
                opcode_str: "sdv",
                operands_str: [Some("$v17[6]"), Some("0x58($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x940C0572, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lhu         $12, 0x572($zero)",
                expected_opcode: Opcode::core_lhu,
                opcode_str: "lhu",
                operands_str: [Some("$12"), Some("0x572($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00095880, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sll         $11, $9, 2",
                expected_opcode: Opcode::core_sll,
                opcode_str: "sll",
                operands_str: [Some("$11"), Some("$9"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xA403057C, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sh          $3, 0x57C($zero)",
                expected_opcode: Opcode::core_sh,
                opcode_str: "sh",
                operands_str: [Some("$3"), Some("0x57C($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xCA832000, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lqv         $v3[0], 0x0($20)",
                expected_opcode: Opcode::rsp_lqv,
                opcode_str: "lqv",
                operands_str: [Some("$v3[0]"), Some("0x0($20)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAE40C0B, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "ssv         $v4[8], 0x16($23)",
                expected_opcode: Opcode::rsp_ssv,
                opcode_str: "ssv",
                operands_str: [Some("$v4[8]"), Some("0x16($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xCBC41807, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "ldv         $v4[0], 0x38($30)",
                expected_opcode: Opcode::rsp_ldv,
                opcode_str: "ldv",
                operands_str: [Some("$v4[0]"), Some("0x38($30)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x3C07F510, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lui         $7, (0xF5100000 >> 16)",
                expected_opcode: Opcode::core_lui,
                opcode_str: "lui",
                operands_str: [Some("$7"), Some("(0xF5100000 >> 16)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00611824, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "and         $3, $3, $1",
                expected_opcode: Opcode::core_and,
                opcode_str: "and",
                operands_str: [Some("$3"), Some("$3"), Some("$1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEBC62000, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sqv         $v6[0], 0x0($30)",
                expected_opcode: Opcode::rsp_sqv,
                opcode_str: "sqv",
                operands_str: [Some("$v6[0]"), Some("0x0($30)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x1193FFFE, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "beq         $12, $19, .L800B304C",
                expected_opcode: Opcode::core_beq,
                opcode_str: "beq",
                operands_str: [Some("$12"), Some("$19"), Some(".L800B304C"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x900B0539, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lbu         $11, 0x539($zero)",
                expected_opcode: Opcode::core_lbu,
                opcode_str: "lbu",
                operands_str: [Some("$11"), Some("0x539($zero)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0C58A8, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vand        $v2, $v11, $v12[0]",
                expected_opcode: Opcode::rsp_vand,
                opcode_str: "vand",
                operands_str: [Some("$v2"), Some("$v11"), Some("$v12[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x008C5822, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sub         $11, $4, $12",
                expected_opcode: Opcode::core_sub,
                opcode_str: "sub",
                operands_str: [Some("$11"), Some("$4"), Some("$12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00432006, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "srlv        $4, $3, $2",
                expected_opcode: Opcode::core_srlv,
                opcode_str: "srlv",
                operands_str: [Some("$4"), Some("$3"), Some("$2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x488ED800, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "mtc2        $14, $v27[0]",
                expected_opcode: Opcode::rsp_mtc2,
                opcode_str: "mtc2",
                operands_str: [Some("$14"), Some("$v27[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC9B12802, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lrv         $v17[0], 0x20($13)",
                expected_opcode: Opcode::rsp_lrv,
                opcode_str: "lrv",
                operands_str: [Some("$v17[0]"), Some("0x20($13)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1B21C6, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmudn       $v7, $v4, $v27[0]",
                expected_opcode: Opcode::rsp_vmudn,
                opcode_str: "vmudn",
                operands_str: [Some("$v7"), Some("$v4"), Some("$v27[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7F488E, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmadn       $v2, $v9, $v31[3]",
                expected_opcode: Opcode::rsp_vmadn,
                opcode_str: "vmadn",
                operands_str: [Some("$v2"), Some("$v9"), Some("$v31[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B2A3A05, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmudm       $v8, $v7, $v10[1]",
                expected_opcode: Opcode::rsp_vmudm,
                opcode_str: "vmudm",
                operands_str: [Some("$v8"), Some("$v7"), Some("$v10[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B3D1EC7, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmudh       $v27, $v3, $v29[1]",
                expected_opcode: Opcode::rsp_vmudh,
                opcode_str: "vmudh",
                operands_str: [Some("$v27"), Some("$v3"), Some("$v29[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7DFA0F, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmadh       $v8, $v31, $v29[3]",
                expected_opcode: Opcode::rsp_vmadh,
                opcode_str: "vmadh",
                operands_str: [Some("$v8"), Some("$v31"), Some("$v29[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0A529D, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vsar        $v10, $v10, $v10[0]",
                expected_opcode: Opcode::rsp_vsar,
                opcode_str: "vsar",
                operands_str: [Some("$v10"), Some("$v10"), Some("$v10[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAFD1204, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "slv         $v29[4], 0x10($23)",
                expected_opcode: Opcode::rsp_slv,
                opcode_str: "slv",
                operands_str: [Some("$v29[4]"), Some("0x10($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC827080F, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lsv         $v7[0], 0x1E($1)",
                expected_opcode: Opcode::rsp_lsv,
                opcode_str: "lsv",
                operands_str: [Some("$v7[0]"), Some("0x1E($1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4BAA4351, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vsub        $v13, $v8, $v10[5]",
                expected_opcode: Opcode::rsp_vsub,
                opcode_str: "vsub",
                operands_str: [Some("$v13"), Some("$v8"), Some("$v10[5]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F8ECD, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmadm       $v27, $v17, $v31[0]",
                expected_opcode: Opcode::rsp_vmadm,
                opcode_str: "vmadm",
                operands_str: [Some("$v27"), Some("$v17"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B3F7384, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmudl       $v14, $v14, $v31[1]",
                expected_opcode: Opcode::rsp_vmudl,
                opcode_str: "vmudl",
                operands_str: [Some("$v14"), Some("$v14"), Some("$v31[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B9F2940, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmulf       $v5, $v5, $v31[4]",
                expected_opcode: Opcode::rsp_vmulf,
                opcode_str: "vmulf",
                operands_str: [Some("$v5"), Some("$v5"), Some("$v31[4]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4BAA4390, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vadd        $v14, $v8, $v10[5]",
                expected_opcode: Opcode::rsp_vadd,
                opcode_str: "vadd",
                operands_str: [Some("$v14"), Some("$v8"), Some("$v10[5]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4813D900, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "mfc2        $19, $v27[2]",
                expected_opcode: Opcode::rsp_mfc2,
                opcode_str: "mfc2",
                operands_str: [Some("$19"), Some("$v27[2]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A1174D5, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vsubc       $v19, $v14, $v17",
                expected_opcode: Opcode::rsp_vsubc,
                opcode_str: "vsubc",
                operands_str: [Some("$v19"), Some("$v14"), Some("$v17"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7D6EE3, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vge         $v27, $v13, $v29[3]",
                expected_opcode: Opcode::rsp_vge,
                opcode_str: "vge",
                operands_str: [Some("$v27"), Some("$v13"), Some("$v29[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4AF4E0E4, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vcl         $v3, $v28, $v20[3h]",
                expected_opcode: Opcode::rsp_vcl,
                opcode_str: "vcl",
                operands_str: [Some("$v3"), Some("$v28"), Some("$v20[3h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4AC550C8, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmacf       $v3, $v10, $v5[2h]",
                expected_opcode: Opcode::rsp_vmacf,
                opcode_str: "vmacf",
                operands_str: [Some("$v3"), Some("$v10"), Some("$v5[2h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x15610003, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "bne         $11, $1, .L800B2F4C",
                expected_opcode: Opcode::core_bne,
                opcode_str: "bne",
                operands_str: [Some("$11"), Some("$1"), Some(".L800B2F4C"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x05C1000F, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "bgez        $14, .L800B3A2C",
                expected_opcode: Opcode::core_bgez,
                opcode_str: "bgez",
                operands_str: [Some("$14"), Some(".L800B3A2C"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xEAFD0688, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sbv         $v29[13], 0x8($23)",
                expected_opcode: Opcode::rsp_sbv,
                opcode_str: "sbv",
                operands_str: [Some("$v29[13]"), Some("0x8($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x01675825, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "or          $11, $11, $7",
                expected_opcode: Opcode::core_or,
                opcode_str: "or",
                operands_str: [Some("$11"), Some("$11"), Some("$7"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x03241804, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sllv        $3, $4, $25",
                expected_opcode: Opcode::core_sllv,
                opcode_str: "sllv",
                operands_str: [Some("$3"), Some("$4"), Some("$25"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00601827, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "not         $3, $3",
                expected_opcode: Opcode::core_not,
                opcode_str: "not",
                operands_str: [Some("$3"), Some("$3"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x83790500, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lb          $25, 0x500($27)",
                expected_opcode: Opcode::core_lb,
                opcode_str: "lb",
                operands_str: [Some("$25"), Some("0x500($27)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x05600002, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "bltz        $11, .L800B3A2C",
                expected_opcode: Opcode::core_bltz,
                opcode_str: "bltz",
                operands_str: [Some("$11"), Some(".L800B3A2C"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xA2EB0009, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sb          $11, 0x9($23)",
                expected_opcode: Opcode::core_sb,
                opcode_str: "sb",
                operands_str: [Some("$11"), Some("0x9($23)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x000B5A83, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sra         $11, $11, 10",
                expected_opcode: Opcode::core_sra,
                opcode_str: "sra",
                operands_str: [Some("$11"), Some("$11"), Some("10"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC98C1000, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "llv         $v12[0], 0x0($12)",
                expected_opcode: Opcode::rsp_llv,
                opcode_str: "llv",
                operands_str: [Some("$v12[0]"), Some("0x0($12)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4AF5E8E5, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vch         $v3, $v29, $v21[3h]",
                expected_opcode: Opcode::rsp_vch,
                opcode_str: "vch",
                operands_str: [Some("$v3"), Some("$v29"), Some("$v21[3h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x484B0800, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "cfc2        $t3, $1",
                expected_opcode: Opcode::rsp_cfc2,
                opcode_str: "cfc2",
                operands_str: [Some("$t3"), Some("$1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A086A27, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmrg        $v8, $v13, $v8",
                expected_opcode: Opcode::rsp_vmrg,
                opcode_str: "vmrg",
                operands_str: [Some("$v8"), Some("$v13"), Some("$v8"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7D7EE0, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vlt         $v27, $v15, $v29[3]",
                expected_opcode: Opcode::rsp_vlt,
                opcode_str: "vlt",
                operands_str: [Some("$v27"), Some("$v15"), Some("$v29[3]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x26F70018, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "addiu       $23, $23, 0x18",
                expected_opcode: Opcode::core_addiu,
                opcode_str: "addiu",
                operands_str: [Some("$23"), Some("$23"), Some("0x18"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00812021, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "addu        $4, $4, $1",
                expected_opcode: Opcode::core_addu,
                opcode_str: "addu",
                operands_str: [Some("$4"), Some("$4"), Some("$1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F6A0C, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmadl       $v8, $v13, $v31[0]",
                expected_opcode: Opcode::rsp_vmadl,
                opcode_str: "vmadl",
                operands_str: [Some("$v8"), Some("$v13"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x0185602A, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "slt         $12, $12, $5",
                expected_opcode: Opcode::core_slt,
                opcode_str: "slt",
                operands_str: [Some("$12"), Some("$12"), Some("$5"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B224AF3, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vmov        $v11[1], $v2[1]",
                expected_opcode: Opcode::rsp_vmov,
                opcode_str: "vmov",
                operands_str: [Some("$v11[1]"), Some("$v2[1]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0343F0, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vrcp        $v15[0], $v3[0]",
                expected_opcode: Opcode::rsp_vrcp,
                opcode_str: "vrcp",
                operands_str: [Some("$v15[0]"), Some("$v3[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B7D4232, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vrcph       $v8[0], $v29[3]",
                expected_opcode: Opcode::rsp_vrcph,
                opcode_str: "vrcph",
                operands_str: [Some("$v8[0]"), Some("$v29[3]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4BDE0026, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vcr         $v0, $v0, $v30[6]",
                expected_opcode: Opcode::rsp_vcr,
                opcode_str: "vcr",
                operands_str: [Some("$v0"), Some("$v0"), Some("$v30[6]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A0A56D3, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vabs        $v27, $v10, $v10",
                expected_opcode: Opcode::rsp_vabs,
                opcode_str: "vabs",
                operands_str: [Some("$v27"), Some("$v10"), Some("$v10"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC9513800, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "luv         $v17[0], 0x0($10)",
                expected_opcode: Opcode::rsp_luv,
                opcode_str: "luv",
                operands_str: [Some("$v17[0]"), Some("0x0($10)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B0D41F1, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vrcpl       $v7[0], $v13[0]",
                expected_opcode: Opcode::rsp_vrcpl,
                opcode_str: "vrcpl",
                operands_str: [Some("$v7[0]"), Some("$v13[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x398C0001, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "xori        $12, $12, 0x1",
                expected_opcode: Opcode::core_xori,
                opcode_str: "xori",
                operands_str: [Some("$12"), Some("$12"), Some("0x1"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00641826, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "xor         $3, $3, $4",
                expected_opcode: Opcode::core_xor,
                opcode_str: "xor",
                operands_str: [Some("$3"), Some("$3"), Some("$4"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4ACB36D4, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vaddc       $v27, $v6, $v11[2h]",
                expected_opcode: Opcode::rsp_vaddc,
                opcode_str: "vaddc",
                operands_str: [Some("$v27"), Some("$v6"), Some("$v11[2h]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F4221, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "veq         $v8, $v8, $v31[0]",
                expected_opcode: Opcode::rsp_veq,
                opcode_str: "veq",
                operands_str: [Some("$v8"), Some("$v8"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F7BE2, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vne         $v15, $v15, $v31[0]",
                expected_opcode: Opcode::rsp_vne,
                opcode_str: "vne",
                operands_str: [Some("$v15"), Some("$v15"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F7A2D, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vnxor       $v8, $v15, $v31[0]",
                expected_opcode: Opcode::rsp_vnxor,
                opcode_str: "vnxor",
                operands_str: [Some("$v8"), Some("$v15"), Some("$v31[0]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xE8FB3800, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "suv         $v27[0], 0x0($7)",
                expected_opcode: Opcode::rsp_suv,
                opcode_str: "suv",
                operands_str: [Some("$v27[0]"), Some("0x0($7)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x03C0F809, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "jalr        $30",
                expected_opcode: Opcode::core_jalr,
                opcode_str: "jalr",
                operands_str: [Some("$30"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC86F3000, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lpv         $v15[0], 0x0($3)",
                expected_opcode: Opcode::rsp_lpv,
                opcode_str: "lpv",
                operands_str: [Some("$v15[0]"), Some("0x0($3)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4A1FEF6A, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vor         $v29, $v29, $v31",
                expected_opcode: Opcode::rsp_vor,
                opcode_str: "vor",
                operands_str: [Some("$v29"), Some("$v29"), Some("$v31"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xE9085F04, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "stv         $v8[14], 0x40($8)",
                expected_opcode: Opcode::rsp_stv,
                opcode_str: "stv",
                operands_str: [Some("$v8[14]"), Some("0x40($8)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC9085904, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "ltv         $v8[2], 0x40($8)",
                expected_opcode: Opcode::rsp_ltv,
                opcode_str: "ltv",
                operands_str: [Some("$v8[2]"), Some("0x40($8)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B1F42F6, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vrsqh       $v11[0], $v31[0]",
                expected_opcode: Opcode::rsp_vrsqh,
                opcode_str: "vrsqh",
                operands_str: [Some("$v11[0]"), Some("$v31[0]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B4743F5, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vrsql       $v15[0], $v7[2]",
                expected_opcode: Opcode::rsp_vrsql,
                opcode_str: "vrsql",
                operands_str: [Some("$v15[0]"), Some("$v7[2]"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xE8273038, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "spv         $v7[0], 0x1C0($1)",
                expected_opcode: Opcode::rsp_spv,
                opcode_str: "spv",
                operands_str: [Some("$v7[0]"), Some("0x1C0($1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0xC94F0786, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "lbv         $v15[15], 0x6($10)",
                expected_opcode: Opcode::rsp_lbv,
                opcode_str: "lbv",
                operands_str: [Some("$v15[15]"), Some("0x6($10)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x48CB0800, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "ctc2        $t3, $1",
                expected_opcode: Opcode::rsp_ctc2,
                opcode_str: "ctc2",
                operands_str: [Some("$t3"), Some("$1"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x4B3E01EB, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "vnor        $v7, $v0, $v30[1]",
                expected_opcode: Opcode::rsp_vnor,
                opcode_str: "vnor",
                operands_str: [Some("$v7"), Some("$v0"), Some("$v30[1]"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x04D1FF9D, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "bgezal      $6, .L800B3020",
                expected_opcode: Opcode::core_bgezal,
                opcode_str: "bgezal",
                operands_str: [Some("$6"), Some(".L800B3020"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00035822, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "neg         $11, $3",
                expected_opcode: Opcode::core_neg,
                opcode_str: "neg",
                operands_str: [Some("$11"), Some("$3"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00042100, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sll         $a0, $a0, 4",
                expected_opcode: Opcode::core_sll,
                opcode_str: "sll",
                operands_str: [Some("$a0"), Some("$a0"), Some("4"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00021882, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "srl        $v1, $v0, 2",
                expected_opcode: Opcode::core_srl,
                opcode_str: "srl",
                operands_str: [Some("$v1"), Some("$v0"), Some("2"), None, None],
            },
            TestEntry {
                instr: Instruction::new_rsp(0x00017443, 0xA4000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default()
                    .with_named_gpr(false)
                    .with_named_fpr(false),
                valid: true,
                expected: "sra        $t6, $at, 17",
                expected_opcode: Opcode::core_sra,
                opcode_str: "sra",
                operands_str: [Some("$t6"), Some("$at"), Some("17"), None, None],
            },
            // removed rsp instructions
            TestEntry::new_rsp_invalid(0x50740008, InstructionFlags::default()), // beql
            TestEntry::new_rsp_invalid(0x56E1FFF8, InstructionFlags::default()), // bnel
            TestEntry::new_rsp_invalid(0x59C00007, InstructionFlags::default()), // blezl
            TestEntry::new_rsp_invalid(0x5D000001, InstructionFlags::default()), // bgtzl
            TestEntry::new_rsp_invalid(0x60840001, InstructionFlags::default()), // daddi
            TestEntry::new_rsp_invalid(0x64840001, InstructionFlags::default()), // daddiu
            TestEntry::new_rsp_invalid(0x69220007, InstructionFlags::default()), // ldl
            TestEntry::new_rsp_invalid(0x6D240008, InstructionFlags::default()), // ldr
            TestEntry::new_rsp_invalid(0x88EE000D, InstructionFlags::default()), // lwl
            TestEntry::new_rsp_invalid(0x98EE0010, InstructionFlags::default()), // lwr
            TestEntry::new_rsp_invalid(0x9FA30010, InstructionFlags::default()), // lwu
            TestEntry::new_rsp_invalid(0xA8C20000, InstructionFlags::default()), // swl
            TestEntry::new_rsp_invalid(0xB0C20007, InstructionFlags::default()), // sdl
            TestEntry::new_rsp_invalid(0xB4C20000, InstructionFlags::default()), // sdr
            TestEntry::new_rsp_invalid(0xB9C1000E, InstructionFlags::default()), // swr
            TestEntry::new_rsp_invalid(0xC0850000, InstructionFlags::default()), // ll
            TestEntry::new_rsp_invalid(0xD0850000, InstructionFlags::default()), // lld
            TestEntry::new_rsp_invalid(0xDFBF0020, InstructionFlags::default()), // ld
            TestEntry::new_rsp_invalid(0xE0BF0020, InstructionFlags::default()), // sc
            TestEntry::new_rsp_invalid(0xF0BF0020, InstructionFlags::default()), // scd
            TestEntry::new_rsp_invalid(0xFFA00038, InstructionFlags::default()), // sd
            TestEntry::new_rsp_invalid(0xBCD00000, InstructionFlags::default()), // cache
            TestEntry::new_rsp_invalid(0xC4D00010, InstructionFlags::default()), // lwc1
            TestEntry::new_rsp_invalid(0xD4D00010, InstructionFlags::default()), // ldc1
            TestEntry::new_rsp_invalid(0xE4D00010, InstructionFlags::default()), // swc1
            TestEntry::new_rsp_invalid(0xF4D00010, InstructionFlags::default()), // sdc1
            // TestEntry::new_rsp_invalid(0xC8D00010, InstructionFlags::default()), // lwc2
            TestEntry::new_rsp_invalid(0xD8D00010, InstructionFlags::default()), // ldc2
            // TestEntry::new_rsp_invalid(0xE8D00010, InstructionFlags::default()), // swc2
            TestEntry::new_rsp_invalid(0xF8D00010, InstructionFlags::default()), // sdc2
            TestEntry::new_rsp_invalid(0x05020031, InstructionFlags::default()), // bltzl
            TestEntry::new_rsp_invalid(0x04630005, InstructionFlags::default()), // bgezl
            TestEntry::new_rsp_invalid(0x04A80010, InstructionFlags::default()), // tgei
            TestEntry::new_rsp_invalid(0x04A90010, InstructionFlags::default()), // tgeiu
            TestEntry::new_rsp_invalid(0x04AA0010, InstructionFlags::default()), // tlti
            TestEntry::new_rsp_invalid(0x04AB0010, InstructionFlags::default()), // tltiu
            TestEntry::new_rsp_invalid(0x04AC0010, InstructionFlags::default()), // teqi
            TestEntry::new_rsp_invalid(0x04AE0010, InstructionFlags::default()), // tnei
            TestEntry::new_rsp_invalid(0x04B20010, InstructionFlags::default()), // bltzall
            TestEntry::new_rsp_invalid(0x04B30010, InstructionFlags::default()), // bgezall
            TestEntry::new_rsp_invalid(0x00042FF8, InstructionFlags::default()), // dsll
            TestEntry::new_rsp_invalid(0x000637FA, InstructionFlags::default()), // dsrl
            TestEntry::new_rsp_invalid(0x0002137B, InstructionFlags::default()), // dsra
            TestEntry::new_rsp_invalid(0x000437FC, InstructionFlags::default()), // dsll32
            TestEntry::new_rsp_invalid(0x0005283E, InstructionFlags::default()), // dsrl32
            TestEntry::new_rsp_invalid(0x0002103F, InstructionFlags::default()), // dsra32
            TestEntry::new_rsp_invalid(0x01EE1014, InstructionFlags::default()), // dsllv
            TestEntry::new_rsp_invalid(0x01EE1016, InstructionFlags::default()), // dsrlv
            TestEntry::new_rsp_invalid(0x01EE1017, InstructionFlags::default()), // dsrav
            TestEntry::new_rsp_invalid(0x03600011, InstructionFlags::default()), // mthi
            TestEntry::new_rsp_invalid(0x03600013, InstructionFlags::default()), // mtlo
            TestEntry::new_rsp_invalid(0x00004010, InstructionFlags::default()), // mfhi
            TestEntry::new_rsp_invalid(0x00004012, InstructionFlags::default()), // mflo
            TestEntry::new_rsp_invalid(0x01AC001A, InstructionFlags::default()), // div
            TestEntry::new_rsp_invalid(0x0101001B, InstructionFlags::default()), // divu
            TestEntry::new_rsp_invalid(0x01CF001E, InstructionFlags::default()), // ddiv
            TestEntry::new_rsp_invalid(0x01CF001F, InstructionFlags::default()), // ddivu
            TestEntry::new_rsp_invalid(0x0162582C, InstructionFlags::default()), // dadd
            TestEntry::new_rsp_invalid(0x012A582D, InstructionFlags::default()), // daddu
            TestEntry::new_rsp_invalid(0x0162582E, InstructionFlags::default()), // dsub
            TestEntry::new_rsp_invalid(0x0162582F, InstructionFlags::default()), // dsubu
            TestEntry::new_rsp_invalid(0x0000000C, InstructionFlags::default()), // syscall
            TestEntry::new_rsp_invalid(0x0000000F, InstructionFlags::default()), // sync
            TestEntry::new_rsp_invalid(0x00870018, InstructionFlags::default()), // mult
            TestEntry::new_rsp_invalid(0x00E20019, InstructionFlags::default()), // multu
            TestEntry::new_rsp_invalid(0x0087001C, InstructionFlags::default()), // dmult
            TestEntry::new_rsp_invalid(0x00E2001D, InstructionFlags::default()), // dmultu
            TestEntry::new_rsp_invalid(0x00E200B0, InstructionFlags::default()), // tge
            TestEntry::new_rsp_invalid(0x00E200B1, InstructionFlags::default()), // tgeu
            TestEntry::new_rsp_invalid(0x00E200B2, InstructionFlags::default()), // tlt
            TestEntry::new_rsp_invalid(0x00E200B3, InstructionFlags::default()), // tltu
            TestEntry::new_rsp_invalid(0x00E200B4, InstructionFlags::default()), // teq
            TestEntry::new_rsp_invalid(0x00E200B6, InstructionFlags::default()), // tne
            TestEntry::new_rsp_invalid(0x40220800, InstructionFlags::default()), // dmfc0
            TestEntry::new_rsp_invalid(0x40420800, InstructionFlags::default()), // cfc0
            TestEntry::new_rsp_invalid(0x40A20800, InstructionFlags::default()), // dmtc0
            TestEntry::new_rsp_invalid(0x40C20800, InstructionFlags::default()), // ctc0
            // Invalid instructions
            TestEntry::new_rsp_invalid(0x44444444, InstructionFlags::default()),
            TestEntry::new_rsp_invalid(0x77777777, InstructionFlags::default()),
            TestEntry::new_rsp_invalid(0xEEEEEEEE, InstructionFlags::default()),
        ];

        assert_eq!(check_test_entries(ENTRIES, true), 0);
    }

    #[test]
    fn check_r3000gte_instructions() {
        const ENTRIES: &[TestEntry] = &[
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A180001,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "RTPS",
                expected_opcode: Opcode::r3000gte_RTPS,
                opcode_str: "RTPS",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A280030,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "RTPT",
                expected_opcode: Opcode::r3000gte_RTPT,
                opcode_str: "RTPT",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A680029,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "DPCL",
                expected_opcode: Opcode::r3000gte_DPCL,
                opcode_str: "DPCL",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A780010,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "DPCS",
                expected_opcode: Opcode::r3000gte_DPCS,
                opcode_str: "DPCS",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4AF8002A,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "DPCT",
                expected_opcode: Opcode::r3000gte_DPCT,
                opcode_str: "DPCT",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A980011,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "INTPL",
                expected_opcode: Opcode::r3000gte_INTPL,
                opcode_str: "INTPL",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4AC8041E,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "NCS",
                expected_opcode: Opcode::r3000gte_NCS,
                opcode_str: "NCS",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4AD80420,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "NCT",
                expected_opcode: Opcode::r3000gte_NCT,
                opcode_str: "NCT",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4AE80413,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "NCDS",
                expected_opcode: Opcode::r3000gte_NCDS,
                opcode_str: "NCDS",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4AF80416,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "NCDT",
                expected_opcode: Opcode::r3000gte_NCDT,
                opcode_str: "NCDT",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B08041B,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "NCCS",
                expected_opcode: Opcode::r3000gte_NCCS,
                opcode_str: "NCCS",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B18043F,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "NCCT",
                expected_opcode: Opcode::r3000gte_NCCT,
                opcode_str: "NCCT",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B280414,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "CDP",
                expected_opcode: Opcode::r3000gte_CDP,
                opcode_str: "CDP",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B38041C,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "CC",
                expected_opcode: Opcode::r3000gte_CC,
                opcode_str: "CC",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B400006,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "NCLIP",
                expected_opcode: Opcode::r3000gte_NCLIP,
                opcode_str: "NCLIP",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B58002D,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "AVSZ3",
                expected_opcode: Opcode::r3000gte_AVSZ3,
                opcode_str: "AVSZ3",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B68002E,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "AVSZ4",
                expected_opcode: Opcode::r3000gte_AVSZ4,
                opcode_str: "AVSZ4",
                operands_str: [Some(""), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A400012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       0, 0, 0, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("0"), Some("0"), Some("0"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4AA00428,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "SQR         0",
                expected_opcode: Opcode::r3000gte_SQR,
                opcode_str: "SQR",
                operands_str: [Some("0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B70000C,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "OP          0",
                expected_opcode: Opcode::r3000gte_OP,
                opcode_str: "OP",
                operands_str: [Some("0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B90003D,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "GPF         0",
                expected_opcode: Opcode::r3000gte_GPF,
                opcode_str: "GPF",
                operands_str: [Some("0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4BA0003E,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "GPL         0",
                expected_opcode: Opcode::r3000gte_GPL,
                opcode_str: "GPL",
                operands_str: [Some("0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A486012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 0, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("0"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A48E012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 1, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("1"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A496012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 2, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("2"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A49E012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 3, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("3"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A41E012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       0, 0, 3, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("0"), Some("0"), Some("3"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A480012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 0, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("0"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A488012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 1, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("1"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A490012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 2, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("2"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A498012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 3, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("3"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A482012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 0, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("0"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A48A012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 1, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("1"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A492012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 2, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("2"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A49A012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 0, 3, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("0"), Some("3"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4A6412,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 0, 3, 1",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("0"), Some("3"), Some("1")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4A6012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 0, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("0"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4AE012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 1, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("1"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4B6012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 2, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("2"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4BE012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 3, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("3"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4A0012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 0, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("0"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4A8012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 1, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("1"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4B0012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 2, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("2"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4B8012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 3, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("3"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4A2012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 0, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("0"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4AA012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 1, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("1"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4B2012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 2, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("2"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4BA012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 1, 3, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("1"), Some("3"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4DA412,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 3, 1, 1",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("3"), Some("1"), Some("1")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4C6012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 0, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("0"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4CE012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 1, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("1"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4D6012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 2, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("2"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4DE012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 3, 3, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("3"), Some("3"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4C0012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 0, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("0"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4C8012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 1, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("1"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4D0012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 2, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("2"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4D8012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 3, 0, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("3"), Some("0"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4C2012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 0, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("0"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4CA012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 1, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("1"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4D2012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 2, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("2"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4A4DA012,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "MVMVA       1, 2, 3, 1, 0",
                expected_opcode: Opcode::r3000gte_MVMVA,
                opcode_str: "MVMVA",
                operands_str: [Some("1"), Some("2"), Some("3"), Some("1"), Some("0")],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4AA80428,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "SQR         1",
                expected_opcode: Opcode::r3000gte_SQR,
                opcode_str: "SQR",
                operands_str: [Some("1"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B78000C,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "OP          1",
                expected_opcode: Opcode::r3000gte_OP,
                opcode_str: "OP",
                operands_str: [Some("1"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4B98003D,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "GPF         1",
                expected_opcode: Opcode::r3000gte_GPF,
                opcode_str: "GPF",
                operands_str: [Some("1"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r3000gte(
                    0x4BA8003E,
                    0x80000000,
                    InstructionFlags::default(),
                ),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "GPL         1",
                expected_opcode: Opcode::r3000gte_GPL,
                opcode_str: "GPL",
                operands_str: [Some("1"), None, None, None, None],
            },
        ];

        assert_eq!(check_test_entries(ENTRIES, true), 0);
    }

    #[test]
    fn check_r5900_instructions() {
        const ENTRIES: &[TestEntry] = &[
            TestEntry {
                instr: Instruction::new_r5900(0x4A000038, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x0",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A004038, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x800",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x800"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A008038, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x1000",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x1000"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A008838, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x1100",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x1100"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A009038, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x1200",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x1200"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A009838, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x1300",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x1300"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A00A038, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x1400",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x1400"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A07FFF8, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0xFFF8",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0xFFF8"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A080038, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x10000",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x10000"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A1F8038, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x3F000",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x3F000"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A1FFFB8, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x3FFF0",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x3FFF0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A1FFFF8, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vcallms     0x3FFF8",
                expected_opcode: Opcode::r5900_vcallms,
                opcode_str: "vcallms",
                operands_str: [Some("0x3FFF8"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x70001030, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "pmfhl.lw    $v0",
                expected_opcode: Opcode::r5900_pmfhl_lw,
                opcode_str: "pmfhl.lw",
                operands_str: [Some("$v0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x70001070, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "pmfhl.uw    $v0",
                expected_opcode: Opcode::r5900_pmfhl_uw,
                opcode_str: "pmfhl.uw",
                operands_str: [Some("$v0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x700010B0, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "pmfhl.slw   $v0",
                expected_opcode: Opcode::r5900_pmfhl_slw,
                opcode_str: "pmfhl.slw",
                operands_str: [Some("$v0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x700010F0, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "pmfhl.lh    $v0",
                expected_opcode: Opcode::r5900_pmfhl_lh,
                opcode_str: "pmfhl.lh",
                operands_str: [Some("$v0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x70001130, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "pmfhl.sh    $v0",
                expected_opcode: Opcode::r5900_pmfhl_sh,
                opcode_str: "pmfhl.sh",
                operands_str: [Some("$v0"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x70000031, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "pmthl.lw    $zero",
                expected_opcode: Opcode::r5900_pmthl_lw,
                opcode_str: "pmthl.lw",
                operands_str: [Some("$zero"), None, None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4B020BFE, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vilwr.x     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_vilwr_x,
                opcode_str: "vilwr.x",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A820BFE, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vilwr.y     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_vilwr_y,
                opcode_str: "vilwr.y",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A420BFE, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vilwr.z     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_vilwr_z,
                opcode_str: "vilwr.z",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A220BFE, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vilwr.w     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_vilwr_w,
                opcode_str: "vilwr.w",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4B020BFF, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "viswr.x     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_viswr_x,
                opcode_str: "viswr.x",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A820BFF, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "viswr.y     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_viswr_y,
                opcode_str: "viswr.y",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A420BFF, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "viswr.z     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_viswr_z,
                opcode_str: "viswr.z",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A220BFF, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "viswr.w     $vi2, ($vi1)",
                expected_opcode: Opcode::r5900_viswr_w,
                opcode_str: "viswr.w",
                operands_str: [Some("$vi2"), Some("($vi1)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x70111334, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "psllh       $v0, $s1, 12",
                expected_opcode: Opcode::r5900_psllh,
                opcode_str: "psllh",
                operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x70111336, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "psrlh       $v0, $s1, 12",
                expected_opcode: Opcode::r5900_psrlh,
                opcode_str: "psrlh",
                operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x70111337, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "psrah       $v0, $s1, 12",
                expected_opcode: Opcode::r5900_psrah,
                opcode_str: "psrah",
                operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x7011133C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "psllw       $v0, $s1, 12",
                expected_opcode: Opcode::r5900_psllw,
                opcode_str: "psllw",
                operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x7011133E, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "psrlw       $v0, $s1, 12",
                expected_opcode: Opcode::r5900_psrlw,
                opcode_str: "psrlw",
                operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x7011133F, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "psraw       $v0, $s1, 12",
                expected_opcode: Opcode::r5900_psraw,
                opcode_str: "psraw",
                operands_str: [Some("$v0"), Some("$s1"), Some("12"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A00551D, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_gnu_as(),
                valid: true,
                expected: "vmaxi       $vf20, $vf10, $I",
                expected_opcode: Opcode::r5900_vmaxi,
                opcode_str: "vmaxi",
                operands_str: [Some("$vf20"), Some("$vf10"), Some("$I"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A00551D, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_legacy_as(),
                valid: true,
                expected: "vmaxi       $vf20, $vf10, I",
                expected_opcode: Opcode::r5900_vmaxi,
                opcode_str: "vmaxi",
                operands_str: [Some("$vf20"), Some("$vf10"), Some("I"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A00551C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_gnu_as(),
                valid: true,
                expected: "vmulq       $vf20, $vf10, $Q",
                expected_opcode: Opcode::r5900_vmulq,
                opcode_str: "vmulq",
                operands_str: [Some("$vf20"), Some("$vf10"), Some("$Q"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A00551C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_legacy_as(),
                valid: true,
                expected: "vmulq       $vf20, $vf10, Q",
                expected_opcode: Opcode::r5900_vmulq,
                opcode_str: "vmulq",
                operands_str: [Some("$vf20"), Some("$vf10"), Some("Q"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06043C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_gnu_as(),
                valid: true,
                expected: "vrnext      $vf6, $R",
                expected_opcode: Opcode::r5900_vrnext,
                opcode_str: "vrnext",
                operands_str: [Some("$vf6"), Some("$R"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06043C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_legacy_as(),
                valid: true,
                expected: "vrnext      $vf6, R",
                expected_opcode: Opcode::r5900_vrnext,
                opcode_str: "vrnext",
                operands_str: [Some("$vf6"), Some("R"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06003C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_gnu_as(),
                valid: true,
                expected: "vaddax      $ACC, $vf0, $vf6x",
                expected_opcode: Opcode::r5900_vaddax,
                opcode_str: "vaddax",
                operands_str: [Some("$ACC"), Some("$vf0"), Some("$vf6x"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06003C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_legacy_as(),
                valid: true,
                expected: "vaddax      ACC, $vf0, $vf6x",
                expected_opcode: Opcode::r5900_vaddax,
                opcode_str: "vaddax",
                operands_str: [Some("ACC"), Some("$vf0"), Some("$vf6x"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A0663BC, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vdiv        Q, $vf12x, $vf6x",
                expected_opcode: Opcode::r5900_vdiv,
                opcode_str: "vdiv",
                operands_str: [Some("Q"), Some("$vf12x"), Some("$vf6x"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A066630, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "viadd       $vi24, $vi12, $vi6",
                expected_opcode: Opcode::r5900_viadd,
                opcode_str: "viadd",
                operands_str: [Some("$vi24"), Some("$vi12"), Some("$vi6"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A066632, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "viaddi      $vi6, $vi12, 0x18",
                expected_opcode: Opcode::r5900_viaddi,
                opcode_str: "viaddi",
                operands_str: [Some("$vi6"), Some("$vi12"), Some("0x18"), None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06637E, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vlqd        $vf6, (--$vi12)",
                expected_opcode: Opcode::r5900_vlqd,
                opcode_str: "vlqd",
                operands_str: [Some("$vf6"), Some("(--$vi12)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06637F, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vsqd        $vf12, (--$vi6)",
                expected_opcode: Opcode::r5900_vsqd,
                opcode_str: "vsqd",
                operands_str: [Some("$vf12"), Some("(--$vi6)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06637C, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vlqi        $vf6, ($vi12++)",
                expected_opcode: Opcode::r5900_vlqi,
                opcode_str: "vlqi",
                operands_str: [Some("$vf6"), Some("($vi12++)"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4A06637D, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::default(),
                valid: true,
                expected: "vsqi        $vf12, ($vi6++)",
                expected_opcode: Opcode::r5900_vsqi,
                opcode_str: "vsqi",
                operands_str: [Some("$vf12"), Some("($vi6++)"), None, None, None],
            },
        ];

        assert_eq!(check_test_entries(ENTRIES, true), 0);
    }

    #[test]
    fn check_r5900_trunc_cvt_instructions() {
        const ENTRIES: &[TestEntry] = &[
            TestEntry {
                instr: Instruction::new_r5900(0x4600600D, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_gnu_as(),
                valid: true,
                expected: ".word       0x4600600D                   /* trunc.w.s   $f0, $f12 / 00000000 <OpcodeCategory: CORE_COP1_FPUS> */",
                expected_opcode: Opcode::core_trunc_w_s,
                opcode_str: "trunc.w.s",
                operands_str: [Some("$f0"), Some("$f12"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x46006024, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_gnu_as(),
                valid: true,
                expected: ".word       0x46006024                   /* cvt.w.s     $f0, $f12 / 00000000 <OpcodeCategory: CORE_COP1_FPUS> */",
                expected_opcode: Opcode::core_cvt_w_s,
                opcode_str: "cvt.w.s",
                operands_str: [Some("$f0"), Some("$f12"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x4600600D, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_legacy_as(),
                valid: true,
                expected: "trunc.w.s   $f0, $f12",
                expected_opcode: Opcode::core_trunc_w_s,
                opcode_str: "trunc.w.s",
                operands_str: [Some("$f0"), Some("$f12"), None, None, None],
            },
            TestEntry {
                instr: Instruction::new_r5900(0x46006024, 0x80000000, InstructionFlags::default()),
                imm_override: None,
                display_flags: DisplayFlags::new_legacy_as(),
                valid: true,
                expected: "cvt.w.s     $f0, $f12",
                expected_opcode: Opcode::core_cvt_w_s,
                opcode_str: "cvt.w.s",
                operands_str: [Some("$f0"), Some("$f12"), None, None, None],
            },
        ];

        assert_eq!(check_test_entries(ENTRIES, true), 0);
    }
}
