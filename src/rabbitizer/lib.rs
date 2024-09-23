/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![warn(clippy::exhaustive_enums)]
#![warn(clippy::use_self)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::pattern_type_mismatch)]
// TODO: consider adding clippy::missing_inline_in_public_items
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate std;

mod generated;

mod abi;
mod access_type;
mod encoded_field_mask;
mod flags;
mod instr_suffix;
mod instr_type;
mod instruction;
mod isa_extension;
mod isa_version;
mod opcode;
mod opcode_decoder;
mod opcode_descriptor;
mod operand;
mod operand_descriptor;
mod register_cop0;
mod register_cop1;
mod register_cop1control;
mod register_cop2;
mod register_descriptor;
mod register_gpr;
mod register_r4000allegrex_m2x2;
mod register_r4000allegrex_m3x3;
mod register_r4000allegrex_m4x4;
mod register_r4000allegrex_s;
mod register_r4000allegrex_v2d;
mod register_r4000allegrex_v3d;
mod register_r4000allegrex_v4d;
mod register_r4000allegrex_vconstant;
mod register_r4000allegrex_vfpucontrol;
mod register_r5900allegrex_vf;
mod register_r5900allegrex_vi;
mod register_rsp_cop0;
mod register_rsp_cop2;
mod register_rsp_cop2_control;
mod register_rsp_gpr;
mod register_rsp_vector;
pub mod traits;
mod valued_operand;

mod utils;

pub use generated::Abi;
pub use generated::AccessType;
pub use generated::InstrSuffix;
pub use generated::IsaExtension;
pub use generated::IsaVersion;
pub use generated::Opcode;
pub use generated::OpcodeCategory;
pub use generated::Operand;
#[allow(deprecated)]
pub use instr_type::InstrType;

pub use generated::OPCODES;
pub use generated::OPERANDS;

pub use encoded_field_mask::EncodedFieldMask;
pub use instruction::Instruction;
pub(crate) use opcode_decoder::OpcodeDecoder;
pub use opcode_descriptor::OpcodeDescriptor;
pub use operand_descriptor::OperandDescriptor;
pub use register_descriptor::RegisterDescriptor;
pub use valued_operand::ValuedOperand;

pub use flags::DecodingFlags;
pub use flags::InstructionFlags;

pub mod registers {
    use crate::generated;

    pub use generated::Cop0;
    pub use generated::Cop1;
    pub use generated::Cop1Control;
    pub use generated::Cop2;
    pub use generated::Gpr;
    pub use generated::R4000AllegrexM2x2;
    pub use generated::R4000AllegrexM3x3;
    pub use generated::R4000AllegrexM4x4;
    pub use generated::R4000AllegrexS;
    pub use generated::R4000AllegrexV2D;
    pub use generated::R4000AllegrexV3D;
    pub use generated::R4000AllegrexV4D;
    pub use generated::R4000AllegrexVConstant;
    pub use generated::R4000AllegrexVfpuControl;
    pub use generated::RspCop0;
    pub use generated::RspCop2;
    pub use generated::RspCop2Control;
    pub use generated::RspGpr;
    pub use generated::RspVector;
    pub use generated::R5900VF;
    pub use generated::R5900VI;
}

pub mod register_descriptors {
    use crate::generated;

    pub use generated::COP0;
    pub use generated::COP1;
    pub use generated::COP1CONTROL;
    pub use generated::COP2;
    pub use generated::GPR;
    pub use generated::R4000ALLEGREX_M2X2;
    pub use generated::R4000ALLEGREX_M3X3;
    pub use generated::R4000ALLEGREX_M4X4;
    pub use generated::R4000ALLEGREX_S;
    pub use generated::R4000ALLEGREX_V2D;
    pub use generated::R4000ALLEGREX_V3D;
    pub use generated::R4000ALLEGREX_V4D;
    pub use generated::R4000ALLEGREX_VCONSTANT;
    pub use generated::R4000ALLEGREX_VFPUCONTROL;
    pub use generated::R5900_VF;
    pub use generated::R5900_VI;
    pub use generated::RSP_COP0;
    pub use generated::RSP_COP2;
    pub use generated::RSP_COP2_CONTROL;
    pub use generated::RSP_GPR;
    pub use generated::RSP_VECTOR;
}

mod tests;
