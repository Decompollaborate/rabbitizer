/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod int_register_conversion_error;
mod register_iterator;
mod register_trait;

pub use int_register_conversion_error::IntRegisterConversionError;
pub use register_iterator::RegisterIterator;
#[cfg(feature = "R4000ALLEGREX")]
pub(crate) use register_trait::R4000AllegrexVectorRegister;
pub use register_trait::Register;
