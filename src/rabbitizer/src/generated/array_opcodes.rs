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
use crate::opcodes::{Opcode, OpcodeDescriptor, OPCODE_COUNT};
use crate::operands::Operand;
pub static OPCODES: [OpcodeDescriptor; OPCODE_COUNT] = {
    let mut table = [OpcodeDescriptor::default(); OPCODE_COUNT];
    {
        table[Opcode::ALL_INVALID as usize] = OpcodeDescriptor {
            is_invalid: true,
            ..OpcodeDescriptor::new("INVALID", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_j as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_label),
            instr_type: InstrType::J,
            is_jump: true,
            is_jump_with_address: true,
            ..OpcodeDescriptor::new("j", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_jal as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_label),
            instr_type: InstrType::J,
            is_jump: true,
            is_jump_with_address: true,
            does_link: true,
            ..OpcodeDescriptor::new("jal", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("beq", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("bne", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("beql", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("bnel", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_blez as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("blez", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_blezl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("blezl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bgtz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("bgtz", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bgtzl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("bgtzl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_addi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new("addi", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_addiu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new("addiu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_slti as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("slti", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sltiu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("sltiu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_andi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            is_unsigned: true,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("andi", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_ori as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            is_unsigned: true,
            modifies_rt: true,
            reads_rs: true,
            can_be_unsigned_lo: true,
            ..OpcodeDescriptor::new("ori", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_xori as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            is_unsigned: true,
            modifies_rt: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("xori", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_daddi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new("daddi", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_daddiu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rt, Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            ..OpcodeDescriptor::new("daddiu", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lui as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate),
            instr_type: InstrType::I,
            is_unsigned: true,
            modifies_rt: true,
            can_be_hi: true,
            ..OpcodeDescriptor::new("lui", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ldl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_LEFT),
            ..OpcodeDescriptor::new("ldl", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ldr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_RIGHT),
            ..OpcodeDescriptor::new("ldr", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lb as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::BYTE),
            ..OpcodeDescriptor::new("lb", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::SHORT),
            ..OpcodeDescriptor::new("lh", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lwl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_WORD_LEFT),
            ..OpcodeDescriptor::new("lwl", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new("lw", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lbu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::BYTE),
            does_unsigned_memory_access: true,
            ..OpcodeDescriptor::new("lbu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lhu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::SHORT),
            does_unsigned_memory_access: true,
            ..OpcodeDescriptor::new("lhu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lwr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::UNALIGNED_WORD_RIGHT),
            ..OpcodeDescriptor::new("lwr", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_lwu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD),
            does_unsigned_memory_access: true,
            ..OpcodeDescriptor::new("lwu", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sb as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::BYTE),
            ..OpcodeDescriptor::new("sb", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::SHORT),
            ..OpcodeDescriptor::new("sh", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_swl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_WORD_LEFT),
            ..OpcodeDescriptor::new("swl", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::WORD),
            ..OpcodeDescriptor::new("sw", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_sdl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_RIGHT),
            ..OpcodeDescriptor::new("sdl", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_sdr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_DOUBLEWORD_LEFT),
            ..OpcodeDescriptor::new("sdr", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_swr as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::UNALIGNED_WORD_RIGHT),
            ..OpcodeDescriptor::new("swr", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ll as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::LINKED_WORD_WORD),
            ..OpcodeDescriptor::new("ll", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_IV")]
    {
        table[Opcode::core_pref as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_hint, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new("pref", IsaVersion::MIPS_IV, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_lld as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::LINKED_WORD_DOUBLEWORD),
            ..OpcodeDescriptor::new("lld", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ld as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::DOUBLEWORD),
            ..OpcodeDescriptor::new("ld", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::LINKED_WORD_WORD),
            ..OpcodeDescriptor::new("sc", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_scd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::LINKED_WORD_DOUBLEWORD),
            ..OpcodeDescriptor::new("scd", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_sd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::DOUBLEWORD),
            ..OpcodeDescriptor::new("sd", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_cache as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_op, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("cache", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lwc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
            instr_type: InstrType::I,
            is_float: true,
            reads_rs: true,
            modifies_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::FLOAT),
            ..OpcodeDescriptor::new("lwc1", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ldc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
            instr_type: InstrType::I,
            is_float: true,
            is_double: true,
            reads_rs: true,
            modifies_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::DOUBLEFLOAT),
            ..OpcodeDescriptor::new("ldc1", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_swc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
            instr_type: InstrType::I,
            is_float: true,
            reads_rs: true,
            reads_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::FLOAT),
            ..OpcodeDescriptor::new("swc1", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sdc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_ft, Operand::core_immediate_base),
            instr_type: InstrType::I,
            is_float: true,
            is_double: true,
            reads_rs: true,
            reads_ft: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::DOUBLEFLOAT),
            ..OpcodeDescriptor::new("sdc1", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_lwc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::WORD_COP2),
            ..OpcodeDescriptor::new("lwc2", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ldc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::DOUBLEWORD_COP2),
            ..OpcodeDescriptor::new("ldc2", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_swc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::WORD_COP2),
            ..OpcodeDescriptor::new("swc2", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sdc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_cop2t, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            not_emitted_by_compilers: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::DOUBLEWORD_COP2),
            ..OpcodeDescriptor::new("sdc2", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_b as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::I,
            is_branch: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new("b", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_beqz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            reads_rs: true,
            is_branch: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new("beqz", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bnez as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::I,
            reads_rs: true,
            is_branch: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new("bnez", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("beqzl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("bnezl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }

    {
        table[Opcode::core_sll as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("sll", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_srl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("srl", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sra as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("sra", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsll as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsll", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsrl", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsra as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsra", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsll32 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsll32", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrl32 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsrl32", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsra32 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsra32", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsllv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsllv", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrlv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsrlv", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dsrav as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("dsrav", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sllv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("sllv", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_srlv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("srlv", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_srav as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("srav", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mthi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            instr_type: InstrType::R,
            reads_rs: true,
            modifies_hi: true,
            ..OpcodeDescriptor::new("mthi", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mtlo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            instr_type: InstrType::R,
            reads_rs: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new("mtlo", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_jr as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            instr_type: InstrType::R,
            is_jump: true,
            jumps_to_register: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("jr", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("jalr", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mfhi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_hi: true,
            ..OpcodeDescriptor::new("mfhi", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mflo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_lo: true,
            ..OpcodeDescriptor::new("mflo", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_IV")]
    {
        table[Opcode::core_movz as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("movz", IsaVersion::MIPS_IV, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_IV")]
    {
        table[Opcode::core_movn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("movn", IsaVersion::MIPS_IV, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_div as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new("div", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_divu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_maybe_zero_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new("divu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("ddiv", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("ddivu", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("add", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_addu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            adds_registers: true,
            ..OpcodeDescriptor::new("addu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("sub", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_subu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            subs_registers: true,
            ..OpcodeDescriptor::new("subu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_and as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ands_registers: true,
            ..OpcodeDescriptor::new("and", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_or as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            ors_registers: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("or", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_xor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("xor", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_nor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("nor", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_slt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("slt", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sltu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("sltu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("dadd", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("daddu", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("dsub", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("dsubu", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_syscall as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_code_lower),
            instr_type: InstrType::R,
            causes_exception: true,
            causes_unconditional_exception: true,
            causes_returnable_exception: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("syscall", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_break as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_code),
            causes_exception: true,
            causes_unconditional_exception: true,
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new("break", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sync as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new("sync", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mult as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new("mult", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_multu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            reads_rs: true,
            reads_rt: true,
            modifies_hi: true,
            modifies_lo: true,
            ..OpcodeDescriptor::new("multu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("dmult", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("dmultu", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("tge", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("tgeu", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("tlt", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("tltu", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("teq", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("tne", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_nop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            is_pseudo: true,
            ..OpcodeDescriptor::new("nop", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_not as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new("not", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_neg as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            not_emitted_by_compilers: true,
            modifies_rd: true,
            reads_rt: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new("neg", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_negu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new("negu", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bltz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("bltz", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bgez as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("bgez", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bltzl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("bltzl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bgezl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            is_branch_likely: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("bgezl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tgei as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tgei", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tgeiu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tgeiu", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tlti as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tlti", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tltiu as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tltiu", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_teqi as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("teqi", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_tnei as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            is_trap: true,
            causes_exception: true,
            causes_conditional_exception: true,
            reads_rs: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tnei", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bltzal as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            does_link: true,
            ..OpcodeDescriptor::new("bltzal", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bgezal as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            reads_rs: true,
            does_link: true,
            ..OpcodeDescriptor::new("bgezal", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("bltzall", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("bgezall", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bal as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::REGIMM,
            is_branch: true,
            not_emitted_by_compilers: true,
            does_link: true,
            is_pseudo: true,
            ..OpcodeDescriptor::new("bal", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("mfc0", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("dmfc0", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0cd),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("cfc0", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mtc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("mtc0", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmtc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0d),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("dmtc0", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_ctc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop0cd),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("ctc0", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }

    {
        table[Opcode::core_bc0f as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new("bc0f", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bc0t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new("bc0t", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc0fl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new("bc0fl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc0tl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new("bc0tl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_tlbr as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tlbr", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_tlbwi as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tlbwi", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_tlbwr as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new("tlbwr", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_tlbp as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("tlbp", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_rfe as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("rfe", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_eret as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            causes_exception: true,
            causes_unconditional_exception: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("eret", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mfc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_rt: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("mfc1", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmfc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_rt: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("dmfc1", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mtc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_rt: true,
            modifies_fs: true,
            ..OpcodeDescriptor::new("mtc1", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_dmtc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_rt: true,
            modifies_fs: true,
            ..OpcodeDescriptor::new("dmtc1", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cfc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop1cs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_rt: true,
            ..OpcodeDescriptor::new("cfc1", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_ctc1 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop1cs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("ctc1", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }

    {
        table[Opcode::core_bc1f as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new("bc1f", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_bc1t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            ..OpcodeDescriptor::new("bc1t", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc1fl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new("bc1fl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_bc1tl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            instr_type: InstrType::UNKNOWN,
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new("bc1tl", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_add_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("add.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sub_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("sub.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mul_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("mul.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_div_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("div.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sqrt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("sqrt.s", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_abs_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("abs.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mov_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("mov.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_neg_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("neg.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_round_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("round.l.s", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_trunc_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("trunc.l.s", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ceil_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("ceil.l.s", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_floor_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("floor.l.s", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_round_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("round.w.s", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_trunc_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("trunc.w.s", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ceil_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("ceil.w.s", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_floor_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("floor.w.s", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cvt_d_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.d.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cvt_w_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.w.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_cvt_l_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.l.s", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_f_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.f.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_un_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.un.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_eq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.eq.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ueq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ueq.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_olt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.olt.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ult_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ult.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ole_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ole.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ule_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ule.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_sf_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.sf.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ngle_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ngle.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_seq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.seq.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ngl_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ngl.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_lt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.lt.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_nge_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.nge.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_le_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.le.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ngt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ngt.s", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_add_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("add.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_sub_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("sub.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mul_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("mul.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_div_d as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("div.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_sqrt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("sqrt.d", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_abs_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("abs.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mov_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("mov.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_neg_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("neg.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_round_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("round.l.d", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_trunc_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("trunc.l.d", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_ceil_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("ceil.l.d", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_floor_l_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("floor.l.d", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_round_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("round.w.d", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_trunc_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("trunc.w.d", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_ceil_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("ceil.w.d", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_II")]
    {
        table[Opcode::core_floor_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("floor.w.d", IsaVersion::MIPS_II, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cvt_s_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.s.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cvt_w_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.w.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("cvt.l.d", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_f_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.f.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_un_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.un.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_eq_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.eq.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ueq_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ueq.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_olt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.olt.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ult_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ult.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ole_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ole.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ule_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ule.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_df_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.df.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ngle_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ngle.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_seq_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.seq.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ngl_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ngl.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_lt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.lt.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_nge_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.nge.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_le_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.le.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_c_ngt_d as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.ngt.d", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cvt_s_w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.s.w", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cvt_d_w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            is_double: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.d.w", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "MIPS_III")]
    {
        table[Opcode::core_cvt_s_l as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fd, Operand::core_fs),
            instr_type: InstrType::UNKNOWN,
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            ..OpcodeDescriptor::new("cvt.s.l", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("cvt.d.l", IsaVersion::MIPS_III, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2d),
            modifies_rt: true,
            ..OpcodeDescriptor::new("mfc2", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_mtc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2d),
            reads_rt: true,
            ..OpcodeDescriptor::new("mtc2", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_cfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2cd),
            modifies_rt: true,
            ..OpcodeDescriptor::new("cfc2", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    {
        table[Opcode::core_ctc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_cop2cd),
            reads_rt: true,
            ..OpcodeDescriptor::new("ctc2", IsaVersion::MIPS_I, None)
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_vs_index),
            modifies_rt: true,
            ..OpcodeDescriptor::new("mfc2", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mtc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_vs_index),
            reads_rt: true,
            ..OpcodeDescriptor::new("mtc2", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_cfc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
            modifies_rt: true,
            ..OpcodeDescriptor::new("cfc2", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ctc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop2cd),
            reads_rt: true,
            ..OpcodeDescriptor::new("ctc2", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmulf as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmulf", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmulu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmulu", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrndp as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vrndp", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmulq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmulq", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmudl", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudm as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmudm", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmudn", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmudh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmudh", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmacf as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmacf", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmacu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmacu", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrndn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vrndn", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmacq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmacq", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmadl", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadm as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmadm", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmadn", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmadh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmadh", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vadd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vadd", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vsub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsub", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsut as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsut", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vabs as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vabs", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vaddc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vaddc", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vsubc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsubc", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vaddb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vaddb", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsubb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsubb", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vaccb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vaccb", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsucb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsucb", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsad as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsad", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsac as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsac", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsum as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsum", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vsar as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsar", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vacc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vacc", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vsuc as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vsuc", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vlt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vlt", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_veq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("veq", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vne as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vne", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vge as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vge", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vcl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vcl", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vch as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vch", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vcr as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vcr", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmrg as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vmrg", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vand", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vnand", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vor", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vnor", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vxor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vxor", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnxor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vnxor", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_v056 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("v056", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_v057 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("v057", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrcp as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new("vrcp", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrcpl as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new("vrcpl", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrcph as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new("vrcph", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vmov as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new("vmov", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrsq as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new("vrsq", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrsql as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new("vrsql", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vrsqh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
            ..OpcodeDescriptor::new("vrsqh", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_vnop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("vnop", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vextt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vextt", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vextq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vextq", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vextn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vextn", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_v073 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("v073", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vinst as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vinst", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vinsq as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vinsq", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vinsn as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::rsp_vd,
                Operand::rsp_vs,
                Operand::rsp_vt_elementhigh,
            ),
            ..OpcodeDescriptor::new("vinsn", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_vnull as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("vnull", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lbv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lbv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lsv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lsv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_llv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("llv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ldv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("ldv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lqv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lqv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lrv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lrv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lpv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lpv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_luv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("luv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lhv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lhv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_lfv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lfv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RspViceMsp")]
    {
        table[Opcode::rsp_lwv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("lwv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ltv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("ltv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sbv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("sbv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_ssv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("ssv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_slv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("slv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sdv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("sdv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sqv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("sqv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_srv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("srv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_spv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("spv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_suv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("suv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_shv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("shv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_sfv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("sfv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_swv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("swv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_stv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("stv", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mfc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
            instr_type: InstrType::UNKNOWN,
            modifies_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("mfc0", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "RSP")]
    {
        table[Opcode::rsp_mtc0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::rsp_cop0d),
            instr_type: InstrType::UNKNOWN,
            reads_rt: true,
            not_emitted_by_compilers: true,
            ..OpcodeDescriptor::new("mtc0", IsaVersion::EXTENSION, Some(IsaExtension::RSP))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_RTPS as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("RTPS", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_RTPT as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("RTPT", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_DPCL as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("DPCL", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_DPCS as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("DPCS", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_DPCT as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("DPCT", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_INTPL as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("INTPL", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_NCS as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("NCS", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_NCT as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("NCT", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_NCDS as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("NCDS", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_NCDT as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("NCDT", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_NCCS as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("NCCS", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_NCCT as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("NCCT", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_CDP as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("CDP", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_CC as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("CC", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_NCLIP as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("NCLIP", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_AVSZ3 as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("AVSZ3", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_AVSZ4 as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("AVSZ4", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_MVMVA as usize] = OpcodeDescriptor {
            operands: Operand::arr5(
                Operand::r3000gte_sf,
                Operand::r3000gte_mx,
                Operand::r3000gte_v,
                Operand::r3000gte_cv,
                Operand::r3000gte_lm,
            ),
            ..OpcodeDescriptor::new("MVMVA", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_SQR as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new("SQR", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_OP as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new("OP", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_GPF as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new("GPF", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R3000GTE")]
    {
        table[Opcode::r3000gte_GPL as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r3000gte_sf),
            ..OpcodeDescriptor::new("GPL", IsaVersion::EXTENSION, Some(IsaExtension::R3000GTE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_lv_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_s_vt_imm,
                Operand::r4000allegrex_offset14_base,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "lv.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_sv_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_s_vt_imm,
                Operand::r4000allegrex_offset14_base,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "sv.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_lv_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_base,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "lv.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_sv_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_base_maybe_wb,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "sv.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_clz as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "clz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_clo as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "clo",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "madd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "maddu",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "msub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "msubu",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "movz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "movn",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "max",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "min",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_srl as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "srl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_rotr as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "rotr",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "srlv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "rotrv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_sleep as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "sleep",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mfie as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                "mfie",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mtie as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rt),
            instr_type: InstrType::R,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "mtie",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "ext",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "ins",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_wsbh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "wsbh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_wsbw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "wsbw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_seb as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "seb",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_seh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "seh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_bitrev as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "bitrev",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "bvf",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "bvt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "bvfl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "bvtl",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mfv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                "mfv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mfvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_cop2cd),
            instr_type: InstrType::R,
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                "mfvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsync2 as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsync2",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mtv as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "mtv",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_mtvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r4000allegrex_cop2cd),
            instr_type: InstrType::R,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "mtvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vadd.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vadd.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vadd.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vadd.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsub.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsub.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsub.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsub.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsbn.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdiv.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdiv.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdiv.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdiv.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmul.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmul.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmul.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmul.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdot.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdot.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdot.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vscl.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vscl.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vscl.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vhdp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vhdp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vhdp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcrs.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vdet.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcmp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcmp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcmp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcmp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcmp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmax.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmax.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmax.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmax.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vscmp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vscmp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vscmp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vscmp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsge.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsge.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsge.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vsge.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vslt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vslt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vslt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vslt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vwbn.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmov.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmov.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmov.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmov_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmov.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vabs.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vabs.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vabs.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vabs_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vabs.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vneg.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vneg.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vneg.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vneg_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vneg.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vidt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vidt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vidt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vidt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat0.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat0.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat0.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat0_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat0.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat1.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat1.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat1.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsat1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsat1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vzero.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vzero.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vzero.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vzero_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vzero.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vone.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vone.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vone.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vone_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vone.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrcp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrcp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrcp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrcp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrcp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrsq.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrsq.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrsq.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrsq_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrsq.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsin_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcos.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcos.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcos.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vcos_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vcos.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vexp2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vexp2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vexp2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vexp2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vexp2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vlog2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vlog2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vlog2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlog2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vlog2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsqrt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsqrt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsqrt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsqrt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsqrt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vasin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vasin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vasin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vasin_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vasin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnrcp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnrcp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnrcp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnrcp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnrcp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnsin.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnsin.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnsin.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnsin_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vnsin.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrexp2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrexp2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrexp2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrexp2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrexp2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrnds_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrnds.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndi.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndi.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndi.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndi_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndi.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf1.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf1.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf1.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_s_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf2.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_p_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_t_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf2.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vrndf2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_q_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vrndf2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2h_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vf2h.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vf2h_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vf2h.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vh2f_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vh2f.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vh2f_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vh2f.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsbz_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsbz.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vlgb_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vlgb.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vuc2ifs_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vuc2ifs.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vc2i_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vc2i.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vus2i_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vus2i.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vus2i_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vus2i.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vs2i_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vs2i.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vs2i_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vs2i.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2uc_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vi2uc.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2c_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vi2c.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2us_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vi2us.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2us_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vi2us.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2s_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vi2s.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vi2s_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vi2s.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsrt1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsrt2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vbfy1_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vbfy1.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vbfy1_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vbfy1.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vbfy2_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vbfy2.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vocp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vocp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vocp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vocp_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vocp.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsocp_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsocp.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsocp_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsocp.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfad_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vfad.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfad_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vfad.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfad_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vfad.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vavg_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vavg.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vavg_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vavg.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vavg_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vavg.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt3_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsrt3.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsrt4_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsrt4.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsgn.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsgn.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsgn.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsgn_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vsgn.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmfvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_cop2cs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmfvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmtvc as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_cop2cd, Operand::r4000allegrex_s_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmtvc",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vt4444_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vt4444.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vt5551_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vt5551.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vt5650_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vt5650.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcst.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcst.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcst.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcst.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2in.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2in.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2in.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2in.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iz.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iz.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iz.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iz.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iu.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iu.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iu.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2iu.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2id.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2id.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2id.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vf2id.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vi2f.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vi2f.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vi2f.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vi2f.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovf.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovf.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovf.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcmovf.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vpfxs",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vpfxt",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vpfxd",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_viim_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_int16),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "viim.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vfim_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_float16),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vfim.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmmul.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmmul.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmmul.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vhtfm2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vtfm2.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vhtfm3.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vtfm3.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vhtfm4.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vtfm4.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmscl.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmscl.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vmscl.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vcrsp.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vqmul.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vrot.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vrot.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
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
                "vrot.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmov_p as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_mp_vd, Operand::r4000allegrex_mp_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmmov.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmov_t as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_mt_vd, Operand::r4000allegrex_mt_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmmov.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmmov_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r4000allegrex_mq_vd, Operand::r4000allegrex_mq_vs),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmmov.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmidt_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmidt.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmidt_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmidt.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmidt_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmidt.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmzero_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmzero.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmzero_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmzero.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmzero_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmzero.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmone_p as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmone.p",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmone_t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmone.t",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vmone_q as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new(
                "vmone.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vnop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                "vnop",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vsync as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                "vsync",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_vflush as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::UNKNOWN,
            ..OpcodeDescriptor::new(
                "vflush",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_svl_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_base,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "svl.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R4000ALLEGREX")]
    {
        table[Opcode::r4000allegrex_svr_q as usize] = OpcodeDescriptor {
            operands: Operand::arr2(
                Operand::r4000allegrex_q_vt_imm,
                Operand::r4000allegrex_offset14_base,
            ),
            instr_type: InstrType::I,
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "svr.q",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R4000ALLEGREX),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_lq as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            modifies_rt: true,
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new("lq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_sq as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::core_immediate_base),
            instr_type: InstrType::I,
            reads_rs: true,
            reads_rt: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new("sq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_lqc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vft, Operand::core_immediate_base),
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_load: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new("lqc2", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_sqc2 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vft, Operand::core_immediate_base),
            reads_rs: true,
            can_be_lo: true,
            does_dereference: true,
            does_store: true,
            access_type: Some(AccessType::QUADWORD),
            ..OpcodeDescriptor::new("sqc2", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_sync_p as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            instr_type: InstrType::R,
            ..OpcodeDescriptor::new("sync.p", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mult as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            instr_type: InstrType::R,
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("mult", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mfsa as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new("mfsa", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtsa as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("mtsa", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtsab as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            reads_rs: true,
            ..OpcodeDescriptor::new("mtsab", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtsah as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_immediate),
            instr_type: InstrType::REGIMM,
            reads_rs: true,
            ..OpcodeDescriptor::new("mtsah", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("madd", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_maddu as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("maddu", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_plzcw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            ..OpcodeDescriptor::new("plzcw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mfhi1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new("mfhi1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mthi1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("mthi1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mflo1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            reads_rd: true,
            ..OpcodeDescriptor::new("mflo1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mtlo1 as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("mtlo1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mult1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("mult1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_multu1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("multu1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_div1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_zero, Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("div1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_divu1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_zero, Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("divu1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madd1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("madd1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_maddu1 as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("maddu1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psllh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psllh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrlh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psrlh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrah as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psrah", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psllw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psllw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrlw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psrlw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psraw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_sa),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psraw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("paddw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcgtw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pcgtw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaxw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmaxw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("paddh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcgth as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pcgth", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaxh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmaxh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("paddb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcgtb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pcgtb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddsw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("paddsw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubsw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubsw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextlw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pextlw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppacw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("ppacw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddsh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("paddsh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubsh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubsh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextlh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pextlh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppach as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("ppach", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddsb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("paddsb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubsb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubsb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextlb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pextlb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppacb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("ppacb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pext5 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pext5", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ppac5 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("ppac5", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pabsw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pabsw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pceqw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pceqw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pminw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pminw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_padsbh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("padsbh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pabsh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pabsh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pceqh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pceqh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pminh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pminh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pceqb as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pceqb", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_padduw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("padduw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubuw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubuw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextuw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pextuw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_padduh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("padduh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubuh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubuh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextuh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pextuh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_paddub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            adds_registers: true,
            ..OpcodeDescriptor::new("paddub", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psubub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psubub", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pextub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pextub", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qfsrv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("qfsrv", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaddw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmaddw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psllvw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psllvw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psrlvw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psrlvw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmsubw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmsubw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new("pmfhi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmflo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new("pmflo", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pinth as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pinth", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmultw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmultw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pdivw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pdivw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcpyld as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pcpyld", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmaddh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmaddh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_phmadh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("phmadh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pand", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pxor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pxor", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmsubh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmsubh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_phmsbh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("phmsbh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexeh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pexeh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_prevh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("prevh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmulth as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pmulth", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pdivbw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pdivbw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexew as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pexew", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_prot3w as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("prot3w", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmadduw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "pmadduw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_psravw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rt, Operand::core_rs),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("psravw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmthi as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("pmthi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmtlo as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new("pmtlo", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pinteh as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pinteh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmultuw as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "pmultuw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pdivuw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rs, Operand::core_rt),
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pdivuw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcpyud as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pcpyud", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_por as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("por", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pnor as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_rd, Operand::core_rs, Operand::core_rt),
            modifies_rd: true,
            reads_rs: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pnor", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexch as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pexch", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pcpyh as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pcpyh", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pexcw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rd, Operand::core_rt),
            modifies_rd: true,
            reads_rt: true,
            ..OpcodeDescriptor::new("pexcw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_lw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                "pmfhl.lw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_uw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                "pmfhl.uw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_slw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                "pmfhl.slw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_lh as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                "pmfhl.lh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmfhl_sh as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rd),
            modifies_rd: true,
            ..OpcodeDescriptor::new(
                "pmfhl.sh",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_pmthl_lw as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_rs),
            reads_rs: true,
            ..OpcodeDescriptor::new(
                "pmthl.lw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ei as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("ei", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_di as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("di", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_c1__sqrt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_copraw),
            ..OpcodeDescriptor::new("c1", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "rsqrt.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_adda_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("adda.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_suba_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("suba.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_mula_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("mula.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madd_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("madd.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_msub_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("msub.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_madda_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                "madda.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_msuba_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new(
                "msuba.s",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_max_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("max.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_min_s as usize] = OpcodeDescriptor {
            operands: Operand::arr3(Operand::core_fd, Operand::core_fs, Operand::core_ft),
            is_float: true,
            modifies_fd: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("min.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_c_lt_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.lt.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_c_le_s as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_fs, Operand::core_ft),
            is_float: true,
            reads_fs: true,
            reads_ft: true,
            ..OpcodeDescriptor::new("c.le.s", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmfc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                "qmfc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_cfc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                "cfc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmtc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "qmtc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ctc2_ni as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "ctc2.ni",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmfc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            modifies_rt: true,
            ..OpcodeDescriptor::new(
                "qmfc2.i",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_cfc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            modifies_rt: true,
            ..OpcodeDescriptor::new("cfc2.i", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_qmtc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vfs),
            reads_rt: true,
            ..OpcodeDescriptor::new(
                "qmtc2.i",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_ctc2_i as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::core_rt, Operand::r5900ee_vis),
            reads_rt: true,
            ..OpcodeDescriptor::new("ctc2.i", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }

    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2f as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            ..OpcodeDescriptor::new("bc2f", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2t as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            ..OpcodeDescriptor::new("bc2t", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2fl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new("bc2fl", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_bc2tl as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::core_branch_target_label),
            is_branch: true,
            is_branch_likely: true,
            ..OpcodeDescriptor::new("bc2tl", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddx", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddy", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubx", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsuby", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaddx", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaddy", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaddz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaddw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsubx", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsuby", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsubz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsubw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaxx", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaxy", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaxz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaxw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vminix", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vminiy", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vminiz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vminiw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulx", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmuly", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaxi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmuli", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vminii", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaddq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmaddi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsubq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsubi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vadd", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmadd", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmul", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmax", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsub", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsub", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vopmsub",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmini", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viadd as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new("viadd", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_visub as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new("visub", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viaddi as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vit,
                Operand::r5900ee_vis,
                Operand::r5900ee_immediate5,
            ),
            ..OpcodeDescriptor::new("viaddi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_viand as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new("viand", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vior as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_vid,
                Operand::r5900ee_vis,
                Operand::r5900ee_vit,
            ),
            ..OpcodeDescriptor::new("vior", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vcallms as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r5900ee_immediate15),
            ..OpcodeDescriptor::new(
                "vcallms",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vcallmsr as usize] = OpcodeDescriptor {
            operands: Operand::arr1(Operand::r5900ee_vis),
            ..OpcodeDescriptor::new(
                "vcallmsr",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddax", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vadday", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddaz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddaw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubax", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubay", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubaz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubaw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vmaddax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vmadday",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vmaddaz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vmaddaw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vmsubax",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vmsubay",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vmsubaz",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vmsubaw",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vitof0", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof4 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vitof4", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof12 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                "vitof12",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vitof15 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                "vitof15",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi0 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vftoi0", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi4 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vftoi4", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi12 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                "vftoi12",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vftoi15 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new(
                "vftoi15",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulax", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulay", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulaz", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulaw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulaq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vabs as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vabs", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmulai", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vclipw as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vftn),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vclipw", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddaq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vmaddaq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vaddai", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vmaddai",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubaq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vmsubaq",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsubai", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vmsubai",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vadda", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmadda", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmula", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vsuba", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
            ..OpcodeDescriptor::new("vmsuba", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vopmula",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vnop as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("vnop", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmove as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vmove", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmr32 as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vfsxyzw),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vmr32", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vlqi as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis_postincr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vlqi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsqi as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vit_postincr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vsqi", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vlqd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis_predecr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vlqd", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsqd as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vfsxyzw, Operand::r5900ee_vit_predecr),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vsqd", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vdiv as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_Q,
                Operand::r5900ee_vfsl,
                Operand::r5900ee_vftm,
            ),
            ..OpcodeDescriptor::new("vdiv", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vsqrt as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_Q, Operand::r5900ee_vftm),
            ..OpcodeDescriptor::new("vsqrt", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrsqrt as usize] = OpcodeDescriptor {
            operands: Operand::arr3(
                Operand::r5900ee_Q,
                Operand::r5900ee_vfsl,
                Operand::r5900ee_vftm,
            ),
            ..OpcodeDescriptor::new("vrsqrt", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vwaitq as usize] = OpcodeDescriptor {
            operands: Operand::arr0(),
            ..OpcodeDescriptor::new("vwaitq", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmtir as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vit, Operand::r5900ee_vfsl),
            ..OpcodeDescriptor::new("vmtir", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vmfir as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_vis),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vmfir", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrnext as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_R),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vrnext", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrget as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_vftxyzw, Operand::r5900ee_R),
            instr_suffix: Some(InstrSuffix::R5900EE_xyzw),
            is_float: true,
            ..OpcodeDescriptor::new("vrget", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrinit as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_R, Operand::r5900ee_vfsl),
            ..OpcodeDescriptor::new("vrinit", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
    }
    #[cfg(feature = "R5900EE")]
    {
        table[Opcode::r5900ee_vrxor as usize] = OpcodeDescriptor {
            operands: Operand::arr2(Operand::r5900ee_R, Operand::r5900ee_vfsl),
            ..OpcodeDescriptor::new("vrxor", IsaVersion::EXTENSION, Some(IsaExtension::R5900EE))
        }
        .check_panic_chain();
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
                "vilwr.w",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vilwr.z",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vilwr.y",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "vilwr.x",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "viswr.w",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "viswr.z",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "viswr.y",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
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
                "viswr.x",
                IsaVersion::EXTENSION,
                Some(IsaExtension::R5900EE),
            )
        }
        .check_panic_chain();
    }
    let mut i = 0;
    while i < OPCODE_COUNT {
        table[i].check_panic();
        i += 1;
    }
    table
};
