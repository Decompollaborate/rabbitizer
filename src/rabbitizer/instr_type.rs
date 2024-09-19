/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// #[deprecated(since="1.3.0")]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::min_ident_chars)]
#[non_exhaustive]
pub enum InstrType {
    UNKNOWN,
    J,
    I,
    R,
    REGIMM,
}

#[allow(deprecated)]
impl InstrType {
    #[must_use]
    pub const fn default() -> Self {
        Self::UNKNOWN
    }
}

#[allow(deprecated)]
impl Default for InstrType {
    fn default() -> Self {
        Self::default()
    }
}
