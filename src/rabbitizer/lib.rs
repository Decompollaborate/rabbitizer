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
pub mod display_flags;
mod encoded_field_mask;
mod error;
pub mod instr;
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

// pub use instruction::Instruction;
pub use encoded_field_mask::EncodedFieldMask;
pub use vram::Vram;
pub use vram_offset::VramOffset;

pub use error::Error;
