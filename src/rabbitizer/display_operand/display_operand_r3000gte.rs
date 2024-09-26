/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::DisplayOperand;

impl<'ins, 'imm> DisplayOperand<'ins, 'imm> {
    pub(crate) fn display_r3000gte_sf(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_r3000gte_mx(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_r3000gte_v(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_r3000gte_cv(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
    pub(crate) fn display_r3000gte_lm(
        _myself: &DisplayOperand,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        todo!()
    }
}
