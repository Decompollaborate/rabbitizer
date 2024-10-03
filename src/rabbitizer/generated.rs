/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// Avoid linting generated code
// #![allow(clippy::all)]
// #![allow(clippy::pedantic)]
// #![allow(clippy::restriction)]
// #![allow(clippy::nursery)]
#![allow(clippy::manual_range_patterns)]
#![allow(clippy::needless_return)]
#![warn(clippy::exhaustive_enums)]

mod array_opcodes;
mod code_decode_isa_extension_none;
mod code_decode_isa_extension_r3000gte;
mod code_decode_isa_extension_r4000allegrex;
mod code_decode_isa_extension_r5900;
mod code_decode_isa_extension_rsp;
mod enum_abi;
mod enum_access_type;
mod enum_instr_suffix;
mod enum_isa_extension;
mod enum_isa_version;
mod enum_opcode;
mod enum_opcode_category;
mod enum_operand;
mod enum_register_cop0;
mod enum_register_cop0control;
mod enum_register_cop1;
mod enum_register_cop1control;
mod enum_register_cop2;
mod enum_register_cop2control;
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
mod enum_register_rsp_cop0;
mod enum_register_rsp_cop2;
mod enum_register_rsp_cop2_control;
mod enum_register_rsp_vector;

pub use array_opcodes::OPCODES;
pub use enum_abi::Abi;
pub use enum_access_type::AccessType;
pub use enum_instr_suffix::InstrSuffix;
pub use enum_isa_extension::IsaExtension;
pub use enum_isa_version::IsaVersion;
pub use enum_opcode::Opcode;
pub use enum_opcode_category::OpcodeCategory;
pub use enum_opcode_category::OPCODE_CATEGORIES;
pub(crate) use enum_operand::DISPLAY_OPERAND_CALLBACKS;
pub use enum_operand::{Operand, ValuedOperand, OPERANDS};
pub use enum_register_cop0::{Cop0, COP0};
pub use enum_register_cop0control::{Cop0Control, COP0_CONTROL};
pub use enum_register_cop1::{Cop1, COP1};
pub use enum_register_cop1control::{Cop1Control, COP1_CONTROL};
pub use enum_register_cop2::{Cop2, COP2};
pub use enum_register_cop2control::{Cop2Control, COP2_CONTROL};
pub use enum_register_gpr::{Gpr, GPR};
pub use enum_register_r4000allegrex_m2x2::{R4000AllegrexM2x2, R4000ALLEGREX_M2X2};
pub use enum_register_r4000allegrex_m3x3::{R4000AllegrexM3x3, R4000ALLEGREX_M3X3};
pub use enum_register_r4000allegrex_m4x4::{R4000AllegrexM4x4, R4000ALLEGREX_M4X4};
pub use enum_register_r4000allegrex_s::{R4000AllegrexS, R4000ALLEGREX_S};
pub use enum_register_r4000allegrex_v2d::{R4000AllegrexV2D, R4000ALLEGREX_V2D};
pub use enum_register_r4000allegrex_v3d::{R4000AllegrexV3D, R4000ALLEGREX_V3D};
pub use enum_register_r4000allegrex_v4d::{R4000AllegrexV4D, R4000ALLEGREX_V4D};
pub use enum_register_r4000allegrex_vconstant::{R4000AllegrexVConstant, R4000ALLEGREX_VCONSTANT};
pub use enum_register_r4000allegrex_vfpucontrol::{
    R4000AllegrexVfpuControl, R4000ALLEGREX_VFPUCONTROL,
};
pub use enum_register_r5900allegrex_vf::{R5900VF, R5900_VF};
pub use enum_register_r5900allegrex_vi::{R5900VI, R5900_VI};
pub use enum_register_rsp_cop0::{RspCop0, RSP_COP0};
pub use enum_register_rsp_cop2::{RspCop2, RSP_COP2};
pub use enum_register_rsp_cop2_control::{RspCop2Control, RSP_COP2_CONTROL};
pub use enum_register_rsp_vector::{RspVector, RSP_VECTOR};
