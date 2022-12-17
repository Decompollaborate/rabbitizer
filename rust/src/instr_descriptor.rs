/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{operand_type_enum, instr_suffix_enum, access_type_enum};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct InstrDescriptor {
    operands: [operand_type_enum::OperandType; 4usize],
    instr_type: u32, // RabbitizerInstrType, // This type is deprecated and I don't care enough to make a binding for it

    instr_suffix: instr_suffix_enum::InstrSuffix,

    is_branch: bool,
    is_branch_likely: bool,
    is_jump: bool,
    is_jump_with_address: bool,
    is_trap: bool,

    is_float: bool,
    is_double: bool,

    is_unsigned: bool,

    modifies_rt: bool,
    modifies_rd: bool,

    reads_rs: bool,
    reads_rt: bool,
    reads_rd: bool,

    reads_hi: bool,
    reads_lo: bool,
    modifies_hi: bool,
    modifies_lo: bool,

    not_emited_by_compilers: bool,

    can_be_hi: bool,
    can_be_lo: bool,
    does_link: bool,

    does_dereference: bool,
    does_load: bool,
    does_store: bool,

    maybe_is_move: bool,

    is_pseudo: bool,

    access_type: access_type_enum::AccessType,
    does_unsigned_memory_access: bool,
}
