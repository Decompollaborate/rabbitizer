/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// Avoid linting generated code
// #![allow(clippy::all)]
// #![allow(clippy::pedantic)]
// #![allow(clippy::restriction)]
// #![allow(clippy::nursery)]
#![allow(clippy::manual_range_patterns)]
#![allow(clippy::needless_return)]
#![allow(clippy::let_unit_value)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::eq_op)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::cognitive_complexity)]
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::exhaustive_enums)]

mod array_opcodes;
mod code_decode_isa_extension_none;
#[cfg(feature = "R3000GTE")]
mod code_decode_isa_extension_r3000gte;
#[cfg(feature = "R4000ALLEGREX")]
mod code_decode_isa_extension_r4000allegrex;
#[cfg(feature = "R5900EE")]
mod code_decode_isa_extension_r5900ee;
#[cfg(feature = "RSP")]
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
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_m2x2;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_m3x3;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_m4x4;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_prefix_dst;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_prefix_src;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_s;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_v2d;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_v3d;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_v4d;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_vcond;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_vconstant;
#[cfg(feature = "R4000ALLEGREX")]
mod enum_register_r4000allegrex_vfpucontrol;
#[cfg(feature = "R5900EE")]
mod enum_register_r5900ee_vf;
#[cfg(feature = "R5900EE")]
mod enum_register_r5900ee_vi;
#[cfg(feature = "RSP")]
mod enum_register_rsp_cop0;
#[cfg(feature = "RSP")]
mod enum_register_rsp_cop2;
#[cfg(feature = "RSP")]
mod enum_register_rsp_cop2_control;
#[cfg(feature = "RSP")]
mod enum_register_rsp_vector;

pub use array_opcodes::OPCODES;
pub use enum_abi::Abi;
pub use enum_access_type::{AccessType, ACCESS_TYPES};
pub use enum_instr_suffix::InstrSuffix;
pub use enum_isa_extension::IsaExtension;
pub use enum_isa_version::IsaVersion;
pub use enum_opcode::Opcode;
pub use enum_opcode_category::OpcodeCategory;
pub use enum_opcode_category::OPCODE_CATEGORIES;
pub use enum_operand::{Operand, ValuedOperand, OPERANDS};
pub use enum_register_cop0::{Cop0, COP0};
pub use enum_register_cop0control::{Cop0Control, COP0_CONTROL};
pub use enum_register_cop1::{Cop1, COP1};
pub use enum_register_cop1control::{Cop1Control, COP1_CONTROL};
pub use enum_register_cop2::{Cop2, COP2};
pub use enum_register_cop2control::{Cop2Control, COP2_CONTROL};
pub use enum_register_gpr::{Gpr, GPR};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_m2x2::{R4000AllegrexM2x2, R4000ALLEGREX_M2X2};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_m3x3::{R4000AllegrexM3x3, R4000ALLEGREX_M3X3};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_m4x4::{R4000AllegrexM4x4, R4000ALLEGREX_M4X4};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_prefix_dst::{
    R4000AllegrexPrefixDst, R4000ALLEGREX_PREFIX_DST,
};
#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_prefix_src::{
    R4000AllegrexPrefixSrc, R4000ALLEGREX_PREFIX_SRC,
};
#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_s::{R4000AllegrexS, R4000ALLEGREX_S};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_v2d::{R4000AllegrexV2D, R4000ALLEGREX_V2D};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_v3d::{R4000AllegrexV3D, R4000ALLEGREX_V3D};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_v4d::{R4000AllegrexV4D, R4000ALLEGREX_V4D};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_vcond::{R4000AllegrexVCond, R4000ALLEGREX_VCOND};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_vconstant::{R4000AllegrexVConstant, R4000ALLEGREX_VCONSTANT};

#[cfg(feature = "R4000ALLEGREX")]
pub use enum_register_r4000allegrex_vfpucontrol::{
    R4000AllegrexVfpuControl, R4000ALLEGREX_VFPUCONTROL,
};
#[cfg(feature = "R5900EE")]
pub use enum_register_r5900ee_vf::{R5900EEVF, R5900EE_VF};
#[cfg(feature = "R5900EE")]
pub use enum_register_r5900ee_vi::{R5900EEVI, R5900EE_VI};
#[cfg(feature = "RSP")]
pub use enum_register_rsp_cop0::{RspCop0, RSP_COP0};
#[cfg(feature = "RSP")]
pub use enum_register_rsp_cop2::{RspCop2, RSP_COP2};
#[cfg(feature = "RSP")]
pub use enum_register_rsp_cop2_control::{RspCop2Control, RSP_COP2_CONTROL};
#[cfg(feature = "RSP")]
pub use enum_register_rsp_vector::{RspVector, RSP_VECTOR};
