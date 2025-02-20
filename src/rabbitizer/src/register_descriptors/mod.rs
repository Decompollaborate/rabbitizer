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
pub use crate::generated::R4000ALLEGREX_M2X2;
pub use crate::generated::R4000ALLEGREX_M3X3;
pub use crate::generated::R4000ALLEGREX_M4X4;
pub use crate::generated::R4000ALLEGREX_PREFIX_DST;
pub use crate::generated::R4000ALLEGREX_PREFIX_SRC;
pub use crate::generated::R4000ALLEGREX_S;
pub use crate::generated::R4000ALLEGREX_V2D;
pub use crate::generated::R4000ALLEGREX_V3D;
pub use crate::generated::R4000ALLEGREX_V4D;
pub use crate::generated::R4000ALLEGREX_VCOND;
pub use crate::generated::R4000ALLEGREX_VCONSTANT;
pub use crate::generated::R4000ALLEGREX_VFPUCONTROL;
pub use crate::generated::R5900EE_VF;
pub use crate::generated::R5900EE_VI;
pub use crate::generated::RSP_COP0;
pub use crate::generated::RSP_COP2;
pub use crate::generated::RSP_COP2_CONTROL;
pub use crate::generated::RSP_VECTOR;
