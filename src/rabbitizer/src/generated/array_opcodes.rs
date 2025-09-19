/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::access_type::AccessType;
#[cfg(feature = "R5900EE")]
use crate::instr::InstrSuffix;
use crate::instr::InstrType;
#[cfg(any(
    feature = "RSP",
    feature = "R3000GTE",
    feature = "R4000ALLEGREX",
    feature = "R5900EE",
))]
use crate::isa::IsaExtension;
use crate::isa::IsaVersion;
use crate::opcodes::{Opcode, OpcodeCategory, OpcodeDescriptor, OPCODE_COUNT};
use crate::operands::Operand;
pub static OPCODES: [OpcodeDescriptor; OPCODE_COUNT] = {
    let mut table = [OpcodeDescriptor::default(); OPCODE_COUNT];
    {
        table[Opcode::ALL_INVALID as usize] = OpcodeDescriptor {
            is_invalid: true,
            ..OpcodeDescriptor::new(
                Opcode::ALL_INVALID,
                OpcodeCategory::CORE_NORMAL,
                !0,
                "INVALID",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_j as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_label),
            instr_type: InstrType::J,
            is_jump: true,
            is_jump_with_address: true,
            ..OpcodeDescriptor::new(
                Opcode::core_j,
                OpcodeCategory::CORE_NORMAL,
                0x02,
                "j",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_jal as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_label),
            instr_type: InstrType::J,
            is_jump: true,
            is_jump_with_address: true,
            does_link: true,
            ..OpcodeDescriptor::new(
                Opcode::core_jal,
                OpcodeCategory::CORE_NORMAL,
                0x03,
                "jal",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_beq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::core_rs,
                Operand::core_rt,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::I,
            is_branch: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_beq,
                OpcodeCategory::CORE_NORMAL,
                0x04,
                "beq",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_bne as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::core_rs,
                Operand::core_rt,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::I,
            is_branch: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bne,
                OpcodeCategory::CORE_NORMAL,
                0x05,
                "bne",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_beql as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::core_rs,
                Operand::core_rt,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_beql,
                OpcodeCategory::CORE_NORMAL,
                0x14,
                "beql",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bnel as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::core_rs,
                Operand::core_rt,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bnel,
                OpcodeCategory::CORE_NORMAL,
                0x15,
                "bnel",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_blez as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_blez,
                OpcodeCategory::CORE_NORMAL,
                0x06,
                "blez",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_blezl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_blezl,
                OpcodeCategory::CORE_NORMAL,
                0x16,
                "blezl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_bgtz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bgtz,
                OpcodeCategory::CORE_NORMAL,
                0x07,
                "bgtz",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bgtzl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bgtzl,
                OpcodeCategory::CORE_NORMAL,
                0x17,
                "bgtzl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_addi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_addi,
                OpcodeCategory::CORE_NORMAL,
                0x08,
                "addi",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_addiu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_addiu,
                OpcodeCategory::CORE_NORMAL,
                0x09,
                "addiu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_slti as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_slti,
                OpcodeCategory::CORE_NORMAL,
                0x0A,
                "slti",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sltiu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sltiu,
                OpcodeCategory::CORE_NORMAL,
                0x0B,
                "sltiu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_andi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_u16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_andi,
                OpcodeCategory::CORE_NORMAL,
                0x0C,
                "andi",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_ori as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_u16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_unsigned_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ori,
                OpcodeCategory::CORE_NORMAL,
                0x0D,
                "ori",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_xori as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_u16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_xori,
                OpcodeCategory::CORE_NORMAL,
                0x0E,
                "xori",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_daddi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_daddi,
                OpcodeCategory::CORE_NORMAL,
                0x18,
                "daddi",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_daddiu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_daddiu,
                OpcodeCategory::CORE_NORMAL,
                0x19,
                "daddiu",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_lui as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_u16),
            instr_type: InstrType::I,
            modifies_rt: true,
            can_be_hi: true,
            ..OpcodeDescriptor::new(
                Opcode::core_lui,
                OpcodeCategory::CORE_NORMAL,
                0x0F,
                "lui",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ldl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_LEFT),
            ..OpcodeDescriptor::new(
                Opcode::core_ldl,
                OpcodeCategory::CORE_NORMAL,
                0x1A,
                "ldl",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ldr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_RIGHT),
            ..OpcodeDescriptor::new(
                Opcode::core_ldr,
                OpcodeCategory::CORE_NORMAL,
                0x1B,
                "ldr",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_lb as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::BYTE),
            ..OpcodeDescriptor::new(
                Opcode::core_lb,
                OpcodeCategory::CORE_NORMAL,
                0x20,
                "lb",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_lh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::SHORT),
            ..OpcodeDescriptor::new(
                Opcode::core_lh,
                OpcodeCategory::CORE_NORMAL,
                0x21,
                "lh",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_lwl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_WORD_LEFT),
            ..OpcodeDescriptor::new(
                Opcode::core_lwl,
                OpcodeCategory::CORE_NORMAL,
                0x22,
                "lwl",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_lw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::core_lw,
                OpcodeCategory::CORE_NORMAL,
                0x23,
                "lw",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_lbu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::BYTE),
            does_unsigned_memory_access: true,
            ..OpcodeDescriptor::new(
                Opcode::core_lbu,
                OpcodeCategory::CORE_NORMAL,
                0x24,
                "lbu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_lhu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::SHORT),
            does_unsigned_memory_access: true,
            ..OpcodeDescriptor::new(
                Opcode::core_lhu,
                OpcodeCategory::CORE_NORMAL,
                0x25,
                "lhu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_lwr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_WORD_RIGHT),
            ..OpcodeDescriptor::new(
                Opcode::core_lwr,
                OpcodeCategory::CORE_NORMAL,
                0x26,
                "lwr",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_lwu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            does_unsigned_memory_access: true,
            ..OpcodeDescriptor::new(
                Opcode::core_lwu,
                OpcodeCategory::CORE_NORMAL,
                0x27,
                "lwu",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_sb as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::BYTE),
            ..OpcodeDescriptor::new(
                Opcode::core_sb,
                OpcodeCategory::CORE_NORMAL,
                0x28,
                "sb",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::SHORT),
            ..OpcodeDescriptor::new(
                Opcode::core_sh,
                OpcodeCategory::CORE_NORMAL,
                0x29,
                "sh",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_swl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_WORD_LEFT),
            ..OpcodeDescriptor::new(
                Opcode::core_swl,
                OpcodeCategory::CORE_NORMAL,
                0x2A,
                "swl",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::core_sw,
                OpcodeCategory::CORE_NORMAL,
                0x2B,
                "sw",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_sdl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_RIGHT),
            ..OpcodeDescriptor::new(
                Opcode::core_sdl,
                OpcodeCategory::CORE_NORMAL,
                0x2C,
                "sdl",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_sdr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_LEFT),
            ..OpcodeDescriptor::new(
                Opcode::core_sdr,
                OpcodeCategory::CORE_NORMAL,
                0x2D,
                "sdr",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_swr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_WORD_RIGHT),
            ..OpcodeDescriptor::new(
                Opcode::core_swr,
                OpcodeCategory::CORE_NORMAL,
                0x2E,
                "swr",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ll as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::LINKED_WORD_WORD),
            ..OpcodeDescriptor::new(
                Opcode::core_ll,
                OpcodeCategory::CORE_NORMAL,
                0x30,
                "ll",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_IV")]
    {
        table[Opcode::core_pref as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_hint, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_pref,
                OpcodeCategory::CORE_NORMAL,
                0x33,
                "pref",
                IsaVersion::MIPS_IV,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_lld as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::LINKED_WORD_DOUBLEWORD),
            ..OpcodeDescriptor::new(
                Opcode::core_lld,
                OpcodeCategory::CORE_NORMAL,
                0x34,
                "lld",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ld as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::DOUBLEWORD),
            ..OpcodeDescriptor::new(
                Opcode::core_ld,
                OpcodeCategory::CORE_NORMAL,
                0x37,
                "ld",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::LINKED_WORD_WORD),
            ..OpcodeDescriptor::new(
                Opcode::core_sc,
                OpcodeCategory::CORE_NORMAL,
                0x38,
                "sc",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_scd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::LINKED_WORD_DOUBLEWORD),
            ..OpcodeDescriptor::new(
                Opcode::core_scd,
                OpcodeCategory::CORE_NORMAL,
                0x3C,
                "scd",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_sd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::DOUBLEWORD),
            ..OpcodeDescriptor::new(
                Opcode::core_sd,
                OpcodeCategory::CORE_NORMAL,
                0x3F,
                "sd",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_cache as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_op, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cache,
                OpcodeCategory::CORE_NORMAL,
                0x2F,
                "cache",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_lwc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_imm_rs),
            instr_type: InstrType::I,
            is_float: true,
            reads_rs: true,
            modifies_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::FLOAT),
            ..OpcodeDescriptor::new(
                Opcode::core_lwc1,
                OpcodeCategory::CORE_NORMAL,
                0x31,
                "lwc1",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ldc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_imm_rs),
            instr_type: InstrType::I,
            is_float: true,
            is_double: true,
            reads_rs: true,
            modifies_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::DOUBLEFLOAT),
            ..OpcodeDescriptor::new(
                Opcode::core_ldc1,
                OpcodeCategory::CORE_NORMAL,
                0x35,
                "ldc1",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_swc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_imm_rs),
            instr_type: InstrType::I,
            is_float: true,
            reads_rs: true,
            reads_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::FLOAT),
            ..OpcodeDescriptor::new(
                Opcode::core_swc1,
                OpcodeCategory::CORE_NORMAL,
                0x39,
                "swc1",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sdc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_imm_rs),
            instr_type: InstrType::I,
            is_float: true,
            is_double: true,
            reads_rs: true,
            reads_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::DOUBLEFLOAT),
            ..OpcodeDescriptor::new(
                Opcode::core_sdc1,
                OpcodeCategory::CORE_NORMAL,
                0x3D,
                "sdc1",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_lwc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD_COP2),
            ..OpcodeDescriptor::new(
                Opcode::core_lwc2,
                OpcodeCategory::CORE_NORMAL,
                0x32,
                "lwc2",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ldc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::DOUBLEWORD_COP2),
            ..OpcodeDescriptor::new(
                Opcode::core_ldc2,
                OpcodeCategory::CORE_NORMAL,
                0x36,
                "ldc2",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_swc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::WORD_COP2),
            ..OpcodeDescriptor::new(
                Opcode::core_swc2,
                OpcodeCategory::CORE_NORMAL,
                0x3A,
                "swc2",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sdc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::DOUBLEWORD_COP2),
            ..OpcodeDescriptor::new(
                Opcode::core_sdc2,
                OpcodeCategory::CORE_NORMAL,
                0x3E,
                "sdc2",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_b as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_b,
                OpcodeCategory::CORE_NORMAL,
                0x04,
                "b",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_beqz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            reads_rs: true,
            is_branch: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_beqz,
                OpcodeCategory::CORE_NORMAL,
                0x04,
                "beqz",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_bnez as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            reads_rs: true,
            is_branch: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bnez,
                OpcodeCategory::CORE_NORMAL,
                0x05,
                "bnez",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_beqzl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_beqzl,
                OpcodeCategory::CORE_NORMAL,
                0x14,
                "beqzl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bnezl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bnezl,
                OpcodeCategory::CORE_NORMAL,
                0x15,
                "bnezl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }

    {
        table[Opcode::core_sll as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sll,
                OpcodeCategory::CORE_SPECIAL,
                0x00,
                "sll",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_srl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_srl,
                OpcodeCategory::CORE_SPECIAL,
                0x02,
                "srl",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sra as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sra,
                OpcodeCategory::CORE_SPECIAL,
                0x03,
                "sra",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsll as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsll,
                OpcodeCategory::CORE_SPECIAL,
                0x38,
                "dsll",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsrl,
                OpcodeCategory::CORE_SPECIAL,
                0x3A,
                "dsrl",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsra as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsra,
                OpcodeCategory::CORE_SPECIAL,
                0x3B,
                "dsra",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsll32 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsll32,
                OpcodeCategory::CORE_SPECIAL,
                0x3C,
                "dsll32",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrl32 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsrl32,
                OpcodeCategory::CORE_SPECIAL,
                0x3E,
                "dsrl32",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsra32 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsra32,
                OpcodeCategory::CORE_SPECIAL,
                0x3F,
                "dsra32",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsllv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsllv,
                OpcodeCategory::CORE_SPECIAL,
                0x14,
                "dsllv",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrlv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsrlv,
                OpcodeCategory::CORE_SPECIAL,
                0x16,
                "dsrlv",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrav as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsrav,
                OpcodeCategory::CORE_SPECIAL,
                0x17,
                "dsrav",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_sllv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sllv,
                OpcodeCategory::CORE_SPECIAL,
                0x04,
                "sllv",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_srlv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_srlv,
                OpcodeCategory::CORE_SPECIAL,
                0x06,
                "srlv",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_srav as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_srav,
                OpcodeCategory::CORE_SPECIAL,
                0x07,
                "srav",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mthi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            instr_type: InstrType::R,
            reads_rs: true,
            modifies_hi: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mthi,
                OpcodeCategory::CORE_SPECIAL,
                0x11,
                "mthi",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mtlo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            instr_type: InstrType::R,
            reads_rs: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mtlo,
                OpcodeCategory::CORE_SPECIAL,
                0x13,
                "mtlo",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_jr as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            instr_type: InstrType::R,
            is_jump: true,
            jumps_to_register: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_jr,
                OpcodeCategory::CORE_SPECIAL,
                0x08,
                "jr",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_jalr as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_maybe_rd_rs),
            instr_type: InstrType::R,
            is_jump: true,
            jumps_to_register: true,
            modifies_rd: true,
            reads_rs: true,
            does_link: true,
            ..OpcodeDescriptor::new(
                Opcode::core_jalr,
                OpcodeCategory::CORE_SPECIAL,
                0x09,
                "jalr",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mfhi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_hi: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mfhi,
                OpcodeCategory::CORE_SPECIAL,
                0x10,
                "mfhi",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mflo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mflo,
                OpcodeCategory::CORE_SPECIAL,
                0x12,
                "mflo",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_IV")]
    {
        table[Opcode::core_movz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_movz,
                OpcodeCategory::CORE_SPECIAL,
                0x0A,
                "movz",
                IsaVersion::MIPS_IV,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_IV")]
    {
        table[Opcode::core_movn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_movn,
                OpcodeCategory::CORE_SPECIAL,
                0x0B,
                "movn",
                IsaVersion::MIPS_IV,
                None,
            )
        };
    }
    {
        table[Opcode::core_div as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_div,
                OpcodeCategory::CORE_SPECIAL,
                0x1A,
                "div",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_divu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_divu,
                OpcodeCategory::CORE_SPECIAL,
                0x1B,
                "divu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ddiv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ddiv,
                OpcodeCategory::CORE_SPECIAL,
                0x1E,
                "ddiv",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ddivu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ddivu,
                OpcodeCategory::CORE_SPECIAL,
                0x1F,
                "ddivu",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_add as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            adds_registers: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_add,
                OpcodeCategory::CORE_SPECIAL,
                0x20,
                "add",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_addu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            adds_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_addu,
                OpcodeCategory::CORE_SPECIAL,
                0x21,
                "addu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            not_emitted_by_compilers: true,
            reads_rs: true,
            reads_rt: true,
            subs_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sub,
                OpcodeCategory::CORE_SPECIAL,
                0x22,
                "sub",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_subu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            subs_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_subu,
                OpcodeCategory::CORE_SPECIAL,
                0x23,
                "subu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_and as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ands_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_and,
                OpcodeCategory::CORE_SPECIAL,
                0x24,
                "and",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_or as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            ors_registers: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_or,
                OpcodeCategory::CORE_SPECIAL,
                0x25,
                "or",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_xor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_xor,
                OpcodeCategory::CORE_SPECIAL,
                0x26,
                "xor",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_nor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_nor,
                OpcodeCategory::CORE_SPECIAL,
                0x27,
                "nor",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_slt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_slt,
                OpcodeCategory::CORE_SPECIAL,
                0x2A,
                "slt",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sltu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sltu,
                OpcodeCategory::CORE_SPECIAL,
                0x2B,
                "sltu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dadd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            adds_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dadd,
                OpcodeCategory::CORE_SPECIAL,
                0x2C,
                "dadd",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_daddu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            adds_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_daddu,
                OpcodeCategory::CORE_SPECIAL,
                0x2D,
                "daddu",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            subs_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsub,
                OpcodeCategory::CORE_SPECIAL,
                0x2E,
                "dsub",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsubu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            subs_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dsubu,
                OpcodeCategory::CORE_SPECIAL,
                0x2F,
                "dsubu",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_syscall as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_code_lower),
            instr_type: InstrType::R,
            causes_exception: true,
            causes_unconditional_exception: true,
            causes_returnable_exception: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_syscall,
                OpcodeCategory::CORE_SPECIAL,
                0x0C,
                "syscall",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_break as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_code),
            causes_exception: true,
            causes_unconditional_exception: true,
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::core_break,
                OpcodeCategory::CORE_SPECIAL,
                0x0D,
                "break",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sync as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::core_sync,
                OpcodeCategory::CORE_SPECIAL,
                0x0F,
                "sync",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_mult as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mult,
                OpcodeCategory::CORE_SPECIAL,
                0x18,
                "mult",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_multu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_multu,
                OpcodeCategory::CORE_SPECIAL,
                0x19,
                "multu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmult as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dmult,
                OpcodeCategory::CORE_SPECIAL,
                0x1C,
                "dmult",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmultu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dmultu,
                OpcodeCategory::CORE_SPECIAL,
                0x1D,
                "dmultu",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tge as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
            instr_type: InstrType::R,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tge,
                OpcodeCategory::CORE_SPECIAL,
                0x30,
                "tge",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tgeu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
            instr_type: InstrType::R,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tgeu,
                OpcodeCategory::CORE_SPECIAL,
                0x31,
                "tgeu",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tlt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
            instr_type: InstrType::R,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tlt,
                OpcodeCategory::CORE_SPECIAL,
                0x32,
                "tlt",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tltu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
            instr_type: InstrType::R,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tltu,
                OpcodeCategory::CORE_SPECIAL,
                0x33,
                "tltu",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_teq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
            instr_type: InstrType::R,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_teq,
                OpcodeCategory::CORE_SPECIAL,
                0x34,
                "teq",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tne as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rs, Operand::core_rt, Operand::core_code_lower),
            instr_type: InstrType::R,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tne,
                OpcodeCategory::CORE_SPECIAL,
                0x36,
                "tne",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_nop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_nop,
                OpcodeCategory::CORE_SPECIAL,
                0x00,
                "nop",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_not as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_not,
                OpcodeCategory::CORE_SPECIAL,
                0x27,
                "not",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_neg as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            not_emitted_by_compilers: true,
            modifies_rd: true,
            reads_rt: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_neg,
                OpcodeCategory::CORE_SPECIAL,
                0x22,
                "neg",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_negu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_negu,
                OpcodeCategory::CORE_SPECIAL,
                0x23,
                "negu",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_bltz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bltz,
                OpcodeCategory::CORE_REGIMM,
                0x00,
                "bltz",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_bgez as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bgez,
                OpcodeCategory::CORE_REGIMM,
                0x01,
                "bgez",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bltzl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bltzl,
                OpcodeCategory::CORE_REGIMM,
                0x02,
                "bltzl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bgezl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bgezl,
                OpcodeCategory::CORE_REGIMM,
                0x03,
                "bgezl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tgei as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tgei,
                OpcodeCategory::CORE_REGIMM,
                0x08,
                "tgei",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tgeiu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tgeiu,
                OpcodeCategory::CORE_REGIMM,
                0x09,
                "tgeiu",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tlti as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tlti,
                OpcodeCategory::CORE_REGIMM,
                0x0A,
                "tlti",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tltiu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tltiu,
                OpcodeCategory::CORE_REGIMM,
                0x0B,
                "tltiu",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_teqi as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_teqi,
                OpcodeCategory::CORE_REGIMM,
                0x0C,
                "teqi",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tnei as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tnei,
                OpcodeCategory::CORE_REGIMM,
                0x0E,
                "tnei",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_bltzal as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            does_link: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bltzal,
                OpcodeCategory::CORE_REGIMM,
                0x10,
                "bltzal",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_bgezal as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            does_link: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bgezal,
                OpcodeCategory::CORE_REGIMM,
                0x11,
                "bgezal",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bltzall as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            does_link: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bltzall,
                OpcodeCategory::CORE_REGIMM,
                0x12,
                "bltzall",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bgezall as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            does_link: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bgezall,
                OpcodeCategory::CORE_REGIMM,
                0x13,
                "bgezall",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_bal as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            not_emitted_by_compilers: true,
            does_link: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bal,
                OpcodeCategory::CORE_REGIMM,
                0x11,
                "bal",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mfc0,
                OpcodeCategory::CORE_COP0,
                0x00,
                "mfc0",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dmfc0,
                OpcodeCategory::CORE_COP0,
                0x01,
                "dmfc0",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_cfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0cd),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cfc0,
                OpcodeCategory::CORE_COP0,
                0x02,
                "cfc0",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mtc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mtc0,
                OpcodeCategory::CORE_COP0,
                0x04,
                "mtc0",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmtc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dmtc0,
                OpcodeCategory::CORE_COP0,
                0x05,
                "dmtc0",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_ctc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0cd),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ctc0,
                OpcodeCategory::CORE_COP0,
                0x06,
                "ctc0",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }

    {
        table[Opcode::core_bc0f as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc0f,
                OpcodeCategory::CORE_COP0_BC0,
                0x00,
                "bc0f",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_bc0t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc0t,
                OpcodeCategory::CORE_COP0_BC0,
                0x01,
                "bc0t",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc0fl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc0fl,
                OpcodeCategory::CORE_COP0_BC0,
                0x02,
                "bc0fl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc0tl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc0tl,
                OpcodeCategory::CORE_COP0_BC0,
                0x03,
                "bc0tl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_tlbr as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tlbr,
                OpcodeCategory::CORE_COP0_TLB,
                0x01,
                "tlbr",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_tlbwi as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tlbwi,
                OpcodeCategory::CORE_COP0_TLB,
                0x02,
                "tlbwi",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_tlbwr as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                Opcode::core_tlbwr,
                OpcodeCategory::CORE_COP0_TLB,
                0x06,
                "tlbwr",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_tlbp as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_tlbp,
                OpcodeCategory::CORE_COP0_TLB,
                0x08,
                "tlbp",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_rfe as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_rfe,
                OpcodeCategory::CORE_COP0_TLB,
                0x10,
                "rfe",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_eret as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            causes_exception: true,
            causes_unconditional_exception: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::core_eret,
                OpcodeCategory::CORE_COP0_TLB,
                0x18,
                "eret",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_mfc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_rt: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mfc1,
                OpcodeCategory::CORE_COP1,
                0x00,
                "mfc1",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmfc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_rt: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dmfc1,
                OpcodeCategory::CORE_COP1,
                0x01,
                "dmfc1",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_mtc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_rt: true,
            modifies_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mtc1,
                OpcodeCategory::CORE_COP1,
                0x04,
                "mtc1",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmtc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_rt: true,
            modifies_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_dmtc1,
                OpcodeCategory::CORE_COP1,
                0x05,
                "dmtc1",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_cfc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop1cs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cfc1,
                OpcodeCategory::CORE_COP1,
                0x02,
                "cfc1",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_ctc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop1cs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ctc1,
                OpcodeCategory::CORE_COP1,
                0x06,
                "ctc1",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }

    {
        table[Opcode::core_bc1f as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc1f,
                OpcodeCategory::CORE_COP1_BC1,
                0x00,
                "bc1f",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_bc1t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc1t,
                OpcodeCategory::CORE_COP1_BC1,
                0x01,
                "bc1t",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc1fl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc1fl,
                OpcodeCategory::CORE_COP1_BC1,
                0x02,
                "bc1fl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc1tl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::core_bc1tl,
                OpcodeCategory::CORE_COP1_BC1,
                0x03,
                "bc1tl",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_add_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_add_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x00,
                "add.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sub_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sub_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x01,
                "sub.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mul_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mul_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x02,
                "mul.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_div_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_div_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x03,
                "div.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sqrt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sqrt_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x04,
                "sqrt.s",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_abs_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_abs_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x05,
                "abs.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mov_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mov_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x06,
                "mov.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_neg_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_neg_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x07,
                "neg.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_round_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_round_l_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x08,
                "round.l.s",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_trunc_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_trunc_l_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x09,
                "trunc.l.s",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ceil_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ceil_l_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x0A,
                "ceil.l.s",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_floor_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_floor_l_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x0B,
                "floor.l.s",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_round_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_round_w_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x0C,
                "round.w.s",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_trunc_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_trunc_w_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x0D,
                "trunc.w.s",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ceil_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ceil_w_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x0E,
                "ceil.w.s",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_floor_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_floor_w_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x0F,
                "floor.w.s",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_cvt_d_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_d_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x21,
                "cvt.d.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_cvt_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_w_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x24,
                "cvt.w.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_cvt_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_l_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x25,
                "cvt.l.s",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_f_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_f_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x30,
                "c.f.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_un_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_un_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x31,
                "c.un.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_eq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_eq_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x32,
                "c.eq.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ueq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ueq_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x33,
                "c.ueq.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_olt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_olt_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x34,
                "c.olt.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ult_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ult_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x35,
                "c.ult.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ole_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ole_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x36,
                "c.ole.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ule_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ule_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x37,
                "c.ule.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_sf_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_sf_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x38,
                "c.sf.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ngle_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ngle_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x39,
                "c.ngle.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_seq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_seq_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x3A,
                "c.seq.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ngl_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ngl_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x3B,
                "c.ngl.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_lt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_lt_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x3C,
                "c.lt.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_nge_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_nge_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x3D,
                "c.nge.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_le_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_le_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x3E,
                "c.le.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ngt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ngt_s,
                OpcodeCategory::CORE_COP1_FPUS,
                0x3F,
                "c.ngt.s",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_add_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_add_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x00,
                "add.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_sub_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sub_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x01,
                "sub.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mul_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mul_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x02,
                "mul.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_div_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_div_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x03,
                "div.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sqrt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_sqrt_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x04,
                "sqrt.d",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_abs_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_abs_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x05,
                "abs.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mov_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mov_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x06,
                "mov.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_neg_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_neg_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x07,
                "neg.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_round_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_round_l_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x08,
                "round.l.d",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_trunc_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_trunc_l_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x09,
                "trunc.l.d",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ceil_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ceil_l_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x0A,
                "ceil.l.d",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_floor_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_floor_l_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x0B,
                "floor.l.d",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_round_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_round_w_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x0C,
                "round.w.d",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_trunc_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_trunc_w_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x0D,
                "trunc.w.d",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ceil_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ceil_w_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x0E,
                "ceil.w.d",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_floor_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_floor_w_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x0F,
                "floor.w.d",
                IsaVersion::MIPS_II,
                None,
            )
        };
    }
    {
        table[Opcode::core_cvt_s_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_s_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x20,
                "cvt.s.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_cvt_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_w_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x24,
                "cvt.w.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_cvt_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_l_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x25,
                "cvt.l.d",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_f_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_f_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x30,
                "c.f.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_un_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_un_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x31,
                "c.un.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_eq_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_eq_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x32,
                "c.eq.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ueq_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ueq_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x33,
                "c.ueq.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_olt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_olt_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x34,
                "c.olt.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ult_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ult_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x35,
                "c.ult.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ole_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ole_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x36,
                "c.ole.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ule_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ule_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x37,
                "c.ule.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_df_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_df_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x38,
                "c.df.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ngle_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ngle_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x39,
                "c.ngle.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_seq_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_seq_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x3A,
                "c.seq.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ngl_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ngl_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x3B,
                "c.ngl.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_lt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_lt_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x3C,
                "c.lt.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_nge_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_nge_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x3D,
                "c.nge.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_le_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_le_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x3E,
                "c.le.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_c_ngt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::core_c_ngt_d,
                OpcodeCategory::CORE_COP1_FPUD,
                0x3F,
                "c.ngt.d",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_cvt_s_w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_s_w,
                OpcodeCategory::CORE_COP1_FPUW,
                0x20,
                "cvt.s.w",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_cvt_d_w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_d_w,
                OpcodeCategory::CORE_COP1_FPUW,
                0x21,
                "cvt.d.w",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_cvt_s_l as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_s_l,
                OpcodeCategory::CORE_COP1_FPUL,
                0x20,
                "cvt.s.l",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_cvt_d_l as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cvt_d_l,
                OpcodeCategory::CORE_COP1_FPUL,
                0x21,
                "cvt.d.l",
                IsaVersion::MIPS_III,
                None,
            )
        };
    }
    {
        table[Opcode::core_mfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2d),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mfc2,
                OpcodeCategory::CORE_COP2,
                0x00,
                "mfc2",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_mtc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2d),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_mtc2,
                OpcodeCategory::CORE_COP2,
                0x04,
                "mtc2",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_cfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2cd),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_cfc2,
                OpcodeCategory::CORE_COP2,
                0x02,
                "cfc2",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    {
        table[Opcode::core_ctc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2cd),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::core_ctc2,
                OpcodeCategory::CORE_COP2,
                0x06,
                "ctc2",
                IsaVersion::MIPS_I,
                None,
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_vs_index),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_mfc2,
                OpcodeCategory::RSP_COP2,
                0x00,
                "mfc2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mtc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_vs_index),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_mtc2,
                OpcodeCategory::RSP_COP2,
                0x04,
                "mtc2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_cfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_cfc2,
                OpcodeCategory::RSP_COP2,
                0x02,
                "cfc2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ctc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_ctc2,
                OpcodeCategory::RSP_COP2,
                0x06,
                "ctc2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }

    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmulf as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmulf,
                OpcodeCategory::RSP_COP2_VU,
                0x00,
                "vmulf",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmulu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmulu,
                OpcodeCategory::RSP_COP2_VU,
                0x01,
                "vmulu",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrndp as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrndp,
                OpcodeCategory::RSP_COP2_VU,
                0x02,
                "vrndp",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmulq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmulq,
                OpcodeCategory::RSP_COP2_VU,
                0x03,
                "vmulq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmudl,
                OpcodeCategory::RSP_COP2_VU,
                0x04,
                "vmudl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudm as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmudm,
                OpcodeCategory::RSP_COP2_VU,
                0x05,
                "vmudm",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmudn,
                OpcodeCategory::RSP_COP2_VU,
                0x06,
                "vmudn",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmudh,
                OpcodeCategory::RSP_COP2_VU,
                0x07,
                "vmudh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmacf as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmacf,
                OpcodeCategory::RSP_COP2_VU,
                0x08,
                "vmacf",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmacu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmacu,
                OpcodeCategory::RSP_COP2_VU,
                0x09,
                "vmacu",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrndn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrndn,
                OpcodeCategory::RSP_COP2_VU,
                0x0A,
                "vrndn",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmacq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmacq,
                OpcodeCategory::RSP_COP2_VU,
                0x0B,
                "vmacq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmadl,
                OpcodeCategory::RSP_COP2_VU,
                0x0C,
                "vmadl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadm as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmadm,
                OpcodeCategory::RSP_COP2_VU,
                0x0D,
                "vmadm",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmadn,
                OpcodeCategory::RSP_COP2_VU,
                0x0E,
                "vmadn",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmadh,
                OpcodeCategory::RSP_COP2_VU,
                0x0F,
                "vmadh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vadd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vadd,
                OpcodeCategory::RSP_COP2_VU,
                0x10,
                "vadd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vsub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsub,
                OpcodeCategory::RSP_COP2_VU,
                0x11,
                "vsub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsut as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsut,
                OpcodeCategory::RSP_COP2_VU,
                0x12,
                "vsut",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vabs as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vabs,
                OpcodeCategory::RSP_COP2_VU,
                0x13,
                "vabs",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vaddc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vaddc,
                OpcodeCategory::RSP_COP2_VU,
                0x14,
                "vaddc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vsubc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsubc,
                OpcodeCategory::RSP_COP2_VU,
                0x15,
                "vsubc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vaddb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vaddb,
                OpcodeCategory::RSP_COP2_VU,
                0x16,
                "vaddb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsubb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsubb,
                OpcodeCategory::RSP_COP2_VU,
                0x17,
                "vsubb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vaccb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vaccb,
                OpcodeCategory::RSP_COP2_VU,
                0x18,
                "vaccb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsucb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsucb,
                OpcodeCategory::RSP_COP2_VU,
                0x19,
                "vsucb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsad as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsad,
                OpcodeCategory::RSP_COP2_VU,
                0x1A,
                "vsad",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsac as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsac,
                OpcodeCategory::RSP_COP2_VU,
                0x1B,
                "vsac",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsum as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsum,
                OpcodeCategory::RSP_COP2_VU,
                0x1C,
                "vsum",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vsar as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsar,
                OpcodeCategory::RSP_COP2_VU,
                0x1D,
                "vsar",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vacc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vacc,
                OpcodeCategory::RSP_COP2_VU,
                0x1E,
                "vacc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsuc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vsuc,
                OpcodeCategory::RSP_COP2_VU,
                0x1F,
                "vsuc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vlt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vlt,
                OpcodeCategory::RSP_COP2_VU,
                0x20,
                "vlt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_veq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_veq,
                OpcodeCategory::RSP_COP2_VU,
                0x21,
                "veq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vne as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vne,
                OpcodeCategory::RSP_COP2_VU,
                0x22,
                "vne",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vge as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vge,
                OpcodeCategory::RSP_COP2_VU,
                0x23,
                "vge",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vcl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vcl,
                OpcodeCategory::RSP_COP2_VU,
                0x24,
                "vcl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vch as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vch,
                OpcodeCategory::RSP_COP2_VU,
                0x25,
                "vch",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vcr as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vcr,
                OpcodeCategory::RSP_COP2_VU,
                0x26,
                "vcr",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmrg as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmrg,
                OpcodeCategory::RSP_COP2_VU,
                0x27,
                "vmrg",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vand,
                OpcodeCategory::RSP_COP2_VU,
                0x28,
                "vand",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vnand,
                OpcodeCategory::RSP_COP2_VU,
                0x29,
                "vnand",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vor,
                OpcodeCategory::RSP_COP2_VU,
                0x2A,
                "vor",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vnor,
                OpcodeCategory::RSP_COP2_VU,
                0x2B,
                "vnor",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vxor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vxor,
                OpcodeCategory::RSP_COP2_VU,
                0x2C,
                "vxor",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnxor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vnxor,
                OpcodeCategory::RSP_COP2_VU,
                0x2D,
                "vnxor",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_v056 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_v056,
                OpcodeCategory::RSP_COP2_VU,
                0x2E,
                "v056",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_v057 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_v057,
                OpcodeCategory::RSP_COP2_VU,
                0x2F,
                "v057",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrcp as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrcp,
                OpcodeCategory::RSP_COP2_VU,
                0x30,
                "vrcp",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrcpl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrcpl,
                OpcodeCategory::RSP_COP2_VU,
                0x31,
                "vrcpl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrcph as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrcph,
                OpcodeCategory::RSP_COP2_VU,
                0x32,
                "vrcph",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmov as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vmov,
                OpcodeCategory::RSP_COP2_VU,
                0x33,
                "vmov",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrsq as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrsq,
                OpcodeCategory::RSP_COP2_VU,
                0x34,
                "vrsq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrsql as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrsql,
                OpcodeCategory::RSP_COP2_VU,
                0x35,
                "vrsql",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrsqh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vrsqh,
                OpcodeCategory::RSP_COP2_VU,
                0x36,
                "vrsqh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vnop,
                OpcodeCategory::RSP_COP2_VU,
                0x37,
                "vnop",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vextt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vextt,
                OpcodeCategory::RSP_COP2_VU,
                0x38,
                "vextt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vextq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vextq,
                OpcodeCategory::RSP_COP2_VU,
                0x39,
                "vextq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vextn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vextn,
                OpcodeCategory::RSP_COP2_VU,
                0x3A,
                "vextn",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_v073 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_v073,
                OpcodeCategory::RSP_COP2_VU,
                0x3B,
                "v073",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vinst as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vinst,
                OpcodeCategory::RSP_COP2_VU,
                0x3C,
                "vinst",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vinsq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vinsq,
                OpcodeCategory::RSP_COP2_VU,
                0x3D,
                "vinsq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vinsn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vinsn,
                OpcodeCategory::RSP_COP2_VU,
                0x3E,
                "vinsn",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vnull as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::rsp_vnull,
                OpcodeCategory::RSP_COP2_VU,
                0x3F,
                "vnull",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lbv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset7_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lbv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x00,
                "lbv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lsv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset8_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lsv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x01,
                "lsv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_llv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset9_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_llv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x02,
                "llv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ldv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset10_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_ldv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x03,
                "ldv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lqv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lqv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x04,
                "lqv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lrv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lrv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x05,
                "lrv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lpv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset10_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lpv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x06,
                "lpv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_luv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset10_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_luv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x07,
                "luv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lhv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lhv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x08,
                "lhv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lfv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lfv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x09,
                "lfv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_lwv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_lwv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x0A,
                "lwv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ltv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_ltv,
                OpcodeCategory::RSP_NORMAL_LWC2,
                0x0B,
                "ltv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sbv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset7_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_sbv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x00,
                "sbv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ssv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset8_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_ssv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x01,
                "ssv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_slv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset9_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_slv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x02,
                "slv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sdv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset10_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_sdv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x03,
                "sdv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sqv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_sqv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x04,
                "sqv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_srv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_srv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x05,
                "srv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_spv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset10_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_spv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x06,
                "spv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_suv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset10_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_suv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x07,
                "suv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_shv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_shv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x08,
                "shv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sfv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_sfv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x09,
                "sfv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_swv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_swv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x0A,
                "swv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_stv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset11_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_stv,
                OpcodeCategory::RSP_NORMAL_SWC2,
                0x0B,
                "stv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }

    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_mfc0,
                OpcodeCategory::RSP_COP0,
                0x00,
                "mfc0",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mtc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new(
                Opcode::rsp_mtc0,
                OpcodeCategory::RSP_COP0,
                0x04,
                "mtc0",
                IsaVersion::EXTENSION,
                Some(IsaExtension::RSP),
            )
        };
    }

    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_rtps as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_rtps,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x01,
                "rtps",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_rtpt as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_rtpt,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x30,
                "rtpt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_dpcl as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_dpcl,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x29,
                "dpcl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_dpcs as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_dpcs,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x10,
                "dpcs",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_dpct as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_gbg),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_dpct,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x2A,
                "dpct",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_intpl as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_intpl,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x11,
                "intpl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_ncs as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_ncs,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x1E,
                "ncs",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_nct as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_nct,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x20,
                "nct",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_ncds as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_ncds,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x13,
                "ncds",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_ncdt as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_ncdt,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x16,
                "ncdt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_nccs as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_nccs,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x1B,
                "nccs",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_ncct as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_ncct,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x3F,
                "ncct",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_cdp as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_cdp,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x14,
                "cdp",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_cc as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_cc,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x1C,
                "cc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_nclip as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_nclip,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x06,
                "nclip",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_avsz3 as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_avsz3,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x2D,
                "avsz3",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_avsz4 as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_avsz4,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x2E,
                "avsz4",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_mvmva as usize] = OpcodeDescriptor {
            operands: Operand::arr5(
                Operand::r3000gte_sf,
                Operand::r3000gte_mx,
                Operand::r3000gte_v,
                Operand::r3000gte_cv,
                Operand::r3000gte_lm,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_mvmva,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x12,
                "mvmva",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_sqr as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_sqr,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x28,
                "sqr",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_op as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_op,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x0C,
                "op",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_gpf as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_gpf,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x3D,
                "gpf",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_gpl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new(
                Opcode::r3000gte_gpl,
                OpcodeCategory::R3000GTE_COP2_GTE,
                0x3E,
                "gpl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R3000GTE),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_lv_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_s_vt_imm,
                Operand::r4000allegrex_offset14_rs,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_lv_s,
                OpcodeCategory::R4000ALLEGREX_NORMAL,
                0x32,
                "lv.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_sv_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_s_vt_imm,
                Operand::r4000allegrex_offset14_rs,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_sv_s,
                OpcodeCategory::R4000ALLEGREX_NORMAL,
                0x3A,
                "sv.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_lv_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_rs,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_lv_q,
                OpcodeCategory::R4000ALLEGREX_NORMAL,
                0x36,
                "lv.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_sv_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_rs_maybe_wb,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_sv_q,
                OpcodeCategory::R4000ALLEGREX_NORMAL,
                0x3E,
                "sv.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_clz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_clz,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x16,
                "clz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_clo as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_clo,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x17,
                "clo",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_madd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_madd,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x1C,
                "madd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_maddu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_maddu,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x1D,
                "maddu",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_msub as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_msub,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x2E,
                "msub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_msubu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_msubu,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x2F,
                "msubu",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_movz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_movz,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x0A,
                "movz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_movn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_movn,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x0B,
                "movn",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_max as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_max,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x2C,
                "max",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_min as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_min,
                OpcodeCategory::R4000ALLEGREX_SPECIAL,
                0x2D,
                "min",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_srl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_srl,
                OpcodeCategory::R4000ALLEGREX_SPECIAL_RS,
                0x00,
                "srl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_rotr as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_rotr,
                OpcodeCategory::R4000ALLEGREX_SPECIAL_RS,
                0x01,
                "rotr",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_srlv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_srlv,
                OpcodeCategory::R4000ALLEGREX_SPECIAL_SA,
                0x00,
                "srlv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_rotrv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_rotrv,
                OpcodeCategory::R4000ALLEGREX_SPECIAL_SA,
                0x01,
                "rotrv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_sleep as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_sleep,
                OpcodeCategory::R4000ALLEGREX_SPECIAL2,
                0x00,
                "sleep",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mfie as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_mfie,
                OpcodeCategory::R4000ALLEGREX_SPECIAL2,
                0x24,
                "mfie",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mtie as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rt),
            instr_type: InstrType::R,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_mtie,
                OpcodeCategory::R4000ALLEGREX_SPECIAL2,
                0x26,
                "mtie",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_ext as usize] = OpcodeDescriptor {
            operands: Operand::arr4(
                Operand::core_rt,
                Operand::core_rs,
                Operand::r4000allegrex_pos,
                Operand::r4000allegrex_size,
            ),
            instr_type: InstrType::R,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_ext,
                OpcodeCategory::R4000ALLEGREX_SPECIAL3,
                0x00,
                "ext",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_ins as usize] = OpcodeDescriptor {
            operands: Operand::arr4(
                Operand::core_rt,
                Operand::core_rs,
                Operand::r4000allegrex_pos,
                Operand::r4000allegrex_size_plus_pos,
            ),
            instr_type: InstrType::R,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_ins,
                OpcodeCategory::R4000ALLEGREX_SPECIAL3,
                0x04,
                "ins",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_wsbh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_wsbh,
                OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL,
                0x02,
                "wsbh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_wsbw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_wsbw,
                OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL,
                0x03,
                "wsbw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_seb as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_seb,
                OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL,
                0x10,
                "seb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_seh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_seh,
                OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL,
                0x18,
                "seh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_bitrev as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_bitrev,
                OpcodeCategory::R4000ALLEGREX_SPECIAL3_BSHFL,
                0x14,
                "bitrev",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_bvf as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_imm3,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_bvf,
                OpcodeCategory::R4000ALLEGREX_COP2_BC2,
                0x00,
                "bvf",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_bvt as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_imm3,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_bvt,
                OpcodeCategory::R4000ALLEGREX_COP2_BC2,
                0x01,
                "bvt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_bvfl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_imm3,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_bvfl,
                OpcodeCategory::R4000ALLEGREX_COP2_BC2,
                0x02,
                "bvfl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_bvtl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_imm3,
                Operand::core_branch_target_label,
            ),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_bvtl,
                OpcodeCategory::R4000ALLEGREX_COP2_BC2,
                0x03,
                "bvtl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mfv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_mfv,
                OpcodeCategory::R4000ALLEGREX_COP2_MFHC2,
                0x0,
                "mfv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mfvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_cop2cd),
            instr_type: InstrType::R,
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_mfvc,
                OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P,
                0x0,
                "mfvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsync2 as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsync2,
                OpcodeCategory::R4000ALLEGREX_COP2_MFHC2_P_S,
                0xF,
                "vsync2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mtv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_mtv,
                OpcodeCategory::R4000ALLEGREX_COP2_MTHC2,
                0x0,
                "mtv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mtvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_cop2cd),
            instr_type: InstrType::R,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_mtvc,
                OpcodeCategory::R4000ALLEGREX_COP2_MTHC2,
                0x1,
                "mtvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vadd_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vadd_s,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x00000,
                "vadd.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vadd_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vadd_p,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x00001,
                "vadd.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vadd_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vadd_t,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x00100,
                "vadd.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vadd_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vadd_q,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x00101,
                "vadd.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsub_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsub_s,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x10000,
                "vsub.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsub_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsub_p,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x10001,
                "vsub.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsub_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsub_t,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x10100,
                "vsub.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsub_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsub_q,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x10101,
                "vsub.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsbn_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsbn_s,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x20000,
                "vsbn.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdiv_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdiv_s,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x70000,
                "vdiv.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdiv_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdiv_p,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x70001,
                "vdiv.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdiv_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdiv_t,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x70100,
                "vdiv.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdiv_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdiv_q,
                OpcodeCategory::R4000ALLEGREX_VFPU0,
                0x70101,
                "vdiv.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmul_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmul_s,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x00000,
                "vmul.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmul_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmul_p,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x00001,
                "vmul.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmul_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmul_t,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x00100,
                "vmul.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmul_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmul_q,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x00101,
                "vmul.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdot_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdot_p,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x10001,
                "vdot.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdot_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdot_t,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x10100,
                "vdot.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdot_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdot_q,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x10101,
                "vdot.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vscl_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vscl_p,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x20001,
                "vscl.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vscl_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vscl_t,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x20100,
                "vscl.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vscl_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vscl_q,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x20101,
                "vscl.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vhdp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vhdp_p,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x40001,
                "vhdp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vhdp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vhdp_t,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x40100,
                "vhdp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vhdp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vhdp_q,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x40101,
                "vhdp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcrs_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcrs_t,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x50100,
                "vcrs.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vdet_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vdet_p,
                OpcodeCategory::R4000ALLEGREX_VFPU1,
                0x60001,
                "vdet.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmp_s,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x00000,
                "vcmp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmp_p,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x00001,
                "vcmp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmp_t,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x00100,
                "vcmp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmp_q,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x00101,
                "vcmp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmin_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmin_s,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x20000,
                "vmin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmin_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmin_p,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x20001,
                "vmin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmin_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmin_t,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x20100,
                "vmin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmin_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmin_q,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x20101,
                "vmin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmax_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmax_s,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x30000,
                "vmax.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmax_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmax_p,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x30001,
                "vmax.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmax_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmax_t,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x30100,
                "vmax.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmax_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmax_q,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x30101,
                "vmax.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vscmp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vscmp_s,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x50000,
                "vscmp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vscmp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vscmp_p,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x50001,
                "vscmp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vscmp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vscmp_t,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x50100,
                "vscmp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vscmp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vscmp_q,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x50101,
                "vscmp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsge_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsge_s,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x60000,
                "vsge.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsge_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsge_p,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x60001,
                "vsge.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsge_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsge_t,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x60100,
                "vsge.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsge_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsge_q,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x60101,
                "vsge.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vslt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vslt_s,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x70000,
                "vslt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vslt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vslt_p,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x70001,
                "vslt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vslt_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vslt_t,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x70100,
                "vslt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vslt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vslt_q,
                OpcodeCategory::R4000ALLEGREX_VFPU3,
                0x70101,
                "vslt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vwbn_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_bn,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vwbn_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4,
                0x60000,
                "vwbn.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmov_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0000,
                "vmov.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmov_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0001,
                "vmov.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmov_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0100,
                "vmov.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmov_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0101,
                "vmov.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vabs_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0200,
                "vabs.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vabs_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0201,
                "vabs.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vabs_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0300,
                "vabs.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vabs_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0301,
                "vabs.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vneg_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0400,
                "vneg.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vneg_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0401,
                "vneg.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vneg_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0500,
                "vneg.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vneg_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0501,
                "vneg.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vidt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vidt_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0601,
                "vidt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vidt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vidt_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0701,
                "vidt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat0_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0800,
                "vsat0.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat0_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0801,
                "vsat0.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat0_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0900,
                "vsat0.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat0_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0901,
                "vsat0.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat1_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0A00,
                "vsat1.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat1_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0A01,
                "vsat1.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat1_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0B00,
                "vsat1.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsat1_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0B01,
                "vsat1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vzero_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0C00,
                "vzero.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vzero_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0C01,
                "vzero.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vzero_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0D00,
                "vzero.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vzero_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0D01,
                "vzero.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vone_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0E00,
                "vone.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vone_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0E01,
                "vone.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vone_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0F00,
                "vone.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vone_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT0,
                0x0F01,
                "vone.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrcp_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0000,
                "vrcp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrcp_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0001,
                "vrcp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrcp_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0100,
                "vrcp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrcp_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0101,
                "vrcp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrsq_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0200,
                "vrsq.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrsq_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0201,
                "vrsq.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrsq_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0300,
                "vrsq.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrsq_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0301,
                "vrsq.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsin_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0400,
                "vsin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsin_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0401,
                "vsin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsin_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0500,
                "vsin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsin_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0501,
                "vsin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcos_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0600,
                "vcos.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcos_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0601,
                "vcos.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcos_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0700,
                "vcos.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcos_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0701,
                "vcos.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vexp2_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0800,
                "vexp2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vexp2_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0801,
                "vexp2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vexp2_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0900,
                "vexp2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vexp2_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0901,
                "vexp2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vlog2_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0A00,
                "vlog2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vlog2_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0A01,
                "vlog2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vlog2_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0B00,
                "vlog2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vlog2_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0B01,
                "vlog2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsqrt_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0C00,
                "vsqrt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsqrt_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0C01,
                "vsqrt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsqrt_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0D00,
                "vsqrt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsqrt_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0D01,
                "vsqrt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vasin_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0E00,
                "vasin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vasin_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0E01,
                "vasin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vasin_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0F00,
                "vasin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vasin_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT2,
                0x0F01,
                "vasin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnrcp_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0000,
                "vnrcp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnrcp_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0001,
                "vnrcp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnrcp_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0100,
                "vnrcp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnrcp_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0101,
                "vnrcp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnsin_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0400,
                "vnsin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnsin_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0401,
                "vnsin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnsin_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0500,
                "vnsin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnsin_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0501,
                "vnsin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrexp2_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0800,
                "vrexp2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrexp2_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0801,
                "vrexp2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrexp2_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0900,
                "vrexp2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrexp2_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT3,
                0x0901,
                "vrexp2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrnds_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrnds_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0000,
                "vrnds.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndi_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0200,
                "vrndi.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndi_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0201,
                "vrndi.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndi_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0300,
                "vrndi.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndi_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0301,
                "vrndi.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf1_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0400,
                "vrndf1.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf1_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0401,
                "vrndf1.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf1_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0500,
                "vrndf1.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf1_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0501,
                "vrndf1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf2_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0600,
                "vrndf2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf2_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0601,
                "vrndf2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf2_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0700,
                "vrndf2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrndf2_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_RND,
                0x0701,
                "vrndf2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2h_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2h_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
                0x0401,
                "vf2h.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2h_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2h_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
                0x0501,
                "vf2h.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vh2f_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vh2f_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
                0x0600,
                "vh2f.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vh2f_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vh2f_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
                0x0601,
                "vh2f.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsbz_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsbz_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
                0x0C00,
                "vsbz.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlgb_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vlgb_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTFLT,
                0x0E00,
                "vlgb.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vuc2ifs_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vuc2ifs_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0000,
                "vuc2ifs.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vc2i_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vc2i_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0200,
                "vc2i.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vus2i_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vus2i_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0400,
                "vus2i.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vus2i_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vus2i_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0401,
                "vus2i.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vs2i_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vs2i_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0600,
                "vs2i.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vs2i_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vs2i_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0601,
                "vs2i.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2uc_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2uc_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0901,
                "vi2uc.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2c_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2c_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0B01,
                "vi2c.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2us_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2us_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0C01,
                "vi2us.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2us_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2us_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0D01,
                "vi2us.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2s_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2s_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0E01,
                "vi2s.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2s_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2s_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CVTINT,
                0x0F01,
                "vi2s.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsrt1_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0101,
                "vsrt1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsrt2_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0301,
                "vsrt2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vbfy1_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vbfy1_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0401,
                "vbfy1.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vbfy1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vbfy1_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0501,
                "vbfy1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vbfy2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vbfy2_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0701,
                "vbfy2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vocp_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0800,
                "vocp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vocp_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0801,
                "vocp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vocp_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0900,
                "vocp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vocp_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0901,
                "vocp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsocp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsocp_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0A00,
                "vsocp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsocp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsocp_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0A01,
                "vsocp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfad_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vfad_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0C01,
                "vfad.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfad_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vfad_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0D00,
                "vfad.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfad_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vfad_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0D01,
                "vfad.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vavg_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vavg_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0E01,
                "vavg.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vavg_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vavg_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0F00,
                "vavg.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vavg_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vavg_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT8,
                0x0F01,
                "vavg.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt3_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsrt3_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9,
                0x0101,
                "vsrt3.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt4_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsrt4_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9,
                0x0301,
                "vsrt4.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsgn_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9,
                0x0400,
                "vsgn.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsgn_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9,
                0x0401,
                "vsgn.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsgn_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9,
                0x0500,
                "vsgn.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsgn_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_FMT9,
                0x0501,
                "vsgn.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmfvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_cop2cs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmfvc,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CONTROL,
                0x0100,
                "vmfvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmtvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_cop2cd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmtvc,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CONTROL,
                0x0201,
                "vmtvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vt4444_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vt4444_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_COLOR,
                0x0301,
                "vt4444.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vt5551_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vt5551_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_COLOR,
                0x0501,
                "vt5551.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vt5650_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vt5650_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_COLOR,
                0x0701,
                "vt5650.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcst_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_vconstant,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcst_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST,
                0x0,
                "vcst.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcst_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_vconstant,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcst_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST,
                0x1,
                "vcst.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcst_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_vconstant,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcst_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST,
                0x100,
                "vcst.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcst_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_vconstant,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcst_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT0_CST,
                0x101,
                "vcst.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2in_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2in_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x00000,
                "vf2in.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2in_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2in_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x00001,
                "vf2in.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2in_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2in_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x00100,
                "vf2in.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2in_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2in_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x00101,
                "vf2in.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iz_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iz_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x04000,
                "vf2iz.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iz_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iz_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x04001,
                "vf2iz.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iz_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iz_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x04100,
                "vf2iz.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iz_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iz_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x04101,
                "vf2iz.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iu_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iu_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x08000,
                "vf2iu.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iu_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iu_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x08001,
                "vf2iu.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iu_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iu_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x08100,
                "vf2iu.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2iu_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2iu_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x08101,
                "vf2iu.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2id_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2id_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x0C000,
                "vf2id.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2id_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2id_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x0C001,
                "vf2id.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2id_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2id_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x0C100,
                "vf2id.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2id_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vf2id_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x0C101,
                "vf2id.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2f_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2f_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x10000,
                "vi2f.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2f_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2f_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x10001,
                "vi2f.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2f_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2f_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x10100,
                "vi2f.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2f_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_power_of_two,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vi2f_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2,
                0x10101,
                "vi2f.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovt_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x00000,
                "vcmovt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovt_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x00001,
                "vcmovt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovt_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovt_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x00100,
                "vcmovt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovt_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x00101,
                "vcmovt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovf_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_s_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovf_s,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x01000,
                "vcmovf.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovf_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_p_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovf_p,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x01001,
                "vcmovf.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovf_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovf_t,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x01100,
                "vcmovf.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmovf_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_vfpu_cc_bit,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcmovf_q,
                OpcodeCategory::R4000ALLEGREX_VFPU4_FMT2_CNDMOVE,
                0x01101,
                "vcmovf.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vpfxs as usize] = OpcodeDescriptor {
            operands: Operand::arr4(
                Operand::r4000allegrex_rpx,
                Operand::r4000allegrex_rpy,
                Operand::r4000allegrex_rpz,
                Operand::r4000allegrex_rpw,
            ),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vpfxs,
                OpcodeCategory::R4000ALLEGREX_VFPU5,
                0x00,
                "vpfxs",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vpfxt as usize] = OpcodeDescriptor {
            operands: Operand::arr4(
                Operand::r4000allegrex_rpx,
                Operand::r4000allegrex_rpy,
                Operand::r4000allegrex_rpz,
                Operand::r4000allegrex_rpw,
            ),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vpfxt,
                OpcodeCategory::R4000ALLEGREX_VFPU5,
                0x02,
                "vpfxt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vpfxd as usize] = OpcodeDescriptor {
            operands: Operand::arr4(
                Operand::r4000allegrex_wpx,
                Operand::r4000allegrex_wpy,
                Operand::r4000allegrex_wpz,
                Operand::r4000allegrex_wpw,
            ),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vpfxd,
                OpcodeCategory::R4000ALLEGREX_VFPU5,
                0x04,
                "vpfxd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_viim_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_int16),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_viim_s,
                OpcodeCategory::R4000ALLEGREX_VFPU5,
                0x6,
                "viim.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfim_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_float16),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vfim_s,
                OpcodeCategory::R4000ALLEGREX_VFPU5,
                0x7,
                "vfim.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmul_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_mp_vd,
                Operand::r4000allegrex_mp_vs_transpose,
                Operand::r4000allegrex_mp_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmmul_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x00001,
                "vmmul.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmul_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_mt_vd,
                Operand::r4000allegrex_mt_vs_transpose,
                Operand::r4000allegrex_mt_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmmul_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x00100,
                "vmmul.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmul_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_mq_vd,
                Operand::r4000allegrex_mq_vs_transpose,
                Operand::r4000allegrex_mq_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmmul_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x00101,
                "vmmul.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vhtfm2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_mp_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vhtfm2_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x10000,
                "vhtfm2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vtfm2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_mp_vs,
                Operand::r4000allegrex_p_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vtfm2_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x10001,
                "vtfm2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vhtfm3_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_mt_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vhtfm3_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x20001,
                "vhtfm3.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vtfm3_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_mt_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vtfm3_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x20100,
                "vtfm3.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vhtfm4_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_mq_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vhtfm4_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x30100,
                "vhtfm4.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vtfm4_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_mq_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vtfm4_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x30101,
                "vtfm4.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmscl_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_mp_vd,
                Operand::r4000allegrex_mp_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmscl_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x40001,
                "vmscl.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmscl_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_mt_vd,
                Operand::r4000allegrex_mt_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmscl_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x40100,
                "vmscl.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmscl_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_mq_vd,
                Operand::r4000allegrex_mq_vs,
                Operand::r4000allegrex_s_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmscl_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x40101,
                "vmscl.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcrsp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_t_vs,
                Operand::r4000allegrex_t_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vcrsp_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x50100,
                "vcrsp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vqmul_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_q_vs,
                Operand::r4000allegrex_q_vt,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vqmul_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6,
                0x50101,
                "vqmul.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrot_p as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_p_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_p_vrot_code,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrot_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7,
                0x04001,
                "vrot.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrot_t as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_t_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_t_vrot_code,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrot_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7,
                0x04100,
                "vrot.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrot_q as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r4000allegrex_q_vd,
                Operand::r4000allegrex_s_vs,
                Operand::r4000allegrex_q_vrot_code,
            ),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vrot_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7,
                0x04101,
                "vrot.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmov_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_mp_vd, Operand::r4000allegrex_mp_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmmov_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0001,
                "vmmov.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmov_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_mt_vd, Operand::r4000allegrex_mt_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmmov_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0100,
                "vmmov.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmov_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_mq_vd, Operand::r4000allegrex_mq_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmmov_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0101,
                "vmmov.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmidt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmidt_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0601,
                "vmidt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmidt_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmidt_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0700,
                "vmidt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmidt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmidt_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0701,
                "vmidt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmzero_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmzero_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0C01,
                "vmzero.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmzero_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmzero_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0D00,
                "vmzero.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmzero_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmzero_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0D01,
                "vmzero.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmone_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmone_p,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0E01,
                "vmone.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmone_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmone_t,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0F00,
                "vmone.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmone_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vmone_q,
                OpcodeCategory::R4000ALLEGREX_VFPU6_FMT7_FMT0,
                0x0F01,
                "vmone.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vnop,
                OpcodeCategory::R4000ALLEGREX_VFPU7,
                0x3FF0000,
                "vnop",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsync as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vsync,
                OpcodeCategory::R4000ALLEGREX_VFPU7,
                0x3FF0320,
                "vsync",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vflush as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_vflush,
                OpcodeCategory::R4000ALLEGREX_VFPU7,
                0x3FF040D,
                "vflush",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_svl_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_rs,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_svl_q,
                OpcodeCategory::R4000ALLEGREX_QUADLR,
                0x0,
                "svl.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_svr_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_rs,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r4000allegrex_svr_q,
                OpcodeCategory::R4000ALLEGREX_QUADLR,
                0x1,
                "svr.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_lq as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_lq,
                OpcodeCategory::R5900EE_NORMAL,
                0x1E,
                "lq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_sq as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_imm_rs),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_sq,
                OpcodeCategory::R5900EE_NORMAL,
                0x1F,
                "sq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_lqc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vft, Operand::core_imm_rs),
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_lqc2,
                OpcodeCategory::R5900EE_NORMAL,
                0x36,
                "lqc2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_sqc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vft, Operand::core_imm_rs),
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_sqc2,
                OpcodeCategory::R5900EE_NORMAL,
                0x3E,
                "sqc2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_sync_p as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_sync_p,
                OpcodeCategory::R5900EE_SPECIAL,
                0x0F,
                "sync.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mult as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mult,
                OpcodeCategory::R5900EE_SPECIAL,
                0x18,
                "mult",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mfsa as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mfsa,
                OpcodeCategory::R5900EE_SPECIAL,
                0x28,
                "mfsa",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtsa as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mtsa,
                OpcodeCategory::R5900EE_SPECIAL,
                0x29,
                "mtsa",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtsab as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mtsab,
                OpcodeCategory::R5900EE_REGIMM,
                0x18,
                "mtsab",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtsah as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_imm_i16),
            instr_type: InstrType::REGIMM,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mtsah,
                OpcodeCategory::R5900EE_REGIMM,
                0x19,
                "mtsah",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_madd,
                OpcodeCategory::R5900EE_MMI,
                0x00,
                "madd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_maddu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_maddu,
                OpcodeCategory::R5900EE_MMI,
                0x01,
                "maddu",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_plzcw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_plzcw,
                OpcodeCategory::R5900EE_MMI,
                0x04,
                "plzcw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mfhi1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mfhi1,
                OpcodeCategory::R5900EE_MMI,
                0x10,
                "mfhi1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mthi1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mthi1,
                OpcodeCategory::R5900EE_MMI,
                0x11,
                "mthi1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mflo1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            reads_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mflo1,
                OpcodeCategory::R5900EE_MMI,
                0x12,
                "mflo1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtlo1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mtlo1,
                OpcodeCategory::R5900EE_MMI,
                0x13,
                "mtlo1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mult1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mult1,
                OpcodeCategory::R5900EE_MMI,
                0x18,
                "mult1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_multu1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_multu1,
                OpcodeCategory::R5900EE_MMI,
                0x19,
                "multu1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_div1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_zero, Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_div1,
                OpcodeCategory::R5900EE_MMI,
                0x1A,
                "div1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_divu1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_zero, Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_divu1,
                OpcodeCategory::R5900EE_MMI,
                0x1B,
                "divu1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madd1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_madd1,
                OpcodeCategory::R5900EE_MMI,
                0x20,
                "madd1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_maddu1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_maddu1,
                OpcodeCategory::R5900EE_MMI,
                0x21,
                "maddu1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psllh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psllh,
                OpcodeCategory::R5900EE_MMI,
                0x34,
                "psllh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrlh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psrlh,
                OpcodeCategory::R5900EE_MMI,
                0x36,
                "psrlh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrah as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psrah,
                OpcodeCategory::R5900EE_MMI,
                0x37,
                "psrah",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psllw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psllw,
                OpcodeCategory::R5900EE_MMI,
                0x3C,
                "psllw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrlw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psrlw,
                OpcodeCategory::R5900EE_MMI,
                0x3E,
                "psrlw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psraw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psraw,
                OpcodeCategory::R5900EE_MMI,
                0x3F,
                "psraw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_paddw,
                OpcodeCategory::R5900EE_MMI_0,
                0x00,
                "paddw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubw,
                OpcodeCategory::R5900EE_MMI_0,
                0x01,
                "psubw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcgtw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pcgtw,
                OpcodeCategory::R5900EE_MMI_0,
                0x02,
                "pcgtw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaxw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmaxw,
                OpcodeCategory::R5900EE_MMI_0,
                0x03,
                "pmaxw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_paddh,
                OpcodeCategory::R5900EE_MMI_0,
                0x04,
                "paddh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubh,
                OpcodeCategory::R5900EE_MMI_0,
                0x05,
                "psubh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcgth as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pcgth,
                OpcodeCategory::R5900EE_MMI_0,
                0x06,
                "pcgth",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaxh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmaxh,
                OpcodeCategory::R5900EE_MMI_0,
                0x07,
                "pmaxh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_paddb,
                OpcodeCategory::R5900EE_MMI_0,
                0x08,
                "paddb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubb,
                OpcodeCategory::R5900EE_MMI_0,
                0x09,
                "psubb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcgtb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pcgtb,
                OpcodeCategory::R5900EE_MMI_0,
                0x0A,
                "pcgtb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddsw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_paddsw,
                OpcodeCategory::R5900EE_MMI_0,
                0x10,
                "paddsw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubsw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubsw,
                OpcodeCategory::R5900EE_MMI_0,
                0x11,
                "psubsw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextlw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pextlw,
                OpcodeCategory::R5900EE_MMI_0,
                0x12,
                "pextlw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppacw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_ppacw,
                OpcodeCategory::R5900EE_MMI_0,
                0x13,
                "ppacw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddsh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_paddsh,
                OpcodeCategory::R5900EE_MMI_0,
                0x14,
                "paddsh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubsh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubsh,
                OpcodeCategory::R5900EE_MMI_0,
                0x15,
                "psubsh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextlh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pextlh,
                OpcodeCategory::R5900EE_MMI_0,
                0x16,
                "pextlh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppach as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_ppach,
                OpcodeCategory::R5900EE_MMI_0,
                0x17,
                "ppach",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddsb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_paddsb,
                OpcodeCategory::R5900EE_MMI_0,
                0x18,
                "paddsb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubsb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubsb,
                OpcodeCategory::R5900EE_MMI_0,
                0x19,
                "psubsb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextlb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pextlb,
                OpcodeCategory::R5900EE_MMI_0,
                0x1A,
                "pextlb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppacb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_ppacb,
                OpcodeCategory::R5900EE_MMI_0,
                0x1B,
                "ppacb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pext5 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pext5,
                OpcodeCategory::R5900EE_MMI_0,
                0x1E,
                "pext5",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppac5 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_ppac5,
                OpcodeCategory::R5900EE_MMI_0,
                0x1F,
                "ppac5",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pabsw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pabsw,
                OpcodeCategory::R5900EE_MMI_1,
                0x01,
                "pabsw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pceqw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pceqw,
                OpcodeCategory::R5900EE_MMI_1,
                0x02,
                "pceqw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pminw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pminw,
                OpcodeCategory::R5900EE_MMI_1,
                0x03,
                "pminw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_padsbh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_padsbh,
                OpcodeCategory::R5900EE_MMI_1,
                0x04,
                "padsbh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pabsh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pabsh,
                OpcodeCategory::R5900EE_MMI_1,
                0x05,
                "pabsh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pceqh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pceqh,
                OpcodeCategory::R5900EE_MMI_1,
                0x06,
                "pceqh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pminh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pminh,
                OpcodeCategory::R5900EE_MMI_1,
                0x07,
                "pminh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pceqb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pceqb,
                OpcodeCategory::R5900EE_MMI_1,
                0x0A,
                "pceqb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_padduw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_padduw,
                OpcodeCategory::R5900EE_MMI_1,
                0x10,
                "padduw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubuw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubuw,
                OpcodeCategory::R5900EE_MMI_1,
                0x11,
                "psubuw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextuw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pextuw,
                OpcodeCategory::R5900EE_MMI_1,
                0x12,
                "pextuw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_padduh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_padduh,
                OpcodeCategory::R5900EE_MMI_1,
                0x14,
                "padduh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubuh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubuh,
                OpcodeCategory::R5900EE_MMI_1,
                0x15,
                "psubuh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextuh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pextuh,
                OpcodeCategory::R5900EE_MMI_1,
                0x16,
                "pextuh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            adds_registers: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_paddub,
                OpcodeCategory::R5900EE_MMI_1,
                0x18,
                "paddub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psubub,
                OpcodeCategory::R5900EE_MMI_1,
                0x19,
                "psubub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pextub,
                OpcodeCategory::R5900EE_MMI_1,
                0x1A,
                "pextub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qfsrv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_qfsrv,
                OpcodeCategory::R5900EE_MMI_1,
                0x1B,
                "qfsrv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaddw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmaddw,
                OpcodeCategory::R5900EE_MMI_2,
                0x00,
                "pmaddw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psllvw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psllvw,
                OpcodeCategory::R5900EE_MMI_2,
                0x02,
                "psllvw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrlvw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psrlvw,
                OpcodeCategory::R5900EE_MMI_2,
                0x03,
                "psrlvw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmsubw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmsubw,
                OpcodeCategory::R5900EE_MMI_2,
                0x04,
                "pmsubw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmfhi,
                OpcodeCategory::R5900EE_MMI_2,
                0x08,
                "pmfhi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmflo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmflo,
                OpcodeCategory::R5900EE_MMI_2,
                0x09,
                "pmflo",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pinth as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pinth,
                OpcodeCategory::R5900EE_MMI_2,
                0x0A,
                "pinth",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmultw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmultw,
                OpcodeCategory::R5900EE_MMI_2,
                0x0C,
                "pmultw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pdivw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pdivw,
                OpcodeCategory::R5900EE_MMI_2,
                0x0D,
                "pdivw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcpyld as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pcpyld,
                OpcodeCategory::R5900EE_MMI_2,
                0x0E,
                "pcpyld",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaddh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmaddh,
                OpcodeCategory::R5900EE_MMI_2,
                0x10,
                "pmaddh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_phmadh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_phmadh,
                OpcodeCategory::R5900EE_MMI_2,
                0x11,
                "phmadh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pand,
                OpcodeCategory::R5900EE_MMI_2,
                0x12,
                "pand",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pxor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pxor,
                OpcodeCategory::R5900EE_MMI_2,
                0x13,
                "pxor",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmsubh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmsubh,
                OpcodeCategory::R5900EE_MMI_2,
                0x14,
                "pmsubh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_phmsbh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_phmsbh,
                OpcodeCategory::R5900EE_MMI_2,
                0x15,
                "phmsbh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexeh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pexeh,
                OpcodeCategory::R5900EE_MMI_2,
                0x1A,
                "pexeh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_prevh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_prevh,
                OpcodeCategory::R5900EE_MMI_2,
                0x1B,
                "prevh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmulth as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmulth,
                OpcodeCategory::R5900EE_MMI_2,
                0x1C,
                "pmulth",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pdivbw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pdivbw,
                OpcodeCategory::R5900EE_MMI_2,
                0x1D,
                "pdivbw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexew as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pexew,
                OpcodeCategory::R5900EE_MMI_2,
                0x1E,
                "pexew",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_prot3w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_prot3w,
                OpcodeCategory::R5900EE_MMI_2,
                0x1F,
                "prot3w",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmadduw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmadduw,
                OpcodeCategory::R5900EE_MMI_3,
                0x00,
                "pmadduw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psravw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_psravw,
                OpcodeCategory::R5900EE_MMI_3,
                0x03,
                "psravw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmthi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmthi,
                OpcodeCategory::R5900EE_MMI_3,
                0x08,
                "pmthi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmtlo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmtlo,
                OpcodeCategory::R5900EE_MMI_3,
                0x09,
                "pmtlo",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pinteh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pinteh,
                OpcodeCategory::R5900EE_MMI_3,
                0x0A,
                "pinteh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmultuw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmultuw,
                OpcodeCategory::R5900EE_MMI_3,
                0x0C,
                "pmultuw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pdivuw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pdivuw,
                OpcodeCategory::R5900EE_MMI_3,
                0x0D,
                "pdivuw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcpyud as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pcpyud,
                OpcodeCategory::R5900EE_MMI_3,
                0x0E,
                "pcpyud",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_por as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_por,
                OpcodeCategory::R5900EE_MMI_3,
                0x12,
                "por",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pnor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pnor,
                OpcodeCategory::R5900EE_MMI_3,
                0x13,
                "pnor",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexch as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pexch,
                OpcodeCategory::R5900EE_MMI_3,
                0x1A,
                "pexch",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcpyh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pcpyh,
                OpcodeCategory::R5900EE_MMI_3,
                0x1B,
                "pcpyh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexcw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pexcw,
                OpcodeCategory::R5900EE_MMI_3,
                0x1E,
                "pexcw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_lw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmfhl_lw,
                OpcodeCategory::R5900EE_MMI_PMFHL,
                0x00,
                "pmfhl.lw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_uw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmfhl_uw,
                OpcodeCategory::R5900EE_MMI_PMFHL,
                0x01,
                "pmfhl.uw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_slw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmfhl_slw,
                OpcodeCategory::R5900EE_MMI_PMFHL,
                0x02,
                "pmfhl.slw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_lh as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmfhl_lh,
                OpcodeCategory::R5900EE_MMI_PMFHL,
                0x03,
                "pmfhl.lh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_sh as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmfhl_sh,
                OpcodeCategory::R5900EE_MMI_PMFHL,
                0x04,
                "pmfhl.sh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmthl_lw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_pmthl_lw,
                OpcodeCategory::R5900EE_MMI_PMTHL,
                0x00,
                "pmthl.lw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ei as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_ei,
                OpcodeCategory::R5900EE_COP0_TLB,
                0x38,
                "ei",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_di as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_di,
                OpcodeCategory::R5900EE_COP0_TLB,
                0x39,
                "di",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_c1__sqrt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_copraw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_c1__sqrt_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x04,
                "c1",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_rsqrt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_rsqrt_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x16,
                "rsqrt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_adda_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_adda_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x18,
                "adda.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_suba_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_suba_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x19,
                "suba.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mula_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_mula_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x1A,
                "mula.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madd_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_madd_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x1C,
                "madd.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_msub_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_msub_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x1D,
                "msub.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madda_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_madda_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x1E,
                "madda.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_msuba_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_msuba_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x1F,
                "msuba.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_max_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_max_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x28,
                "max.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_min_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_min_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x29,
                "min.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_c_lt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_c_lt_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x34,
                "c.lt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_c_le_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_c_le_s,
                OpcodeCategory::R5900EE_COP1_FPUS,
                0x36,
                "c.le.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmfc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_qmfc2_ni,
                OpcodeCategory::R5900EE_COP2_NI,
                0x01,
                "qmfc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_cfc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_cfc2_ni,
                OpcodeCategory::R5900EE_COP2_NI,
                0x02,
                "cfc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmtc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_qmtc2_ni,
                OpcodeCategory::R5900EE_COP2_NI,
                0x05,
                "qmtc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ctc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_ctc2_ni,
                OpcodeCategory::R5900EE_COP2_NI,
                0x06,
                "ctc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmfc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_qmfc2_i,
                OpcodeCategory::R5900EE_COP2_I,
                0x01,
                "qmfc2.i",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_cfc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_cfc2_i,
                OpcodeCategory::R5900EE_COP2_I,
                0x02,
                "cfc2.i",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmtc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_qmtc2_i,
                OpcodeCategory::R5900EE_COP2_I,
                0x05,
                "qmtc2.i",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ctc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_ctc2_i,
                OpcodeCategory::R5900EE_COP2_I,
                0x06,
                "ctc2.i",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2f as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_bc2f,
                OpcodeCategory::R5900EE_COP2_BC2,
                0x00,
                "bc2f",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_bc2t,
                OpcodeCategory::R5900EE_COP2_BC2,
                0x01,
                "bc2t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2fl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_bc2fl,
                OpcodeCategory::R5900EE_COP2_BC2,
                0x02,
                "bc2fl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2tl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_bc2tl,
                OpcodeCategory::R5900EE_COP2_BC2,
                0x03,
                "bc2tl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddx as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddx,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x00,
                "vaddx",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddy as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddy,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x01,
                "vaddy",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddz,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x02,
                "vaddz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddw,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x03,
                "vaddw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubx as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubx,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x04,
                "vsubx",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsuby as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsuby,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x05,
                "vsuby",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubz,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x06,
                "vsubz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubw,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x07,
                "vsubw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddx as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddx,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x08,
                "vmaddx",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddy as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddy,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x09,
                "vmaddy",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddz,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x0A,
                "vmaddz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddw,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x0B,
                "vmaddw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubx as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubx,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x0C,
                "vmsubx",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsuby as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsuby,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x0D,
                "vmsuby",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubz,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x0E,
                "vmsubz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubw,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x0F,
                "vmsubw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaxx as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaxx,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x10,
                "vmaxx",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaxy as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaxy,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x11,
                "vmaxy",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaxz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaxz,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x12,
                "vmaxz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaxw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaxw,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x13,
                "vmaxw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vminix as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vminix,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x14,
                "vminix",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vminiy as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vminiy,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x15,
                "vminiy",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vminiz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vminiz,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x16,
                "vminiz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vminiw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vminiw,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x17,
                "vminiw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulx as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulx,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x18,
                "vmulx",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmuly as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmuly,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x19,
                "vmuly",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulz,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x1A,
                "vmulz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulw,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x1B,
                "vmulw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulq,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x1C,
                "vmulq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaxi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaxi,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x1D,
                "vmaxi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmuli as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmuli,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x1E,
                "vmuli",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vminii as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vminii,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x1F,
                "vminii",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddq,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x20,
                "vaddq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddq,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x21,
                "vmaddq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddi,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x22,
                "vaddi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddi,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x23,
                "vmaddi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubq,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x24,
                "vsubq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubq,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x25,
                "vmsubq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubi,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x26,
                "vsubi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubi,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x27,
                "vmsubi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vadd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vadd,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x28,
                "vadd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmadd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmadd,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x29,
                "vmadd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmul as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmul,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x2A,
                "vmul",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmax as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmax,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x2B,
                "vmax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsub,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x2C,
                "vsub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsub,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x2D,
                "vmsub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vopmsub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vopmsub,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x2E,
                "vopmsub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmini as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vfdxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmini,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x2F,
                "vmini",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viadd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_viadd,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x30,
                "viadd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_visub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_visub,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x31,
                "visub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viaddi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vit,
                Operand::r5900ee_vis,
                Operand::r5900ee_imm5,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_viaddi,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x32,
                "viaddi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_viand,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x34,
                "viand",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vior as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vior,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x35,
                "vior",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vcallms as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r5900ee_imm15),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vcallms,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x38,
                "vcallms",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vcallmsr as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r5900ee_vis),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vcallmsr,
                OpcodeCategory::R5900EE_COP2_SPECIAL1,
                0x39,
                "vcallmsr",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddax as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddax,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x00,
                "vaddax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vadday as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vadday,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x01,
                "vadday",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddaz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddaz,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x02,
                "vaddaz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddaw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddaw,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x03,
                "vaddaw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubax as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubax,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x40,
                "vsubax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubay as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubay,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x41,
                "vsubay",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubaz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubaz,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x42,
                "vsubaz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubaw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubaw,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x43,
                "vsubaw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddax as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddax,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x80,
                "vmaddax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmadday as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmadday,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x81,
                "vmadday",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddaz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddaz,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x82,
                "vmaddaz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddaw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddaw,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x83,
                "vmaddaw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubax as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubax,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0xC0,
                "vmsubax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubay as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubay,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0xC1,
                "vmsubay",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubaz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubaz,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0xC2,
                "vmsubaz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubaw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubaw,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0xC3,
                "vmsubaw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vitof0,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x100,
                "vitof0",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof4 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vitof4,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x101,
                "vitof4",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof12 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vitof12,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x102,
                "vitof12",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof15 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vitof15,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x103,
                "vitof15",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vftoi0,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x140,
                "vftoi0",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi4 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vftoi4,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x141,
                "vftoi4",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi12 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vftoi12,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x142,
                "vftoi12",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi15 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vftoi15,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x143,
                "vftoi15",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulax as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulax,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x180,
                "vmulax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulay as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulay,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x181,
                "vmulay",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulaz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulaz,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x182,
                "vmulaz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulaw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftn,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulaw,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x183,
                "vmulaw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulaq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulaq,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x1C0,
                "vmulaq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vabs as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vabs,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x1C1,
                "vabs",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmulai as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmulai,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x1C2,
                "vmulai",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vclipw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vclipw,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x1C3,
                "vclipw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddaq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddaq,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x200,
                "vaddaq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddaq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddaq,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x201,
                "vmaddaq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vaddai as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vaddai,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x202,
                "vaddai",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmaddai as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmaddai,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x203,
                "vmaddai",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubaq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubaq,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x240,
                "vsubaq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubaq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_Q,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubaq,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x241,
                "vmsubaq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsubai as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsubai,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x242,
                "vsubai",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsubai as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_I,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsubai,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x243,
                "vmsubai",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vadda as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vftxyzw,
                Operand::r5900ee_vfsxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vadda,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x280,
                "vadda",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmadda as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmadda,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x281,
                "vmadda",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmula as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmula,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x282,
                "vmula",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsuba as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsuba,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x2C0,
                "vsuba",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmsuba as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vftxyzw,
                Operand::r5900ee_vfsxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmsuba,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x2C1,
                "vmsuba",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vopmula as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_ACCxyzw,
                Operand::r5900ee_vfsxyzw,
                Operand::r5900ee_vftxyzw,
            ),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vopmula,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x2C2,
                "vopmula",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vnop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vnop,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x2C3,
                "vnop",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmove as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmove,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x300,
                "vmove",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmr32 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmr32,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x301,
                "vmr32",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vlqi as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis_postincr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vlqi,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x340,
                "vlqi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsqi as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vit_postincr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsqi,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x341,
                "vsqi",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vlqd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis_predecr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vlqd,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x342,
                "vlqd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsqd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vit_predecr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsqd,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x343,
                "vsqd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vdiv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_Q,
                Operand::r5900ee_vfsl,
                Operand::r5900ee_vftm,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vdiv,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x380,
                "vdiv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsqrt as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_Q, Operand::r5900ee_vftm),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vsqrt,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x381,
                "vsqrt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrsqrt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_Q,
                Operand::r5900ee_vfsl,
                Operand::r5900ee_vftm,
            ),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vrsqrt,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x382,
                "vrsqrt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vwaitq as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vwaitq,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x383,
                "vwaitq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmtir as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vfsl),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmtir,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x3C0,
                "vmtir",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmfir as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vmfir,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x3C1,
                "vmfir",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrnext as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_R),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vrnext,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x400,
                "vrnext",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrget as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_R),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vrget,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x401,
                "vrget",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrinit as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_R, Operand::r5900ee_vfsl),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vrinit,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x402,
                "vrinit",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrxor as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_R, Operand::r5900ee_vfsl),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vrxor,
                OpcodeCategory::R5900EE_COP2_SPECIAL2,
                0x403,
                "vrxor",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vilwr_w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vilwr_w,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x0200002,
                "vilwr.w",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vilwr_z as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vilwr_z,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x0400002,
                "vilwr.z",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vilwr_y as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vilwr_y,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x0800002,
                "vilwr.y",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vilwr_x as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_vilwr_x,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x1000002,
                "vilwr.x",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viswr_w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_viswr_w,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x0200003,
                "viswr.w",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viswr_z as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_viswr_z,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x0400003,
                "viswr.z",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viswr_y as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_viswr_y,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x0800003,
                "viswr.y",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viswr_x as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vis_parenthesis),
            is_float: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new(
                Opcode::r5900ee_viswr_x,
                OpcodeCategory::R5900EE_COP2_VIWR,
                0x1000003,
                "viswr.x",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        };
    }
    table
};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_descriptor_valid() {
        for x in OPCODES {
            x.check_valid_entry();
        }
    }
}
