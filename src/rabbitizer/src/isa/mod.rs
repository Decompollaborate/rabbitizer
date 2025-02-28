/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[cfg(any(
    feature = "RSP",
    feature = "R3000GTE",
    feature = "R4000ALLEGREX",
    feature = "R5900EE",
))]
mod isa_extension;
mod isa_version;

pub use crate::generated::IsaExtension;
pub use crate::generated::IsaVersion;
