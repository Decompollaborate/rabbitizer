/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod instr_suffix;
mod instr_suffix_descriptor;
mod instr_suffix_display;

pub use crate::generated::InstrSuffix;
pub use crate::generated::INSTR_SUFFIXES;

pub(crate) use instr_suffix::INSTR_SUFFIX_COUNT;
pub use instr_suffix_descriptor::InstrSuffixDescriptor;
pub use instr_suffix_display::InstrSuffixDisplay;
