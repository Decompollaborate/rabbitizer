/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

mod register_iterator;
mod register_trait;

pub use register_iterator::RegisterIterator;
pub(crate) use register_trait::R4000AllegrexVectorRegister;
pub use register_trait::Register;
