/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{instr_id_enum, instr_category_enum, instr_descriptor};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
struct Instruction {
    #[allow(non_camel_case_types)]
    word: u32,
    #[allow(non_camel_case_types)]
    _mandatorybits: u32,
    #[allow(non_camel_case_types)]
    uniqueId: instr_id_enum::InstrId,
    #[allow(non_camel_case_types)]
    descriptor: *const instr_descriptor::InstrDescriptor,
    #[allow(non_camel_case_types)]
    vram: u32,
    #[allow(non_camel_case_types)]
    _handwrittenCategory: bool,
    #[allow(non_camel_case_types)]
    inHandwrittenFunction: bool,
    #[allow(non_camel_case_types)]
    category: instr_category_enum::InstrCategory,
}
