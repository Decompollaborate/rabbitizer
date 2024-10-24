/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![warn(clippy::exhaustive_enums)]
#![warn(clippy::use_self)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::pattern_type_mismatch)]
// TODO: consider adding clippy::missing_inline_in_public_items
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

//#[cfg(feature = "std")]
//#[macro_use]
//use std::prelude::*;

mod generated;

mod abi;
mod access_type;
mod encoded_field_mask;
mod error;
mod flags;
mod instr_suffix;
mod instr_type;
mod instruction;
mod instruction_display;
pub mod isa;
pub mod opcodes;
pub mod operands;
mod register_descriptor;
mod register_impls;
pub mod traits;
mod vram;
mod vram_offset;

mod utils;

pub use generated::Abi;
pub use generated::AccessType;
pub use generated::InstrSuffix;

#[allow(deprecated)]
pub use instr_type::InstrType;

pub use encoded_field_mask::EncodedFieldMask;
pub use instruction::Instruction;
pub use register_descriptor::RegisterDescriptor;
pub use vram::Vram;
pub use vram_offset::VramOffset;

// TODO: maybe not make public?
pub use instruction_display::InstructionDisplay;

pub use flags::DecodingFlags;
pub use flags::DisplayFlags;
pub use flags::InstructionFlags;

pub use error::Error;

pub mod registers {
    use crate::generated;

    pub use generated::Cop0;
    pub use generated::Cop0Control;
    pub use generated::Cop1;
    pub use generated::Cop1Control;
    pub use generated::Cop2;
    pub use generated::Cop2Control;
    pub use generated::Gpr;
    pub use generated::R4000AllegrexM2x2;
    pub use generated::R4000AllegrexM3x3;
    pub use generated::R4000AllegrexM4x4;
    pub use generated::R4000AllegrexPrefixDst;
    pub use generated::R4000AllegrexPrefixSrc;
    pub use generated::R4000AllegrexS;
    pub use generated::R4000AllegrexV2D;
    pub use generated::R4000AllegrexV3D;
    pub use generated::R4000AllegrexV4D;
    pub use generated::R4000AllegrexVCond;
    pub use generated::R4000AllegrexVConstant;
    pub use generated::R4000AllegrexVfpuControl;
    pub use generated::RspCop0;
    pub use generated::RspCop2;
    pub use generated::RspCop2Control;
    pub use generated::RspVector;
    pub use generated::R5900VF;
    pub use generated::R5900VI;
}

pub mod register_descriptors {
    use crate::generated;

    pub use generated::COP0;
    pub use generated::COP0_CONTROL;
    pub use generated::COP1;
    pub use generated::COP1_CONTROL;
    pub use generated::COP2;
    pub use generated::COP2_CONTROL;
    pub use generated::GPR;
    pub use generated::R4000ALLEGREX_M2X2;
    pub use generated::R4000ALLEGREX_M3X3;
    pub use generated::R4000ALLEGREX_M4X4;
    pub use generated::R4000ALLEGREX_PREFIX_DST;
    pub use generated::R4000ALLEGREX_PREFIX_SRC;
    pub use generated::R4000ALLEGREX_S;
    pub use generated::R4000ALLEGREX_V2D;
    pub use generated::R4000ALLEGREX_V3D;
    pub use generated::R4000ALLEGREX_V4D;
    pub use generated::R4000ALLEGREX_VCOND;
    pub use generated::R4000ALLEGREX_VCONSTANT;
    pub use generated::R4000ALLEGREX_VFPUCONTROL;
    pub use generated::R5900_VF;
    pub use generated::R5900_VI;
    pub use generated::RSP_COP0;
    pub use generated::RSP_COP2;
    pub use generated::RSP_COP2_CONTROL;
    pub use generated::RSP_VECTOR;
}
