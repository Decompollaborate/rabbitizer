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
mod traits;

mod utils;

pub use generated::Abi;
pub use generated::AccessType;
pub use generated::InstrSuffix;
pub use generated::IsaExtension;
pub use generated::IsaVersion;
pub use generated::Opcode;
pub use generated::OpcodeCategory;
pub use generated::Operand;
pub use generated::RegisterCop0;
pub use generated::RegisterCop1;
pub use generated::RegisterCop1Control;
pub use generated::RegisterCop2;
pub use generated::RegisterGpr;
pub use generated::RegisterR4000AllegrexM2x2;
pub use generated::RegisterR4000AllegrexM3x3;
pub use generated::RegisterR4000AllegrexM4x4;
pub use generated::RegisterR4000AllegrexS;
pub use generated::RegisterR4000AllegrexV2D;
pub use generated::RegisterR4000AllegrexV3D;
pub use generated::RegisterR4000AllegrexV4D;
pub use generated::RegisterR4000AllegrexVConstant;
pub use generated::RegisterR4000AllegrexVfpuControl;
pub use generated::RegisterR5900VF;
pub use generated::RegisterR5900VI;
pub use generated::RegisterRspCop0;
pub use generated::RegisterRspCop2;
pub use generated::RegisterRspCop2Control;
pub use generated::RegisterRspGpr;
pub use generated::RegisterRspVector;
pub use generated::COP0_REGISTERS;
pub use generated::COP1CONTROL_REGISTERS;
pub use generated::COP1_REGISTERS;
pub use generated::COP2_REGISTERS;
pub use generated::GPR_REGISTERS;
pub use generated::R4000ALLEGREX_M2X2_REGISTERS;
pub use generated::R4000ALLEGREX_M3X3_REGISTERS;
pub use generated::R4000ALLEGREX_M4X4_REGISTERS;
pub use generated::R4000ALLEGREX_S_REGISTERS;
pub use generated::R4000ALLEGREX_V2D_REGISTERS;
pub use generated::R4000ALLEGREX_V3D_REGISTERS;
pub use generated::R4000ALLEGREX_V4D_REGISTERS;
pub use generated::R4000ALLEGREX_VCONSTANT_REGISTERS;
pub use generated::R4000ALLEGREX_VFPUCONTROL_REGISTERS;
pub use generated::R5900_VF_REGISTERS;
pub use generated::R5900_VI_REGISTERS;
pub use generated::RSP_COP0_REGISTERS;
pub use generated::RSP_COP2_CONTROL_REGISTERS;
pub use generated::RSP_COP2_REGISTERS;
pub use generated::RSP_GPR_REGISTERS;
pub use generated::RSP_VECTOR_REGISTERS;
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
pub use traits::Register;

pub use flags::DecodingFlags;
pub use flags::InstructionFlags;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j() {
        assert!(OPCODES[Opcode::cpu_j].is_jump);
        assert!(Opcode::cpu_j.get_descriptor().is_jump);
        assert!(OPCODES[Opcode::cpu_j].is_jump_with_address);
        assert!(Opcode::cpu_j.get_descriptor().is_jump_with_address);
        assert!(!OPCODES[Opcode::cpu_j].is_branch);
        assert!(!Opcode::cpu_j.get_descriptor().is_branch);
    }

    #[test]
    fn test_addiu_operands() {
        let mut operands = Opcode::cpu_addiu.get_descriptor().operands_iter();

        assert_eq!(operands.next(), Some(Operand::cpu_rt).as_ref());
        assert_eq!(operands.next(), Some(Operand::cpu_rs).as_ref());
        assert_eq!(operands.next(), Some(Operand::cpu_immediate).as_ref());
        assert_eq!(operands.next(), None);
    }
}
