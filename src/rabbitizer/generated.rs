/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod array_opcodes;
mod code_decode_isa_extension_none;
mod code_decode_isa_extension_r5900;
mod enum_access_type;
mod enum_instr_suffix;
mod enum_isa_extension;
mod enum_isa_version;
mod enum_opcode;
mod enum_opcode_category;
mod enum_operand;

pub use array_opcodes::OPCODES;
pub use enum_access_type::AccessType;
pub use enum_instr_suffix::InstrSuffix;
pub use enum_isa_extension::IsaExtension;
pub use enum_isa_version::IsaVersion;
pub use enum_opcode::Opcode;
pub use enum_opcode_category::OpcodeCategory;
pub use enum_operand::Operand;
