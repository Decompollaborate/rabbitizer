/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// Avoid linting generated code
// #![allow(clippy::all)]
// #![allow(clippy::pedantic)]
// #![allow(clippy::restriction)]
// #![allow(clippy::nursery)]
#![allow(clippy::manual_range_patterns)]
#![allow(clippy::needless_late_init)]
#![warn(clippy::exhaustive_enums)]

mod array_opcodes;
mod array_operands;
mod array_register_cop1;
mod array_register_gpr;
mod code_decode_isa_extension_none;
mod code_decode_isa_extension_r3000gte;
mod code_decode_isa_extension_r5900;
mod enum_abi;
mod enum_access_type;
mod enum_instr_suffix;
mod enum_isa_extension;
mod enum_isa_version;
mod enum_opcode;
mod enum_opcode_category;
mod enum_operand;
mod enum_register_cop1;
mod enum_register_gpr;

pub use array_opcodes::OPCODES;
pub use array_operands::OPERANDS;
pub use array_register_cop1::COP1_REGISTERS;
pub use array_register_gpr::GPR_REGISTERS;
pub use enum_abi::Abi;
pub use enum_access_type::AccessType;
pub use enum_instr_suffix::InstrSuffix;
pub use enum_isa_extension::IsaExtension;
pub use enum_isa_version::IsaVersion;
pub use enum_opcode::Opcode;
pub use enum_opcode_category::OpcodeCategory;
pub use enum_operand::Operand;
pub use enum_register_cop1::RegisterCop1;
pub use enum_register_gpr::RegisterGpr;
