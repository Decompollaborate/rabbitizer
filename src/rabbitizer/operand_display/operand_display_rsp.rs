/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::OperandDisplay;

impl<'ins, 'imm, 'flg> OperandDisplay<'ins, 'imm, 'flg> {
    pub(crate) fn display_rsp_cop0d(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_cop2t(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_cop2cd(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_vs(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_vt(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_vd(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }

    pub(crate) fn display_rsp_vt_elementhigh(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_vt_elementlow(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_vd_de(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_vs_index(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
    pub(crate) fn display_rsp_offset_rs(
        _myself: &OperandDisplay,
        _f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Ok(()) // todo!()
    }
}
