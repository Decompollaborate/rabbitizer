/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod instr_field;
mod instr_suffix;
mod instr_type;
mod instruction;
mod instruction_display;
mod instruction_flags;

pub use crate::generated::InstrSuffix;

#[allow(deprecated)]
pub use instr_type::InstrType;

pub use instr_field::InstrField;
pub use instruction::Instruction;
pub use instruction_display::InstructionDisplay;
pub use instruction_flags::InstructionFlags;
