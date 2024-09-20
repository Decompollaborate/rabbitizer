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
mod array_register_cop0;
mod array_register_cop1;
mod array_register_cop1control;
mod array_register_cop2;
mod array_register_gpr;
mod array_register_r4000allegrex_m2x2;
mod array_register_r4000allegrex_m3x3;
mod array_register_r4000allegrex_m4x4;
mod array_register_r4000allegrex_s;
mod array_register_r4000allegrex_v2d;
mod array_register_r4000allegrex_v3d;
mod array_register_r4000allegrex_v4d;
mod array_register_r4000allegrex_vconstant;
mod array_register_r4000allegrex_vfpucontrol;
mod array_register_r5900allegrex_vf;
mod array_register_r5900allegrex_vi;
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
mod enum_register_cop0;
mod enum_register_cop1;
mod enum_register_cop1control;
mod enum_register_cop2;
mod enum_register_gpr;
mod enum_register_r4000allegrex_m2x2;
mod enum_register_r4000allegrex_m3x3;
mod enum_register_r4000allegrex_m4x4;
mod enum_register_r4000allegrex_s;
mod enum_register_r4000allegrex_v2d;
mod enum_register_r4000allegrex_v3d;
mod enum_register_r4000allegrex_v4d;
mod enum_register_r4000allegrex_vconstant;
mod enum_register_r4000allegrex_vfpucontrol;
mod enum_register_r5900allegrex_vf;
mod enum_register_r5900allegrex_vi;

pub use array_opcodes::OPCODES;
pub use array_operands::OPERANDS;
pub use array_register_cop0::COP0_REGISTERS;
pub use array_register_cop1::COP1_REGISTERS;
pub use array_register_cop1control::COP1CONTROL_REGISTERS;
pub use array_register_cop2::COP2_REGISTERS;
pub use array_register_gpr::GPR_REGISTERS;
pub use array_register_r4000allegrex_m2x2::R4000ALLEGREX_M2X2_REGISTERS;
pub use array_register_r4000allegrex_m3x3::R4000ALLEGREX_M3X3_REGISTERS;
pub use array_register_r4000allegrex_m4x4::R4000ALLEGREX_M4X4_REGISTERS;
pub use array_register_r4000allegrex_s::R4000ALLEGREX_S_REGISTERS;
pub use array_register_r4000allegrex_v2d::R4000ALLEGREX_V2D_REGISTERS;
pub use array_register_r4000allegrex_v3d::R4000ALLEGREX_V3D_REGISTERS;
pub use array_register_r4000allegrex_v4d::R4000ALLEGREX_V4D_REGISTERS;
pub use array_register_r4000allegrex_vconstant::R4000ALLEGREX_VCONSTANT_REGISTERS;
pub use array_register_r4000allegrex_vfpucontrol::R4000ALLEGREX_VFPUCONTROL_REGISTERS;
pub use array_register_r5900allegrex_vf::R5900_VF_REGISTERS;
pub use array_register_r5900allegrex_vi::R5900_VI_REGISTERS;
pub use enum_abi::Abi;
pub use enum_access_type::AccessType;
pub use enum_instr_suffix::InstrSuffix;
pub use enum_isa_extension::IsaExtension;
pub use enum_isa_version::IsaVersion;
pub use enum_opcode::Opcode;
pub use enum_opcode_category::OpcodeCategory;
pub use enum_operand::Operand;
pub use enum_register_cop0::RegisterCop0;
pub use enum_register_cop1::RegisterCop1;
pub use enum_register_cop1control::RegisterCop1Control;
pub use enum_register_cop2::RegisterCop2;
pub use enum_register_gpr::RegisterGpr;
pub use enum_register_r4000allegrex_m2x2::RegisterR4000AllegrexM2x2;
pub use enum_register_r4000allegrex_m3x3::RegisterR4000AllegrexM3x3;
pub use enum_register_r4000allegrex_m4x4::RegisterR4000AllegrexM4x4;
pub use enum_register_r4000allegrex_s::RegisterR4000AllegrexS;
pub use enum_register_r4000allegrex_v2d::RegisterR4000AllegrexV2D;
pub use enum_register_r4000allegrex_v3d::RegisterR4000AllegrexV3D;
pub use enum_register_r4000allegrex_v4d::RegisterR4000AllegrexV4D;
pub use enum_register_r4000allegrex_vconstant::RegisterR4000AllegrexVConstant;
pub use enum_register_r4000allegrex_vfpucontrol::RegisterR4000AllegrexVfpuControl;
pub use enum_register_r5900allegrex_vf::RegisterR5900VF;
pub use enum_register_r5900allegrex_vi::RegisterR5900VI;
