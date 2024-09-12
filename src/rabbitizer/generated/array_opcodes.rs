/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{opcode, AccessType, InstrSuffix, InstrType, Opcode, OpcodeDescriptor, Operand};
pub static OPCODES: [OpcodeDescriptor; opcode::OPCODE_COUNT] = {
    let mut table = [OpcodeDescriptor::new(""); opcode::OPCODE_COUNT as usize];
    table[Opcode::cpu_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        ..OpcodeDescriptor::new("INVALID")
    }
    .check_panic_chain();
    table[Opcode::cpu_j as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        ..OpcodeDescriptor::new("j")
    }
    .check_panic_chain();
    table[Opcode::cpu_jal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        does_link: true,
        ..OpcodeDescriptor::new("jal")
    }
    .check_panic_chain();
    table[Opcode::cpu_beq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("beq")
    }
    .check_panic_chain();
    table[Opcode::cpu_bne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("bne")
    }
    .check_panic_chain();
    table[Opcode::cpu_beql as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("beql")
    }
    .check_panic_chain();
    table[Opcode::cpu_bnel as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::cpu_rs,
            Operand::cpu_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("bnel")
    }
    .check_panic_chain();
    table[Opcode::cpu_blez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("blez")
    }
    .check_panic_chain();
    table[Opcode::cpu_blezl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("blezl")
    }
    .check_panic_chain();
    table[Opcode::cpu_bgtz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgtz")
    }
    .check_panic_chain();
    table[Opcode::cpu_bgtzl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgtzl")
    }
    .check_panic_chain();
    table[Opcode::cpu_addi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addi")
    }
    .check_panic_chain();
    table[Opcode::cpu_addiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addiu")
    }
    .check_panic_chain();
    table[Opcode::cpu_slti as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("slti")
    }
    .check_panic_chain();
    table[Opcode::cpu_sltiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("sltiu")
    }
    .check_panic_chain();
    table[Opcode::cpu_andi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("andi")
    }
    .check_panic_chain();
    table[Opcode::cpu_ori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("ori")
    }
    .check_panic_chain();
    table[Opcode::cpu_xori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("xori")
    }
    .check_panic_chain();
    table[Opcode::cpu_daddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("daddi")
    }
    .check_panic_chain();
    table[Opcode::cpu_daddiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rt, Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("daddiu")
    }
    .check_panic_chain();
    table[Opcode::cpu_lui as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        can_be_hi: true,
        ..OpcodeDescriptor::new("lui")
    }
    .check_panic_chain();
    table[Opcode::cpu_ldl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ldl")
    }
    .check_panic_chain();
    table[Opcode::cpu_ldr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ldr")
    }
    .check_panic_chain();
    table[Opcode::cpu_lb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("lb")
    }
    .check_panic_chain();
    table[Opcode::cpu_lh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("lh")
    }
    .check_panic_chain();
    table[Opcode::cpu_lwl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lwl")
    }
    .check_panic_chain();
    table[Opcode::cpu_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("lw")
    }
    .check_panic_chain();
    table[Opcode::cpu_lbu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lbu")
    }
    .check_panic_chain();
    table[Opcode::cpu_lhu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lhu")
    }
    .check_panic_chain();
    table[Opcode::cpu_lwr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lwr")
    }
    .check_panic_chain();
    table[Opcode::cpu_lwu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lwu")
    }
    .check_panic_chain();
    table[Opcode::cpu_sb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("sb")
    }
    .check_panic_chain();
    table[Opcode::cpu_sh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("sh")
    }
    .check_panic_chain();
    table[Opcode::cpu_swl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("swl")
    }
    .check_panic_chain();
    table[Opcode::cpu_sw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("sw")
    }
    .check_panic_chain();
    table[Opcode::cpu_sdl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sdl")
    }
    .check_panic_chain();
    table[Opcode::cpu_sdr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sdr")
    }
    .check_panic_chain();
    table[Opcode::cpu_swr as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("swr")
    }
    .check_panic_chain();
    table[Opcode::cpu_ll as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ll")
    }
    .check_panic_chain();
    table[Opcode::cpu_pref as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_hint, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("pref")
    }
    .check_panic_chain();
    table[Opcode::cpu_lld as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lld")
    }
    .check_panic_chain();
    table[Opcode::cpu_ld as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::DOUBLEWORD,
        ..OpcodeDescriptor::new("ld")
    }
    .check_panic_chain();
    table[Opcode::cpu_sc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sc")
    }
    .check_panic_chain();
    table[Opcode::cpu_scd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("scd")
    }
    .check_panic_chain();
    table[Opcode::cpu_sd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::DOUBLEWORD,
        ..OpcodeDescriptor::new("sd")
    }
    .check_panic_chain();
    table[Opcode::cpu_cache as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_op, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("cache")
    }
    .check_panic_chain();
    table[Opcode::cpu_lwc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::FLOAT,
        ..OpcodeDescriptor::new("lwc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_ldc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        modifies_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::DOUBLEFLOAT,
        ..OpcodeDescriptor::new("ldc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_swc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::FLOAT,
        ..OpcodeDescriptor::new("swc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_sdc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_ft, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        is_float: true,
        is_double: true,
        reads_rs: true,
        reads_ft: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::DOUBLEFLOAT,
        ..OpcodeDescriptor::new("sdc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_lwc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lwc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_ldc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("ldc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_swc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("swc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_sdc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_cop2t, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sdc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_b as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        is_branch: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("b")
    }
    .check_panic_chain();
    table[Opcode::cpu_beqz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("beqz")
    }
    .check_panic_chain();
    table[Opcode::cpu_bnez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::I,
        reads_rs: true,
        is_branch: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("bnez")
    }
    .check_panic_chain();
    table[Opcode::cpu_sll as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sll")
    }
    .check_panic_chain();
    table[Opcode::cpu_srl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srl")
    }
    .check_panic_chain();
    table[Opcode::cpu_sra as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sra")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsll as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsll")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrl")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsra as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsra")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsll32 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsll32")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrl32 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrl32")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsra32 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsra32")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsllv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsllv")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrlv")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsrav as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsrav")
    }
    .check_panic_chain();
    table[Opcode::cpu_sllv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sllv")
    }
    .check_panic_chain();
    table[Opcode::cpu_srlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srlv")
    }
    .check_panic_chain();
    table[Opcode::cpu_srav as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srav")
    }
    .check_panic_chain();
    table[Opcode::cpu_mthi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_hi: true,
        ..OpcodeDescriptor::new("mthi")
    }
    .check_panic_chain();
    table[Opcode::cpu_mtlo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("mtlo")
    }
    .check_panic_chain();
    table[Opcode::cpu_jr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true,
        ..OpcodeDescriptor::new("jr")
    }
    .check_panic_chain();
    table[Opcode::cpu_jalr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("jalr")
    }
    .check_panic_chain();
    table[Opcode::cpu_mfhi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_hi: true,
        ..OpcodeDescriptor::new("mfhi")
    }
    .check_panic_chain();
    table[Opcode::cpu_mflo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_lo: true,
        ..OpcodeDescriptor::new("mflo")
    }
    .check_panic_chain();
    table[Opcode::cpu_movz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movz")
    }
    .check_panic_chain();
    table[Opcode::cpu_movn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movn")
    }
    .check_panic_chain();
    table[Opcode::cpu_div as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("div")
    }
    .check_panic_chain();
    table[Opcode::cpu_divu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("divu")
    }
    .check_panic_chain();
    table[Opcode::cpu_sn64_div as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("sn64_div")
    }
    .check_panic_chain();
    table[Opcode::cpu_sn64_divu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("sn64_divu")
    }
    .check_panic_chain();
    table[Opcode::cpu_ddiv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("ddiv")
    }
    .check_panic_chain();
    table[Opcode::cpu_ddivu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        reads_rd: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("ddivu")
    }
    .check_panic_chain();
    table[Opcode::cpu_add as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("add")
    }
    .check_panic_chain();
    table[Opcode::cpu_addu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true,
        ..OpcodeDescriptor::new("addu")
    }
    .check_panic_chain();
    table[Opcode::cpu_sub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sub")
    }
    .check_panic_chain();
    table[Opcode::cpu_subu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("subu")
    }
    .check_panic_chain();
    table[Opcode::cpu_and as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("and")
    }
    .check_panic_chain();
    table[Opcode::cpu_or as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        maybe_is_move: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("or")
    }
    .check_panic_chain();
    table[Opcode::cpu_xor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("xor")
    }
    .check_panic_chain();
    table[Opcode::cpu_nor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("nor")
    }
    .check_panic_chain();
    table[Opcode::cpu_slt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("slt")
    }
    .check_panic_chain();
    table[Opcode::cpu_sltu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sltu")
    }
    .check_panic_chain();
    table[Opcode::cpu_dadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dadd")
    }
    .check_panic_chain();
    table[Opcode::cpu_daddu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true,
        ..OpcodeDescriptor::new("daddu")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsub")
    }
    .check_panic_chain();
    table[Opcode::cpu_dsubu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("dsubu")
    }
    .check_panic_chain();
    table[Opcode::cpu_syscall as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_code_lower),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("syscall")
    }
    .check_panic_chain();
    table[Opcode::cpu_break as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_code),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("break")
    }
    .check_panic_chain();
    table[Opcode::cpu_sync as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("sync")
    }
    .check_panic_chain();
    table[Opcode::cpu_mult as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("mult")
    }
    .check_panic_chain();
    table[Opcode::cpu_multu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("multu")
    }
    .check_panic_chain();
    table[Opcode::cpu_dmult as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("dmult")
    }
    .check_panic_chain();
    table[Opcode::cpu_dmultu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("dmultu")
    }
    .check_panic_chain();
    table[Opcode::cpu_tge as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tge")
    }
    .check_panic_chain();
    table[Opcode::cpu_tgeu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tgeu")
    }
    .check_panic_chain();
    table[Opcode::cpu_tlt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlt")
    }
    .check_panic_chain();
    table[Opcode::cpu_tltu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tltu")
    }
    .check_panic_chain();
    table[Opcode::cpu_teq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("teq")
    }
    .check_panic_chain();
    table[Opcode::cpu_tne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_code_lower),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        is_trap: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tne")
    }
    .check_panic_chain();
    table[Opcode::cpu_nop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        is_pseudo: true,
        ..OpcodeDescriptor::new("nop")
    }
    .check_panic_chain();
    table[Opcode::cpu_move as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("move")
    }
    .check_panic_chain();
    table[Opcode::cpu_not as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("not")
    }
    .check_panic_chain();
    table[Opcode::cpu_neg as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("neg")
    }
    .check_panic_chain();
    table[Opcode::cpu_negu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("negu")
    }
    .check_panic_chain();
    table[Opcode::cpu_bltz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bltz")
    }
    .check_panic_chain();
    table[Opcode::cpu_bgez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgez")
    }
    .check_panic_chain();
    table[Opcode::cpu_bltzl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bltzl")
    }
    .check_panic_chain();
    table[Opcode::cpu_bgezl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("bgezl")
    }
    .check_panic_chain();
    table[Opcode::cpu_tgei as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tgei")
    }
    .check_panic_chain();
    table[Opcode::cpu_tgeiu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tgeiu")
    }
    .check_panic_chain();
    table[Opcode::cpu_tlti as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlti")
    }
    .check_panic_chain();
    table[Opcode::cpu_tltiu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tltiu")
    }
    .check_panic_chain();
    table[Opcode::cpu_teqi as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("teqi")
    }
    .check_panic_chain();
    table[Opcode::cpu_tnei as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        is_trap: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tnei")
    }
    .check_panic_chain();
    table[Opcode::cpu_bltzal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("bltzal")
    }
    .check_panic_chain();
    table[Opcode::cpu_bgezal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("bgezal")
    }
    .check_panic_chain();
    table[Opcode::cpu_bltzall as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("bltzall")
    }
    .check_panic_chain();
    table[Opcode::cpu_bgezall as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        is_branch_likely: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        does_link: true,
        ..OpcodeDescriptor::new("bgezall")
    }
    .check_panic_chain();
    table[Opcode::cpu_bal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("bal")
    }
    .check_panic_chain();
    table[Opcode::cpu_mfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mfc0")
    }
    .check_panic_chain();
    table[Opcode::cpu_dmfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("dmfc0")
    }
    .check_panic_chain();
    table[Opcode::cpu_cfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("cfc0")
    }
    .check_panic_chain();
    table[Opcode::cpu_mtc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mtc0")
    }
    .check_panic_chain();
    table[Opcode::cpu_dmtc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("dmtc0")
    }
    .check_panic_chain();
    table[Opcode::cpu_ctc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("ctc0")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0f as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc0f")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc0t")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0fl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc0fl")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc0tl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc0tl")
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbr as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlbr")
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbwi as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlbwi")
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbwr as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("tlbwr")
    }
    .check_panic_chain();
    table[Opcode::cpu_tlbp as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("tlbp")
    }
    .check_panic_chain();
    table[Opcode::cpu_rfe as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("rfe")
    }
    .check_panic_chain();
    table[Opcode::cpu_eret as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("eret")
    }
    .check_panic_chain();
    table[Opcode::cpu_mfc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("mfc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_dmfc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("dmfc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_mtc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true,
        ..OpcodeDescriptor::new("mtc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_dmtc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        modifies_fs: true,
        ..OpcodeDescriptor::new("dmtc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_cfc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_ctc1 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop1cs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc1")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1f as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc1f")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bc1t")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1fl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc1fl")
    }
    .check_panic_chain();
    table[Opcode::cpu_bc1tl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc1tl")
    }
    .check_panic_chain();
    table[Opcode::cpu_add_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("add_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_sub_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("sub_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_mul_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("mul_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_div_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("div_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_sqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("sqrt_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_abs_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("abs_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_mov_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("mov_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_neg_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("neg_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_round_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_l_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_l_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_l_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_l_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_round_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_w_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_w_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_w_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_w_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_d_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_d_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_w_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_w_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_l_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_l_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_f_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_f_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_un_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_un_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_eq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_eq_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ueq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ueq_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_olt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_olt_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ult_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ult_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ole_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ole_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ule_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ule_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_sf_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_sf_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngle_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngle_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_seq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_seq_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngl_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngl_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_lt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_lt_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_nge_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_nge_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_le_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_le_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngt_s")
    }
    .check_panic_chain();
    table[Opcode::cpu_add_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("add_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_sub_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("sub_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_mul_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("mul_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_div_d as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("div_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_sqrt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("sqrt_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_abs_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("abs_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_mov_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("mov_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_neg_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("neg_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_round_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_l_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_l_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_l_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_l_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_round_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("round_w_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_trunc_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("trunc_w_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_ceil_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("ceil_w_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_floor_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("floor_w_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_s_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_s_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_w_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_w_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_l_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_l_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_f_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_f_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_un_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_un_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_eq_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_eq_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ueq_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ueq_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_olt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_olt_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ult_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ult_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ole_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ole_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ule_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ule_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_df_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_df_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngle_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngle_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_seq_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_seq_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngl_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngl_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_lt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_lt_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_nge_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_nge_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_le_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_le_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_c_ngt_d as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_ngt_d")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_s_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_s_w")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_d_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_d_w")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_s_l as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_s_l")
    }
    .check_panic_chain();
    table[Opcode::cpu_cvt_d_l as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fd, Operand::cpu_fs),
        instr_type: InstrType::UNKNOWN,
        is_float: true,
        is_double: true,
        modifies_fd: true,
        reads_fs: true,
        ..OpcodeDescriptor::new("cvt_d_l")
    }
    .check_panic_chain();
    table[Opcode::cpu_mfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_mtc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        reads_rt: true,
        ..OpcodeDescriptor::new("mtc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_cfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_ctc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_cop2cd),
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc2")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18")
    }
    .check_panic_chain();
    table[Opcode::cpu_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19")
    }
    .check_panic_chain();
    table[Opcode::cpu_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX")
    }
    .check_panic_chain();
    table[Opcode::rsp_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("INVALID")
    }
    .check_panic_chain();
    table[Opcode::rsp_mfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_cop2t, Operand::rsp_vs_index),
        ..OpcodeDescriptor::new("mfc2")
    }
    .check_panic_chain();
    table[Opcode::rsp_mtc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_cop2t, Operand::rsp_vs_index),
        ..OpcodeDescriptor::new("mtc2")
    }
    .check_panic_chain();
    table[Opcode::rsp_cfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::rsp_cop2cd),
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc2")
    }
    .check_panic_chain();
    table[Opcode::rsp_ctc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::rsp_cop2cd),
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc2")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmulf as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmulf")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmulu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmulu")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrndp as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vrndp")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmulq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmulq")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudl")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudm as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudm")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudn")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmudh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmudh")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmacf as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmacf")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmacu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmacu")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrndn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vrndn")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmacq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmacq")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadl")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadm as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadm")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadn")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmadh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmadh")
    }
    .check_panic_chain();
    table[Opcode::rsp_vadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vadd")
    }
    .check_panic_chain();
    table[Opcode::rsp_vsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vsub")
    }
    .check_panic_chain();
    table[Opcode::rsp_vabs as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vabs")
    }
    .check_panic_chain();
    table[Opcode::rsp_vaddc as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vaddc")
    }
    .check_panic_chain();
    table[Opcode::rsp_vsubc as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vsubc")
    }
    .check_panic_chain();
    table[Opcode::rsp_vsar as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vsar")
    }
    .check_panic_chain();
    table[Opcode::rsp_vand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vand")
    }
    .check_panic_chain();
    table[Opcode::rsp_vnand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vnand")
    }
    .check_panic_chain();
    table[Opcode::rsp_vor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vor")
    }
    .check_panic_chain();
    table[Opcode::rsp_vnor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vnor")
    }
    .check_panic_chain();
    table[Opcode::rsp_vxor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vxor")
    }
    .check_panic_chain();
    table[Opcode::rsp_vnxor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vnxor")
    }
    .check_panic_chain();
    table[Opcode::rsp_vlt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vlt")
    }
    .check_panic_chain();
    table[Opcode::rsp_veq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("veq")
    }
    .check_panic_chain();
    table[Opcode::rsp_vne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vne")
    }
    .check_panic_chain();
    table[Opcode::rsp_vge as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vge")
    }
    .check_panic_chain();
    table[Opcode::rsp_vcl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vcl")
    }
    .check_panic_chain();
    table[Opcode::rsp_vch as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vch")
    }
    .check_panic_chain();
    table[Opcode::rsp_vcr as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vcr")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmrg as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_vd,
            Operand::rsp_vs,
            Operand::rsp_vt_elementhigh,
        ),
        ..OpcodeDescriptor::new("vmrg")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrcp as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrcp")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrcpl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrcpl")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrcph as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrcph")
    }
    .check_panic_chain();
    table[Opcode::rsp_vmov as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vmov")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrsq as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrsq")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrsql as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrsql")
    }
    .check_panic_chain();
    table[Opcode::rsp_vrsqh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vd_de, Operand::rsp_vt_elementhigh),
        ..OpcodeDescriptor::new("vrsqh")
    }
    .check_panic_chain();
    table[Opcode::rsp_vnop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("vnop")
    }
    .check_panic_chain();
    table[Opcode::rsp_lbv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lbv")
    }
    .check_panic_chain();
    table[Opcode::rsp_lsv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lsv")
    }
    .check_panic_chain();
    table[Opcode::rsp_llv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("llv")
    }
    .check_panic_chain();
    table[Opcode::rsp_ldv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("ldv")
    }
    .check_panic_chain();
    table[Opcode::rsp_lqv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lqv")
    }
    .check_panic_chain();
    table[Opcode::rsp_lrv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lrv")
    }
    .check_panic_chain();
    table[Opcode::rsp_lpv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lpv")
    }
    .check_panic_chain();
    table[Opcode::rsp_luv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("luv")
    }
    .check_panic_chain();
    table[Opcode::rsp_lhv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lhv")
    }
    .check_panic_chain();
    table[Opcode::rsp_lfv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("lfv")
    }
    .check_panic_chain();
    table[Opcode::rsp_ltv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("ltv")
    }
    .check_panic_chain();
    table[Opcode::rsp_sbv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sbv")
    }
    .check_panic_chain();
    table[Opcode::rsp_ssv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("ssv")
    }
    .check_panic_chain();
    table[Opcode::rsp_slv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("slv")
    }
    .check_panic_chain();
    table[Opcode::rsp_sdv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sdv")
    }
    .check_panic_chain();
    table[Opcode::rsp_sqv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sqv")
    }
    .check_panic_chain();
    table[Opcode::rsp_srv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("srv")
    }
    .check_panic_chain();
    table[Opcode::rsp_spv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("spv")
    }
    .check_panic_chain();
    table[Opcode::rsp_suv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("suv")
    }
    .check_panic_chain();
    table[Opcode::rsp_shv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("shv")
    }
    .check_panic_chain();
    table[Opcode::rsp_sfv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("sfv")
    }
    .check_panic_chain();
    table[Opcode::rsp_stv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("stv")
    }
    .check_panic_chain();
    table[Opcode::rsp_swv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_vt_elementlow, Operand::rsp_offset_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("swv")
    }
    .check_panic_chain();
    table[Opcode::rsp_j as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        ..OpcodeDescriptor::new("j")
    }
    .check_panic_chain();
    table[Opcode::rsp_jal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_label),
        instr_type: InstrType::J,
        is_jump: true,
        is_jump_with_address: true,
        does_link: true,
        ..OpcodeDescriptor::new("jal")
    }
    .check_panic_chain();
    table[Opcode::rsp_beq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_rs,
            Operand::rsp_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        reads_rt: true,
        is_branch: true,
        ..OpcodeDescriptor::new("beq")
    }
    .check_panic_chain();
    table[Opcode::rsp_bne as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::rsp_rs,
            Operand::rsp_rt,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        reads_rt: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bne")
    }
    .check_panic_chain();
    table[Opcode::rsp_blez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("blez")
    }
    .check_panic_chain();
    table[Opcode::rsp_bgtz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bgtz")
    }
    .check_panic_chain();
    table[Opcode::rsp_addi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        not_emitted_by_compilers: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addi")
    }
    .check_panic_chain();
    table[Opcode::rsp_addiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("addiu")
    }
    .check_panic_chain();
    table[Opcode::rsp_slti as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("slti")
    }
    .check_panic_chain();
    table[Opcode::rsp_sltiu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("sltiu")
    }
    .check_panic_chain();
    table[Opcode::rsp_andi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("andi")
    }
    .check_panic_chain();
    table[Opcode::rsp_ori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        ..OpcodeDescriptor::new("ori")
    }
    .check_panic_chain();
    table[Opcode::rsp_xori as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rt, Operand::rsp_rs, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("xori")
    }
    .check_panic_chain();
    table[Opcode::rsp_lui as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::cpu_immediate),
        instr_type: InstrType::I,
        is_unsigned: true,
        modifies_rt: true,
        can_be_hi: true,
        ..OpcodeDescriptor::new("lui")
    }
    .check_panic_chain();
    table[Opcode::rsp_lb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("lb")
    }
    .check_panic_chain();
    table[Opcode::rsp_lh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("lh")
    }
    .check_panic_chain();
    table[Opcode::rsp_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("lw")
    }
    .check_panic_chain();
    table[Opcode::rsp_lbu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::BYTE,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lbu")
    }
    .check_panic_chain();
    table[Opcode::rsp_lhu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        access_type: AccessType::SHORT,
        does_unsigned_memory_access: true,
        ..OpcodeDescriptor::new("lhu")
    }
    .check_panic_chain();
    table[Opcode::rsp_sb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::BYTE,
        ..OpcodeDescriptor::new("sb")
    }
    .check_panic_chain();
    table[Opcode::rsp_sh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::SHORT,
        ..OpcodeDescriptor::new("sh")
    }
    .check_panic_chain();
    table[Opcode::rsp_sw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        access_type: AccessType::WORD,
        ..OpcodeDescriptor::new("sw")
    }
    .check_panic_chain();
    table[Opcode::rsp_pref as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_hint, Operand::rsp_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("pref")
    }
    .check_panic_chain();
    table[Opcode::rsp_b as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        ..OpcodeDescriptor::new("b")
    }
    .check_panic_chain();
    table[Opcode::rsp_beqz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("beqz")
    }
    .check_panic_chain();
    table[Opcode::rsp_bnez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bnez")
    }
    .check_panic_chain();
    table[Opcode::rsp_sll as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sll")
    }
    .check_panic_chain();
    table[Opcode::rsp_srl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srl")
    }
    .check_panic_chain();
    table[Opcode::rsp_sra as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sra")
    }
    .check_panic_chain();
    table[Opcode::rsp_sllv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sllv")
    }
    .check_panic_chain();
    table[Opcode::rsp_srlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srlv")
    }
    .check_panic_chain();
    table[Opcode::rsp_srav as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rt, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srav")
    }
    .check_panic_chain();
    table[Opcode::rsp_jr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::rsp_rs),
        instr_type: InstrType::R,
        reads_rs: true,
        is_jump: true,
        ..OpcodeDescriptor::new("jr")
    }
    .check_panic_chain();
    table[Opcode::rsp_jalr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::rsp_maybe_rd_rs),
        instr_type: InstrType::R,
        is_jump: true,
        modifies_rd: true,
        reads_rs: true,
        does_link: true,
        ..OpcodeDescriptor::new("jalr")
    }
    .check_panic_chain();
    table[Opcode::rsp_movz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movz")
    }
    .check_panic_chain();
    table[Opcode::rsp_movn as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("movn")
    }
    .check_panic_chain();
    table[Opcode::rsp_add as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("add")
    }
    .check_panic_chain();
    table[Opcode::rsp_addu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("addu")
    }
    .check_panic_chain();
    table[Opcode::rsp_sub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        not_emitted_by_compilers: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sub")
    }
    .check_panic_chain();
    table[Opcode::rsp_subu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("subu")
    }
    .check_panic_chain();
    table[Opcode::rsp_and as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("and")
    }
    .check_panic_chain();
    table[Opcode::rsp_or as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("or")
    }
    .check_panic_chain();
    table[Opcode::rsp_xor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("xor")
    }
    .check_panic_chain();
    table[Opcode::rsp_nor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("nor")
    }
    .check_panic_chain();
    table[Opcode::rsp_slt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("slt")
    }
    .check_panic_chain();
    table[Opcode::rsp_sltu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::rsp_rd, Operand::rsp_rs, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("sltu")
    }
    .check_panic_chain();
    table[Opcode::rsp_break as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_code),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("break")
    }
    .check_panic_chain();
    table[Opcode::rsp_nop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        is_pseudo: true,
        ..OpcodeDescriptor::new("nop")
    }
    .check_panic_chain();
    table[Opcode::rsp_move as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        maybe_is_move: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("move")
    }
    .check_panic_chain();
    table[Opcode::rsp_not as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("not")
    }
    .check_panic_chain();
    table[Opcode::rsp_neg as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        not_emitted_by_compilers: true,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("neg")
    }
    .check_panic_chain();
    table[Opcode::rsp_negu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rd, Operand::rsp_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("negu")
    }
    .check_panic_chain();
    table[Opcode::rsp_bltz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bltz")
    }
    .check_panic_chain();
    table[Opcode::rsp_bgez as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        ..OpcodeDescriptor::new("bgez")
    }
    .check_panic_chain();
    table[Opcode::rsp_bltzal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        does_link: true,
        ..OpcodeDescriptor::new("bltzal")
    }
    .check_panic_chain();
    table[Opcode::rsp_bgezal as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rs, Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        ..OpcodeDescriptor::new("bgezal")
    }
    .check_panic_chain();
    table[Opcode::rsp_bal as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        instr_type: InstrType::REGIMM,
        is_branch: true,
        not_emitted_by_compilers: true,
        does_link: true,
        is_pseudo: true,
        ..OpcodeDescriptor::new("bal")
    }
    .check_panic_chain();
    table[Opcode::rsp_mfc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        modifies_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mfc0")
    }
    .check_panic_chain();
    table[Opcode::rsp_mtc0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::rsp_rt, Operand::rsp_cop0d),
        instr_type: InstrType::UNKNOWN,
        reads_rt: true,
        not_emitted_by_compilers: true,
        ..OpcodeDescriptor::new("mtc0")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18")
    }
    .check_panic_chain();
    table[Opcode::rsp_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19")
    }
    .check_panic_chain();
    table[Opcode::rsp_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("INVALID")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_RTPS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("RTPS")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_RTPT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("RTPT")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_DPCL as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("DPCL")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_DPCS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("DPCS")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_DPCT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("DPCT")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_INTPL as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("INTPL")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCS")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCT")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCDS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCDS")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCDT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCDT")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCCS as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCCS")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCCT as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCCT")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_CDP as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("CDP")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_CC as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("CC")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_NCLIP as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("NCLIP")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_AVSZ3 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("AVSZ3")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_AVSZ4 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("AVSZ4")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_MVMVA as usize] = OpcodeDescriptor {
        operands: Operand::arr5(
            Operand::r3000gte_sf,
            Operand::r3000gte_mx,
            Operand::r3000gte_v,
            Operand::r3000gte_cv,
            Operand::r3000gte_lm,
        ),
        ..OpcodeDescriptor::new("MVMVA")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_SQR as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("SQR")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_OP as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("OP")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_GPF as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("GPF")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_GPL as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r3000gte_sf),
        ..OpcodeDescriptor::new("GPL")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19")
    }
    .check_panic_chain();
    table[Opcode::r3000gte_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("INVALID")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_lv_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_s_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("lv_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_sv_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_s_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        modifies_rs: true,
        ..OpcodeDescriptor::new("sv_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_lv_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("lv_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_sv_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base_maybe_wb,
        ),
        instr_type: InstrType::I,
        modifies_rs: true,
        ..OpcodeDescriptor::new("sv_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_clz as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("clz")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_clo as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("clo")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_madd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("madd")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_maddu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("maddu")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_msub as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("msub")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_msubu as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rs: true,
        reads_rt: true,
        modifies_hi: true,
        modifies_lo: true,
        ..OpcodeDescriptor::new("msubu")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_max as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("max")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_min as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("min")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_srl as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srl")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_rotr as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("rotr")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_srlv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("srlv")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_rotrv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("rotrv")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_sleep as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("sleep")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mfie as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfie")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mtie as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rt),
        instr_type: InstrType::R,
        reads_rt: true,
        ..OpcodeDescriptor::new("mtie")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_ext as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::cpu_rt,
            Operand::cpu_rs,
            Operand::r4000allegrex_pos,
            Operand::r4000allegrex_size,
        ),
        instr_type: InstrType::R,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("ext")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_ins as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::cpu_rt,
            Operand::cpu_rs,
            Operand::r4000allegrex_pos,
            Operand::r4000allegrex_size_plus_pos,
        ),
        instr_type: InstrType::R,
        modifies_rt: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("ins")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_wsbh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("wsbh")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_wsbw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("wsbw")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_seb as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("seb")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_seh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("seh")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bitrev as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("bitrev")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvf as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bvf")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvt as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        ..OpcodeDescriptor::new("bvt")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvfl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bvfl")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_bvtl as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_imm3,
            Operand::cpu_branch_target_label,
        ),
        instr_type: InstrType::UNKNOWN,
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bvtl")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mfv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfv")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mfvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_cop2cd),
        instr_type: InstrType::R,
        modifies_rt: true,
        ..OpcodeDescriptor::new("mfvc")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsync2 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsync2")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mtv as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        reads_rt: true,
        ..OpcodeDescriptor::new("mtv")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_mtvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r4000allegrex_cop2cd),
        instr_type: InstrType::R,
        reads_rt: true,
        ..OpcodeDescriptor::new("mtvc")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vadd_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vadd_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsub_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsub_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsbn_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsbn_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdiv_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdiv_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmul_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmul_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdot_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdot_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdot_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdot_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdot_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdot_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscl_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscl_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscl_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscl_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscl_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscl_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhdp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhdp_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhdp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhdp_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhdp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhdp_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcrs_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcrs_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vdet_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vdet_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmp_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmin_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmax_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmax_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscmp_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscmp_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscmp_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vscmp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vscmp_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsge_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsge_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vslt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vslt_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vwbn_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_bn,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vwbn_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmov_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmov_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vabs_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vabs_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vneg_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vneg_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vidt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vidt_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vidt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vidt_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat0_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat0_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat0_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat0_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat0_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat1_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat1_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat1_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsat1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsat1_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vzero_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vzero_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vzero_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vzero_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vzero_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vone_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vone_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrcp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrcp_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrsq_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrsq_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsin_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcos_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcos_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vexp2_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vexp2_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vexp2_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vexp2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vexp2_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vlog2_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vlog2_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vlog2_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlog2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vlog2_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsqrt_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsqrt_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsqrt_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsqrt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsqrt_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vasin_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vasin_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vasin_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vasin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vasin_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnrcp_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnrcp_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnrcp_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnrcp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnrcp_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnsin_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnsin_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnsin_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnsin_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vnsin_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrexp2_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrexp2_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrexp2_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrexp2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrexp2_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrnds_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrnds_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndi_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndi_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndi_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndi_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndi_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf1_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf1_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf1_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf1_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_s_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf2_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_p_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf2_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_t_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf2_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrndf2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_q_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrndf2_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2h_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2h_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2h_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2h_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vh2f_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vh2f_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vh2f_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vh2f_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsbz_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsbz_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vlgb_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vlgb_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vuc2ifs_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vuc2ifs_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vc2i_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vc2i_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vus2i_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vus2i_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vus2i_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vus2i_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vs2i_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vs2i_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vs2i_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vs2i_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2uc_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2uc_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2c_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2c_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2us_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2us_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2us_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2us_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2s_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2s_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2s_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2s_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsrt1_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsrt2_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vbfy1_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vbfy1_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vbfy1_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vbfy1_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vbfy2_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vbfy2_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vocp_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vocp_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsocp_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsocp_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsocp_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsocp_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfad_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfad_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfad_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfad_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfad_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfad_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vavg_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vavg_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vavg_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vavg_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vavg_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vavg_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt3_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsrt3_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsrt4_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsrt4_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_p_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_t_vd, Operand::r4000allegrex_t_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsgn_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_q_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vsgn_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmfvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vd, Operand::r4000allegrex_cop2cs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmfvc")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmtvc as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_cop2cd, Operand::r4000allegrex_s_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmtvc")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vt4444_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vt4444_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vt5551_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vt5551_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vt5650_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_p_vd, Operand::r4000allegrex_q_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vt5650_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcst_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_vconstant,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcst_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2in_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2in_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2in_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2in_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2in_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iz_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iz_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iz_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iz_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iz_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iu_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iu_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iu_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2iu_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2iu_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2id_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2id_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2id_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vf2id_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vf2id_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vi2f_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_power_of_two,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vi2f_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovt_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovt_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovt_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovt_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_s_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovf_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_p_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovf_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovf_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcmovf_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_vfpu_cc_bit,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcmovf_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vpfxs as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::r4000allegrex_rpx,
            Operand::r4000allegrex_rpy,
            Operand::r4000allegrex_rpz,
            Operand::r4000allegrex_rpw,
        ),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vpfxs")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vpfxt as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::r4000allegrex_rpx,
            Operand::r4000allegrex_rpy,
            Operand::r4000allegrex_rpz,
            Operand::r4000allegrex_rpw,
        ),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vpfxt")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vpfxd as usize] = OpcodeDescriptor {
        operands: Operand::arr4(
            Operand::r4000allegrex_wpx,
            Operand::r4000allegrex_wpy,
            Operand::r4000allegrex_wpz,
            Operand::r4000allegrex_wpw,
        ),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vpfxd")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_viim_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_int16),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("viim_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vfim_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_s_vt, Operand::r4000allegrex_float16),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vfim_s")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmul_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mp_vd,
            Operand::r4000allegrex_mp_vs_transpose,
            Operand::r4000allegrex_mp_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmmul_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmul_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mt_vd,
            Operand::r4000allegrex_mt_vs_transpose,
            Operand::r4000allegrex_mt_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmmul_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmul_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mq_vd,
            Operand::r4000allegrex_mq_vs_transpose,
            Operand::r4000allegrex_mq_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmmul_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhtfm2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_mp_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhtfm2_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vtfm2_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_mp_vs,
            Operand::r4000allegrex_p_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vtfm2_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhtfm3_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_mt_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhtfm3_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vtfm3_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_mt_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vtfm3_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vhtfm4_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_mq_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vhtfm4_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vtfm4_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_mq_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vtfm4_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmscl_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mp_vd,
            Operand::r4000allegrex_mp_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmscl_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmscl_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mt_vd,
            Operand::r4000allegrex_mt_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmscl_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmscl_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_mq_vd,
            Operand::r4000allegrex_mq_vs,
            Operand::r4000allegrex_s_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmscl_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vcrsp_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_t_vs,
            Operand::r4000allegrex_t_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vcrsp_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vqmul_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_q_vs,
            Operand::r4000allegrex_q_vt,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vqmul_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrot_p as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_p_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_p_vrot_code,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrot_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrot_t as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_t_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_t_vrot_code,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrot_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vrot_q as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r4000allegrex_q_vd,
            Operand::r4000allegrex_s_vs,
            Operand::r4000allegrex_q_vrot_code,
        ),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vrot_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmov_p as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_mp_vd, Operand::r4000allegrex_mp_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmmov_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmov_t as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_mt_vd, Operand::r4000allegrex_mt_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmmov_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmmov_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r4000allegrex_mq_vd, Operand::r4000allegrex_mq_vs),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmmov_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmidt_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmidt_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmidt_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmidt_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmidt_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmidt_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmzero_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmzero_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmzero_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmzero_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmzero_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmzero_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmone_p as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mp_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmone_p")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmone_t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mt_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmone_t")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vmone_q as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r4000allegrex_mq_vd),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("vmone_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vnop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vnop")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vsync as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vsync")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_vflush as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("vflush")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_svl_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("svl_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_svr_q as usize] = OpcodeDescriptor {
        operands: Operand::arr2(
            Operand::r4000allegrex_q_vt_imm,
            Operand::r4000allegrex_offset14_base,
        ),
        instr_type: InstrType::I,
        reads_rs: true,
        ..OpcodeDescriptor::new("svr_q")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19")
    }
    .check_panic_chain();
    table[Opcode::r4000allegrex_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX")
    }
    .check_panic_chain();
    table[Opcode::r5900_INVALID as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rs, Operand::cpu_rt, Operand::cpu_immediate),
        instr_type: InstrType::UNKNOWN,
        ..OpcodeDescriptor::new("INVALID")
    }
    .check_panic_chain();
    table[Opcode::r5900_lq as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        modifies_rt: true,
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lq")
    }
    .check_panic_chain();
    table[Opcode::r5900_sq as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::cpu_immediate_base),
        instr_type: InstrType::I,
        reads_rs: true,
        reads_rt: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sq")
    }
    .check_panic_chain();
    table[Opcode::r5900_lqc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vft, Operand::cpu_immediate_base),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("lqc2")
    }
    .check_panic_chain();
    table[Opcode::r5900_sqc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vft, Operand::cpu_immediate_base),
        reads_rs: true,
        can_be_lo: true,
        does_dereference: true,
        does_store: true,
        ..OpcodeDescriptor::new("sqc2")
    }
    .check_panic_chain();
    table[Opcode::r5900_sync_p as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        instr_type: InstrType::R,
        ..OpcodeDescriptor::new("sync_p")
    }
    .check_panic_chain();
    table[Opcode::r5900_mult as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        instr_type: InstrType::R,
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("mult")
    }
    .check_panic_chain();
    table[Opcode::r5900_mfsa as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("mfsa")
    }
    .check_panic_chain();
    table[Opcode::r5900_mtsa as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("mtsa")
    }
    .check_panic_chain();
    table[Opcode::r5900_mtsab as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        ..OpcodeDescriptor::new("mtsab")
    }
    .check_panic_chain();
    table[Opcode::r5900_mtsah as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_immediate),
        instr_type: InstrType::REGIMM,
        reads_rs: true,
        ..OpcodeDescriptor::new("mtsah")
    }
    .check_panic_chain();
    table[Opcode::r5900_madd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("madd")
    }
    .check_panic_chain();
    table[Opcode::r5900_maddu as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("maddu")
    }
    .check_panic_chain();
    table[Opcode::r5900_plzcw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        ..OpcodeDescriptor::new("plzcw")
    }
    .check_panic_chain();
    table[Opcode::r5900_mfhi1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("mfhi1")
    }
    .check_panic_chain();
    table[Opcode::r5900_mthi1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("mthi1")
    }
    .check_panic_chain();
    table[Opcode::r5900_mflo1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        reads_rd: true,
        ..OpcodeDescriptor::new("mflo1")
    }
    .check_panic_chain();
    table[Opcode::r5900_mtlo1 as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("mtlo1")
    }
    .check_panic_chain();
    table[Opcode::r5900_mult1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("mult1")
    }
    .check_panic_chain();
    table[Opcode::r5900_multu1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("multu1")
    }
    .check_panic_chain();
    table[Opcode::r5900_div1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_zero, Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("div1")
    }
    .check_panic_chain();
    table[Opcode::r5900_divu1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_zero, Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("divu1")
    }
    .check_panic_chain();
    table[Opcode::r5900_madd1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("madd1")
    }
    .check_panic_chain();
    table[Opcode::r5900_maddu1 as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("maddu1")
    }
    .check_panic_chain();
    table[Opcode::r5900_psllh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psllh")
    }
    .check_panic_chain();
    table[Opcode::r5900_psrlh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrlh")
    }
    .check_panic_chain();
    table[Opcode::r5900_psrah as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrah")
    }
    .check_panic_chain();
    table[Opcode::r5900_psllw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psllw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psrlw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrlw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psraw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_sa),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psraw")
    }
    .check_panic_chain();
    table[Opcode::r5900_paddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pcgtw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcgtw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaxw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaxw")
    }
    .check_panic_chain();
    table[Opcode::r5900_paddh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddh")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pcgth as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcgth")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaxh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaxh")
    }
    .check_panic_chain();
    table[Opcode::r5900_paddb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddb")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubb")
    }
    .check_panic_chain();
    table[Opcode::r5900_pcgtb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcgtb")
    }
    .check_panic_chain();
    table[Opcode::r5900_paddsw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddsw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubsw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubsw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pextlw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextlw")
    }
    .check_panic_chain();
    table[Opcode::r5900_ppacw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppacw")
    }
    .check_panic_chain();
    table[Opcode::r5900_paddsh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddsh")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubsh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubsh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pextlh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextlh")
    }
    .check_panic_chain();
    table[Opcode::r5900_ppach as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppach")
    }
    .check_panic_chain();
    table[Opcode::r5900_paddsb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("paddsb")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubsb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubsb")
    }
    .check_panic_chain();
    table[Opcode::r5900_pextlb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextlb")
    }
    .check_panic_chain();
    table[Opcode::r5900_ppacb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppacb")
    }
    .check_panic_chain();
    table[Opcode::r5900_pext5 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pext5")
    }
    .check_panic_chain();
    table[Opcode::r5900_ppac5 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("ppac5")
    }
    .check_panic_chain();
    table[Opcode::r5900_pabsw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pabsw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pceqw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pceqw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pminw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pminw")
    }
    .check_panic_chain();
    table[Opcode::r5900_padsbh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("padsbh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pabsh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pabsh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pceqh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pceqh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pminh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pminh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pceqb as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pceqb")
    }
    .check_panic_chain();
    table[Opcode::r5900_padduw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("padduw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubuw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubuw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pextuw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextuw")
    }
    .check_panic_chain();
    table[Opcode::r5900_padduh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("padduh")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubuh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubuh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pextuh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextuh")
    }
    .check_panic_chain();
    table[Opcode::r5900_paddub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        maybe_is_move: true,
        ..OpcodeDescriptor::new("paddub")
    }
    .check_panic_chain();
    table[Opcode::r5900_psubub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psubub")
    }
    .check_panic_chain();
    table[Opcode::r5900_pextub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pextub")
    }
    .check_panic_chain();
    table[Opcode::r5900_qfsrv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("qfsrv")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaddw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psllvw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psllvw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psrlvw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psrlvw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmsubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmsubw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhi")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmflo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmflo")
    }
    .check_panic_chain();
    table[Opcode::r5900_pinth as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pinth")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmultw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmultw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pdivw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pdivw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pcpyld as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcpyld")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmaddh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmaddh")
    }
    .check_panic_chain();
    table[Opcode::r5900_phmadh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("phmadh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pand")
    }
    .check_panic_chain();
    table[Opcode::r5900_pxor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pxor")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmsubh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmsubh")
    }
    .check_panic_chain();
    table[Opcode::r5900_phmsbh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("phmsbh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pexeh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexeh")
    }
    .check_panic_chain();
    table[Opcode::r5900_prevh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("prevh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmulth as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmulth")
    }
    .check_panic_chain();
    table[Opcode::r5900_pdivbw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pdivbw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pexew as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexew")
    }
    .check_panic_chain();
    table[Opcode::r5900_prot3w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("prot3w")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmadduw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmadduw")
    }
    .check_panic_chain();
    table[Opcode::r5900_psravw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rt, Operand::cpu_rs),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("psravw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmthi as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("pmthi")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmtlo as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("pmtlo")
    }
    .check_panic_chain();
    table[Opcode::r5900_pinteh as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pinteh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmultuw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pmultuw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pdivuw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rs, Operand::cpu_rt),
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pdivuw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pcpyud as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcpyud")
    }
    .check_panic_chain();
    table[Opcode::r5900_por as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("por")
    }
    .check_panic_chain();
    table[Opcode::r5900_pnor as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_rd, Operand::cpu_rs, Operand::cpu_rt),
        modifies_rd: true,
        reads_rs: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pnor")
    }
    .check_panic_chain();
    table[Opcode::r5900_pexch as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexch")
    }
    .check_panic_chain();
    table[Opcode::r5900_pcpyh as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pcpyh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pexcw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rd, Operand::cpu_rt),
        modifies_rd: true,
        reads_rt: true,
        ..OpcodeDescriptor::new("pexcw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_lw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_uw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_uw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_slw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_slw")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_lh as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_lh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmfhl_sh as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rd),
        modifies_rd: true,
        ..OpcodeDescriptor::new("pmfhl_sh")
    }
    .check_panic_chain();
    table[Opcode::r5900_pmthl_lw as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_rs),
        reads_rs: true,
        ..OpcodeDescriptor::new("pmthl_lw")
    }
    .check_panic_chain();
    table[Opcode::r5900_ei as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("ei")
    }
    .check_panic_chain();
    table[Opcode::r5900_di as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("di")
    }
    .check_panic_chain();
    table[Opcode::r5900_c1__sqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_copraw),
        ..OpcodeDescriptor::new("c1__sqrt_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_rsqrt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("rsqrt_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_adda_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("adda_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_suba_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("suba_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_mula_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("mula_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_madd_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("madd_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_msub_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("msub_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_madda_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("madda_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_msuba_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("msuba_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_max_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("max_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_min_s as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::cpu_fd, Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        modifies_fd: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("min_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_c_lt_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_lt_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_c_le_s as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_fs, Operand::cpu_ft),
        is_float: true,
        reads_fs: true,
        reads_ft: true,
        ..OpcodeDescriptor::new("c_le_s")
    }
    .check_panic_chain();
    table[Opcode::r5900_qmfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vfs),
        modifies_rt: true,
        ..OpcodeDescriptor::new("qmfc2")
    }
    .check_panic_chain();
    table[Opcode::r5900_cfc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vis),
        modifies_rt: true,
        ..OpcodeDescriptor::new("cfc2")
    }
    .check_panic_chain();
    table[Opcode::r5900_qmtc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vfs),
        reads_rt: true,
        ..OpcodeDescriptor::new("qmtc2")
    }
    .check_panic_chain();
    table[Opcode::r5900_ctc2 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::cpu_rt, Operand::r5900_vis),
        reads_rt: true,
        ..OpcodeDescriptor::new("ctc2")
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2f as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        ..OpcodeDescriptor::new("bc2f")
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2t as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        ..OpcodeDescriptor::new("bc2t")
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2fl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc2fl")
    }
    .check_panic_chain();
    table[Opcode::r5900_bc2tl as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::cpu_branch_target_label),
        is_branch: true,
        is_branch_likely: true,
        ..OpcodeDescriptor::new("bc2tl")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddx")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddy")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubx")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsuby as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsuby")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddx")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddy")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubx")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsuby as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsuby")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxx")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxy")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vminix as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminix")
    }
    .check_panic_chain();
    table[Opcode::r5900_vminiy as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminiy")
    }
    .check_panic_chain();
    table[Opcode::r5900_vminiz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminiz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vminiw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminiw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulx as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulx")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmuly as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmuly")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmulq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaxi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaxi")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmuli as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmuli")
    }
    .check_panic_chain();
    table[Opcode::r5900_vminii as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vminii")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddi")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddi")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubi")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubi")
    }
    .check_panic_chain();
    table[Opcode::r5900_vadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vadd")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmadd")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmul as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmul")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmax")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsub")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsub")
    }
    .check_panic_chain();
    table[Opcode::r5900_vopmsub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vopmsub")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmini as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vfdxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmini")
    }
    .check_panic_chain();
    table[Opcode::r5900_viadd as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("viadd")
    }
    .check_panic_chain();
    table[Opcode::r5900_visub as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("visub")
    }
    .check_panic_chain();
    table[Opcode::r5900_viaddi as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_vit,
            Operand::r5900_vis,
            Operand::r5900_immediate5,
        ),
        ..OpcodeDescriptor::new("viaddi")
    }
    .check_panic_chain();
    table[Opcode::r5900_viand as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("viand")
    }
    .check_panic_chain();
    table[Opcode::r5900_vior as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_vid, Operand::r5900_vis, Operand::r5900_vit),
        ..OpcodeDescriptor::new("vior")
    }
    .check_panic_chain();
    table[Opcode::r5900_vcallms as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r5900_immediate15),
        ..OpcodeDescriptor::new("vcallms")
    }
    .check_panic_chain();
    table[Opcode::r5900_vcallmsr as usize] = OpcodeDescriptor {
        operands: Operand::arr1(Operand::r5900_vis),
        ..OpcodeDescriptor::new("vcallmsr")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddax")
    }
    .check_panic_chain();
    table[Opcode::r5900_vadday as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vadday")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddaz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddaw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubax")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubay as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubay")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubaz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubaw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmaddax")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmadday as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmadday")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmaddaz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmaddaw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubax")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubay as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubay")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubaz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubaw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof0")
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof4 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof4")
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof12 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof12")
    }
    .check_panic_chain();
    table[Opcode::r5900_vitof15 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vitof15")
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi0 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi0")
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi4 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi4")
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi12 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi12")
    }
    .check_panic_chain();
    table[Opcode::r5900_vftoi15 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vftoi15")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulax as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulax")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulay as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulay")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulaz as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulaz")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulaw as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftn,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulaw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulaq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vabs as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vabs")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmulai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmulai")
    }
    .check_panic_chain();
    table[Opcode::r5900_vclipw as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vftn),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vclipw")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddaq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddaq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vaddai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vaddai")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmaddai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmaddai")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubaq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubaq as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_Q,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubaq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsubai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsubai")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsubai as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_I,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsubai")
    }
    .check_panic_chain();
    table[Opcode::r5900_vadda as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vftxyzw,
            Operand::r5900_vfsxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vadda")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmadda as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmadda")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmula as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        ..OpcodeDescriptor::new("vmula")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsuba as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsuba")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmsuba as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vftxyzw,
            Operand::r5900_vfsxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmsuba")
    }
    .check_panic_chain();
    table[Opcode::r5900_vopmula as usize] = OpcodeDescriptor {
        operands: Operand::arr3(
            Operand::r5900_ACCxyzw,
            Operand::r5900_vfsxyzw,
            Operand::r5900_vftxyzw,
        ),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vopmula")
    }
    .check_panic_chain();
    table[Opcode::r5900_vnop as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("vnop")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmove as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmove")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmr32 as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vfsxyzw),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmr32")
    }
    .check_panic_chain();
    table[Opcode::r5900_vlqi as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vlqi")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsqi as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_postincr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsqi")
    }
    .check_panic_chain();
    table[Opcode::r5900_vlqd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vlqd")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsqd as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vfsxyzw, Operand::r5900_vit_predecr),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vsqd")
    }
    .check_panic_chain();
    table[Opcode::r5900_vdiv as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl, Operand::r5900_vftm),
        ..OpcodeDescriptor::new("vdiv")
    }
    .check_panic_chain();
    table[Opcode::r5900_vsqrt as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_Q, Operand::r5900_vftm),
        ..OpcodeDescriptor::new("vsqrt")
    }
    .check_panic_chain();
    table[Opcode::r5900_vrsqrt as usize] = OpcodeDescriptor {
        operands: Operand::arr3(Operand::r5900_Q, Operand::r5900_vfsl, Operand::r5900_vftm),
        ..OpcodeDescriptor::new("vrsqrt")
    }
    .check_panic_chain();
    table[Opcode::r5900_vwaitq as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("vwaitq")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmtir as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vfsl),
        ..OpcodeDescriptor::new("vmtir")
    }
    .check_panic_chain();
    table[Opcode::r5900_vmfir as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_vis),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vmfir")
    }
    .check_panic_chain();
    table[Opcode::r5900_vrnext as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vrnext")
    }
    .check_panic_chain();
    table[Opcode::r5900_vrget as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vftxyzw, Operand::r5900_R),
        instr_suffix: InstrSuffix::R5900_xyzw,
        is_float: true,
        ..OpcodeDescriptor::new("vrget")
    }
    .check_panic_chain();
    table[Opcode::r5900_vrinit as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl),
        ..OpcodeDescriptor::new("vrinit")
    }
    .check_panic_chain();
    table[Opcode::r5900_vrxor as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_R, Operand::r5900_vfsl),
        ..OpcodeDescriptor::new("vrxor")
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_w")
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_z as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_z")
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_y as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_y")
    }
    .check_panic_chain();
    table[Opcode::r5900_vilwr_x as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("vilwr_x")
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_w as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_w")
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_z as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_z")
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_y as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_y")
    }
    .check_panic_chain();
    table[Opcode::r5900_viswr_x as usize] = OpcodeDescriptor {
        operands: Operand::arr2(Operand::r5900_vit, Operand::r5900_vis_parenthesis),
        is_float: true,
        does_dereference: true,
        does_load: true,
        ..OpcodeDescriptor::new("viswr_x")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_00 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_00")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_01 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_01")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_02 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_02")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_03 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_03")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_04 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_04")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_05 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_05")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_06 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_06")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_07 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_07")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_08 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_08")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_09 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_09")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_10 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_10")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_11 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_11")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_12 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_12")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_13 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_13")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_14 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_14")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_15 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_15")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_16 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_16")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_17 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_17")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_18 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_18")
    }
    .check_panic_chain();
    table[Opcode::r5900_USERDEF_19 as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("USERDEF_19")
    }
    .check_panic_chain();
    table[Opcode::r5900_MAX as usize] = OpcodeDescriptor {
        operands: Operand::arr0(),
        ..OpcodeDescriptor::new("MAX")
    }
    .check_panic_chain();
    let mut i = 0;
    while i < opcode::OPCODE_COUNT {
        table[i].check_panic();
        i += 1;
    }
    table
};
