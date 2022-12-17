/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::{operand_type_enum, instr_suffix_enum, access_type_enum};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct InstrDescriptor {
    operands: [operand_type_enum::OperandType; 4usize],
    instrType: u32, // RabbitizerInstrType, // This type is deprecated and I don't care enough to make a binding for it
    instrSuffix: instr_suffix_enum::InstrSuffix,
    isBranch: bool,
    isBranchLikely: bool,
    isJump: bool,
    isJumpWithAddress: bool,
    isTrap: bool,
    isFloat: bool,
    isDouble: bool,
    isUnsigned: bool,
    modifiesRt: bool,
    modifiesRd: bool,
    readsHI: bool,
    readsLO: bool,
    modifiesHI: bool,
    modifiesLO: bool,
    notEmitedByCompilers: bool,
    canBeHi: bool,
    canBeLo: bool,
    doesLink: bool,
    doesDereference: bool,
    doesLoad: bool,
    doesStore: bool,
    maybeIsMove: bool,
    isPseudo: bool,
    accessType: access_type_enum::AccessType,
    doesUnsignedMemoryAccess: bool,
}
