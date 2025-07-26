/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![doc = include_str!("../README.md")]
/*
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::too_many_lines)] // ?
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cast_lossless)] // maybe warn?
#![allow(clippy::match_same_arms)] // maybe warn?
#![allow(clippy::trivially_copy_pass_by_ref)] // ?
#![allow(clippy::unused_self)]
#![allow(clippy::missing_panics_doc)] // maybe warn?
#![allow(clippy::doc_markdown)] // ?
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::missing_errors_doc)] // maybe warn?
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::unreadable_literal)]
*/

/*
#![warn(clippy::restriction)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::same_name_method)]
#![allow(clippy::min_ident_chars)]
#![allow(clippy::as_conversions)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::implicit_return)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::missing_inline_in_public_items)] // !
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::missing_docs_in_private_items)] // warn!
#![allow(clippy::indexing_slicing)] // TODO: consider
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::allow_attributes)] // TODO: consider
#![allow(clippy::todo)] // TODO: consider
#![allow(clippy::pub_use)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::shadow_unrelated)] // TODO: consider
#![allow(clippy::wildcard_enum_match_arm)]
#![allow(clippy::if_then_some_else_none)] // TODO: consider
#![allow(clippy::unwrap_used)] // TODO: consider
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::single_char_lifetime_names)] // TODO: consider
#![allow(clippy::field_scoped_visibility_modifiers)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::empty_enum_variants_with_brackets)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::error_impl_error)]
#![allow(clippy::self_named_module_files)]
*/

/*
#![warn(clippy::nursery)]
#![allow(clippy::redundant_pub_crate)]
*/
#![deny(unreachable_patterns)]
#![allow(clippy::exhaustive_enums)]
#![warn(clippy::use_self)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::pattern_type_mismatch)]
// #![warn(clippy::missing_inline_in_public_items)] // TODO
// #![warn(missing_docs)] // TODO: change to `deny`
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(clippy::doc_markdown)] // ?
// #![warn(clippy::missing_errors_doc)]
#![allow(clippy::pub_with_shorthand)]
#![warn(clippy::pub_without_shorthand)]
// #![warn(clippy::option_if_let_else)] // It can get kinda ugly. Reconsider later
#![warn(clippy::option_map_or_none)]
#![warn(clippy::bind_instead_of_map)]
#![warn(clippy::cognitive_complexity)] // Maybe remove in the future (?)
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::ref_option)]
#![warn(clippy::manual_let_else)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::manual_non_exhaustive)]
//
#![cfg_attr(not(feature = "std"), no_std)]
//
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

mod generated;

pub mod abi;
pub mod access_type;
pub mod access_type_descriptor;
pub mod display_flags;
mod encoded_field_mask;
pub mod instr;
pub mod isa;
pub mod opcodes;
pub mod operands;
pub mod register_descriptors;
pub mod registers;
pub mod registers_meta;
pub mod vram;

#[cfg(feature = "encoder")]
#[cfg_attr(docsrs, doc(cfg(feature = "encoder")))]
pub mod encoder;

mod utils;

pub use display_flags::InstructionDisplayFlags;
pub use instr::{Instruction, InstructionFlags};
pub use isa::{IsaExtension, IsaVersion};
pub use vram::Vram;

#[cfg(feature = "pyo3")]
#[pymodule]
fn rabbitizer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<display_flags::InstructionDisplayFlags>()?;

    m.add_class::<instr::InstructionFlags>()?;

    m.add_class::<isa::IsaVersion>()?;
    m.add_class::<isa::IsaExtension>()?;

    m.add_class::<vram::Vram>()?;

    Ok(())
}
