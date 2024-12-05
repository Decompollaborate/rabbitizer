/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use rabbitizer::display_flags::DisplayFlags;
use rabbitizer::instr::{Instruction, InstructionFlags};
use rabbitizer::isa::IsaExtension;
use rabbitizer::opcodes::Opcode;
use rabbitizer::operands::OPERAND_COUNT_MAX;
use rabbitizer::vram::Vram;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestEntry {
    pub instr: Instruction,
    pub imm_override: Option<&'static str>,
    pub display_flags: DisplayFlags,

    pub valid: bool,

    pub expected: &'static str,
    pub expected_opcode: Opcode,
    pub opcode_str: &'static str,
    pub operands_str: [Option<&'static str>; OPERAND_COUNT_MAX],
}

impl TestEntry {
    #[allow(dead_code)]
    pub const fn new_rsp_invalid(
        word: u32,
        flags: InstructionFlags,
        expected: &'static str,
    ) -> Self {
        Self {
            instr: Instruction::new(
                word,
                Vram::new(0xA4000000),
                flags.with_isa_extension(IsaExtension::RSP),
            ),
            imm_override: None,
            display_flags: DisplayFlags::default(),
            valid: false,
            expected,
            expected_opcode: Opcode::ALL_INVALID,
            opcode_str: "INVALID",
            operands_str: [None, None, None, None, None],
        }
    }

    #[allow(dead_code)]
    pub const fn new_r4000allegrex(
        word: u32,
        expected: &'static str,
        expected_opcode: Opcode,
        opcode_str: &'static str,
        operands_str: [Option<&'static str>; OPERAND_COUNT_MAX],
    ) -> Self {
        Self {
            instr: Instruction::new(
                word,
                Vram::new(0x80000000),
                InstructionFlags::default().with_isa_extension(IsaExtension::R4000ALLEGREX),
            ),
            imm_override: None,
            display_flags: DisplayFlags::new(),
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

        if self.instr.opcode().is_branch() {
            if self.instr.get_branch_offset_generic().is_none() {
                println!(
                    "'{}' ({:08X}) is a branch opcode but `get_branch_offset_generic` returned `None`.",
                    self.opcode_str,
                    self.instr.word()
                );
                errors += 1;
            }
            if self.instr.get_branch_vram_generic().is_none() {
                println!(
                    "'{}' ({:08X}) is a branch but `get_branch_vram_generic` returned `None`.",
                    self.opcode_str,
                    self.instr.word()
                );
                errors += 1;
            }
        }

        if self.instr.opcode() == Opcode::core_j {
            if self.instr.flags().j_as_branch() {
                if self.instr.get_branch_offset_generic().is_none() {
                    println!(
                        "'{}' ({:08X}) is the `j` opcode but `get_branch_offset_generic` returned `None`.",
                        self.opcode_str,
                        self.instr.word()
                    );
                    errors += 1;
                }
                if self.instr.get_branch_vram_generic().is_none() {
                    println!(
                        "'{}' ({:08X}) is the `j` but `get_branch_vram_generic` returned `None`.",
                        self.opcode_str,
                        self.instr.word()
                    );
                    errors += 1;
                }
            } else {
                if self.instr.get_branch_offset_generic().is_some() {
                    println!(
                        "'{}' ({:08X}) is the `j` opcode but `get_branch_offset_generic` returned `Some` when `j_as_branch` is turned off.",
                        self.opcode_str,
                        self.instr.word()
                    );
                    errors += 1;
                }
                if self.instr.get_branch_vram_generic().is_some() {
                    println!(
                        "'{}' ({:08X}) is the `j` opcode but `get_branch_vram_generic` returned `Some` when `j_as_branch` is turned off.",
                        self.opcode_str,
                        self.instr.word()
                    );
                    errors += 1;
                }
            }
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

pub fn entries_sanity_check(entries: &[TestEntry]) {
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

pub fn check_test_entries(entries: &[TestEntry], thingy: bool) -> (u32, u32) {
    let mut instructions_with_errors = 0;
    let mut individual_errors = 0;

    entries_sanity_check(entries);

    for entry in entries {
        let mut errors = entry.check_validity();
        if thingy {
            errors += entry.check_disassembly();
        }

        individual_errors += errors;
        if errors != 0 {
            instructions_with_errors += 1;
        }
    }

    (instructions_with_errors, individual_errors)
}
