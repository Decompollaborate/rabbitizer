/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#[allow(clippy::module_inception)]
mod vram;
mod vram_offset;

pub use vram::Vram;
pub use vram_offset::VramOffset;
