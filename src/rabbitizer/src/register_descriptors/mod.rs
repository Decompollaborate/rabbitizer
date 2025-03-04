/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod register_descriptor;

pub use register_descriptor::RegisterDescriptor;

pub use crate::generated::COP0;
pub use crate::generated::COP0_CONTROL;
pub use crate::generated::COP1;
pub use crate::generated::COP1_CONTROL;
pub use crate::generated::COP2;
pub use crate::generated::COP2_CONTROL;
pub use crate::generated::GPR;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_M2X2;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_M3X3;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_M4X4;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_PREFIX_DST;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_PREFIX_SRC;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_S;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_V2D;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_V3D;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_V4D;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_VCOND;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_VCONSTANT;
#[cfg(feature = "R4000ALLEGREX")]
pub use crate::generated::R4000ALLEGREX_VFPUCONTROL;
#[cfg(feature = "R5900EE")]
pub use crate::generated::R5900EE_VF;
#[cfg(feature = "R5900EE")]
pub use crate::generated::R5900EE_VI;
#[cfg(feature = "RSP")]
pub use crate::generated::RSP_COP0;
#[cfg(feature = "RSP")]
pub use crate::generated::RSP_COP2;
#[cfg(feature = "RSP")]
pub use crate::generated::RSP_COP2_CONTROL;
#[cfg(feature = "RSP")]
pub use crate::generated::RSP_VECTOR;
