/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod array_opcodes;
mod enum_access_type;
mod enum_instr_suffix;
mod enum_opcode;
mod enum_operand;

pub use array_opcodes::OPCODES;
pub use enum_access_type::AccessType;
pub use enum_instr_suffix::InstrSuffix;
pub use enum_opcode::Opcode;
pub use enum_operand::Operand;
