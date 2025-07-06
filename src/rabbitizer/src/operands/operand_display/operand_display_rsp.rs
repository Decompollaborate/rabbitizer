/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::operands::OperandDisplay;
use crate::registers_meta::Register;

impl<T> OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    pub(crate) fn display_rsp_cop0d(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rsp_cop0d_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_rsp_cop0(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_rsp_cop2cd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rsp_cop2cd_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }

    pub(crate) fn display_rsp_vs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rsp_vs_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_rsp_vt(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rsp_vt_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }
    pub(crate) fn display_rsp_vd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field().rsp_vd_impl();
        let s = reg.either_name(
            instr.abi(),
            myself.display_flags.named_registers(),
            !myself.display_flags.use_dollar(),
        );

        write!(f, "{}", s)
    }

    pub(crate) fn display_rsp_vt_elementhigh(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_rsp_vt(myself, f)?;

        let element = myself.instr.field().rsp_elementhigh_impl();
        if element != 0 {
            if (element & 0x8) == 0x8 {
                write!(f, "[{}]", element & 7)
            } else if (element & 0xC) == 0x4 {
                write!(f, "[{}h]", element & (!0xC))
            } else if (element & 0xE) == 0x2 {
                write!(f, "[{}q]", element & (!0xE))
            } else {
                write!(f, "[{}]", element)
            }
        } else {
            Ok(())
        }
    }
    pub(crate) fn display_rsp_vt_elementlow(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_rsp_vt(myself, f)?;

        let element = myself.instr.field().rsp_elementlow_impl();
        write!(f, "[{}]", element)
    }
    pub(crate) fn display_rsp_vd_de(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_rsp_vd(myself, f)?;

        let element = myself.instr.field().rsp_de_impl();
        if element != 0 {
            if (element & 0x8) == 0x8 {
                write!(f, "[{}]", element & (!0x8))
            } else if (element & 0xC) == 0x4 {
                write!(f, "[{}h]", element & (!0xC))
            } else if (element & 0xE) == 0x2 {
                write!(f, "[{}q]", element & (!0xE))
            } else {
                write!(f, "[{}]", element)
            }
        } else {
            Ok(())
        }
    }
    pub(crate) fn display_rsp_vs_index(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.field().rsp_index_impl();

        Self::display_rsp_vs(myself, f)?;
        write!(f, "[{}]", s)
    }
    pub(crate) fn display_rsp_offset_rs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let s = instr.get_processed_rsp_offset_impl();

        write!(f, "0x{:X}", s)?;
        write!(f, "(")?;
        Self::display_core_rs(myself, f)?;
        write!(f, ")")
    }
}
