/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod register_cop0;
mod register_cop0control;
mod register_cop1;
mod register_cop1control;
mod register_cop2;
mod register_cop2control;
mod register_gpr;
mod register_r4000allegrex_m2x2;
mod register_r4000allegrex_m3x3;
mod register_r4000allegrex_m4x4;
mod register_r4000allegrex_prefix_dst;
mod register_r4000allegrex_prefix_src;
mod register_r4000allegrex_s;
mod register_r4000allegrex_v2d;
mod register_r4000allegrex_v3d;
mod register_r4000allegrex_v4d;
mod register_r4000allegrex_vcond;
mod register_r4000allegrex_vconstant;
mod register_r4000allegrex_vfpucontrol;
mod register_r5900ee_vf;
mod register_r5900ee_vi;
mod register_rsp_cop0;
mod register_rsp_cop2;
mod register_rsp_cop2_control;
mod register_rsp_vector;

pub use crate::generated::Cop0;
pub use crate::generated::Cop0Control;
pub use crate::generated::Cop1;
pub use crate::generated::Cop1Control;
pub use crate::generated::Cop2;
pub use crate::generated::Cop2Control;
pub use crate::generated::Gpr;
pub use crate::generated::R4000AllegrexM2x2;
pub use crate::generated::R4000AllegrexM3x3;
pub use crate::generated::R4000AllegrexM4x4;
pub use crate::generated::R4000AllegrexPrefixDst;
pub use crate::generated::R4000AllegrexPrefixSrc;
pub use crate::generated::R4000AllegrexS;
pub use crate::generated::R4000AllegrexV2D;
pub use crate::generated::R4000AllegrexV3D;
pub use crate::generated::R4000AllegrexV4D;
pub use crate::generated::R4000AllegrexVCond;
pub use crate::generated::R4000AllegrexVConstant;
pub use crate::generated::R4000AllegrexVfpuControl;
pub use crate::generated::RspCop0;
pub use crate::generated::RspCop2;
pub use crate::generated::RspCop2Control;
pub use crate::generated::RspVector;
pub use crate::generated::R5900EEVF;
pub use crate::generated::R5900EEVI;
