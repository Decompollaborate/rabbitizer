/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::operands::OperandDisplay;

impl<T> OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    pub(crate) fn display_r3000gte_sf(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().r3000gte_sf_impl();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r3000gte_mx(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().r3000gte_mx_impl();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r3000gte_v(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().r3000gte_v_impl();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r3000gte_cv(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().r3000gte_cv_impl();

        write!(f, "{}", s)
    }
    pub(crate) fn display_r3000gte_lm(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().r3000gte_lm_impl();

        write!(f, "{}", s)
    }
}
