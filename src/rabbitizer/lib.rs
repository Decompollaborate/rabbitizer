/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! This crate provides a MIPS instruction decoder for MIPS versions I to IV. This also recognizes multiple MIPS
//! extensions, including RSP (N64 reality signal procesor), R3000 GTE (PSX), R4000 ALLEGREX (PSP) and R5900 EE (PS2
//! Emotion Engine).

#![warn(clippy::exhaustive_enums)]
#![warn(clippy::use_self)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::pattern_type_mismatch)]
// #![warn(clippy::missing_inline_in_public_items)] // TODO
// #![warn(missing_docs)] // TODO: change to `deny`
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
pub mod register_descriptors;
pub mod registers;
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
pub use vram::Vram;
pub use vram_offset::VramOffset;

// TODO: maybe not make public?
pub use instruction_display::InstructionDisplay;

pub use flags::DecodingFlags;
pub use flags::DisplayFlags;
pub use flags::InstructionFlags;

pub use error::Error;
