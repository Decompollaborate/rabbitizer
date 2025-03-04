/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod register_cop0;
mod register_cop0control;
mod register_cop1;
mod register_cop1control;
mod register_cop2;
mod register_cop2control;
mod register_gpr;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_m2x2;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_m3x3;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_m4x4;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_prefix_dst;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_prefix_src;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_s;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_v2d;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_v3d;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_v4d;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_vcond;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_vconstant;
#[cfg(feature = "R4000ALLEGREX")]
mod register_r4000allegrex_vfpucontrol;
#[cfg(feature = "R5900EE")]
mod register_r5900ee_vf;
#[cfg(feature = "R5900EE")]
mod register_r5900ee_vi;
#[cfg(feature = "RSP")]
mod register_rsp_cop0;
#[cfg(feature = "RSP")]
mod register_rsp_cop2;
#[cfg(feature = "RSP")]
mod register_rsp_cop2_control;
#[cfg(feature = "RSP")]
mod register_rsp_vector;

pub use crate::generated::Cop0;
pub use crate::generated::Cop0Control;
pub use crate::generated::Cop1;
pub use crate::generated::Cop1Control;
pub use crate::generated::Cop2;
pub use crate::generated::Cop2Control;
pub use crate::generated::Gpr;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexM2x2;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexM3x3;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexM4x4;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexPrefixDst;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexPrefixSrc;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexS;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexV2D;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexV3D;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexV4D;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexVCond;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexVConstant;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000AllegrexVfpuControl;
#[cfg(feature = "RSP")]
pub use crate::generated::RspCop0;
#[cfg(feature = "RSP")]
pub use crate::generated::RspCop2;
#[cfg(feature = "RSP")]
pub use crate::generated::RspCop2Control;
#[cfg(feature = "RSP")]
pub use crate::generated::RspVector;
#[cfg(feature = "R5900EE")]
pub use crate::generated::R5900EEVF;
#[cfg(feature = "R5900EE")]
pub use crate::generated::R5900EEVI;
