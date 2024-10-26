/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod decoding_flags;
mod opcode;
mod opcode_category;
mod opcode_category_descriptor;
mod opcode_decoder;
mod opcode_descriptor;

pub(crate) use decoding_flags::DecodingFlags;

pub(crate) use crate::generated::OPCODE_CATEGORIES;
pub use crate::generated::{Opcode, OpcodeCategory, OPCODES};

pub(crate) use opcode::OPCODE_COUNT;
pub(crate) use opcode_category::OPCODE_CATEGORY_COUNT;
pub use opcode_category_descriptor::OpcodeCategoryDescriptor;
pub(crate) use opcode_decoder::OpcodeDecoder;
pub use opcode_descriptor::OpcodeDescriptor;
