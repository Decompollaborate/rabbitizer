/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use crate::operands::{operand_display, OperandDisplay};
use crate::registers_meta::Register;

impl<T> OperandDisplay<'_, '_, T>
where
    T: fmt::Display,
{
    #[allow(non_snake_case)]
    pub(crate) fn display_r5900_I(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if myself.display_flags.r5900_use_dollar() {
            write!(f, "$I")
        } else {
            write!(f, "I")
        }
    }
    #[allow(non_snake_case)]
    pub(crate) fn display_r5900_Q(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if myself.display_flags.r5900_use_dollar() {
            write!(f, "$Q")
        } else {
            write!(f, "Q")
        }
    }
    #[allow(non_snake_case)]
    pub(crate) fn display_r5900_R(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if myself.display_flags.r5900_use_dollar() {
            write!(f, "$R")
        } else {
            write!(f, "R")
        }
    }
    #[allow(non_snake_case)]
    pub(crate) fn display_r5900_ACC(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if myself.display_flags.r5900_use_dollar() {
            write!(f, "$ACC")
        } else {
            write!(f, "ACC")
        }
    }

    pub(crate) fn display_r5900_immediate5(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.field_r5900_immediate5_unchecked() as i32;

            operand_display::display_signed_imm(s, f, myself.display_flags)
        })
    }
    pub(crate) fn display_r5900_immediate15(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_imm_override_or(myself, f, |myself, f| {
            let instr = myself.instr;
            let s = instr.field_r5900_immediate15_unchecked() as i32 * 8;

            operand_display::display_signed_imm(s, f, myself.display_flags)
        })
    }

    pub(crate) fn display_r5900_vfs(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r5900_vfs_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r5900_vft(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r5900_vft_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r5900_vfd(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r5900_vfd_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    pub(crate) fn display_r5900_vis(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r5900_vis_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r5900_vit(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r5900_vit_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }
    pub(crate) fn display_r5900_vid(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let reg = instr.field_r5900_vid_unchecked();
        let s = reg.either_name(instr.abi(), myself.display_flags.named_registers());

        write!(f, "{}", s)
    }

    #[allow(non_snake_case)]
    pub(crate) fn display_r5900_ACCxyzw(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_r5900_ACC(myself, f)?;

        /*
        let instr = myself.instr;
        if instr.field_r5900_xyzw_x_unchecked() {
            write!(f, "x")?;
        }
        if instr.field_r5900_xyzw_y_unchecked() {
            write!(f, "y")?;
        }
        if instr.field_r5900_xyzw_z_unchecked() {
            write!(f, "z")?;
        }
        if instr.field_r5900_xyzw_w_unchecked() {
            write!(f, "w")?;
        }
        */

        Ok(())
    }

    pub(crate) fn display_r5900_vfsxyzw(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_r5900_vfs(myself, f)?;

        /*
        let instr = myself.instr;
        if instr.field_r5900_xyzw_x_unchecked() {
            write!(f, "x")?;
        }
        if instr.field_r5900_xyzw_y_unchecked() {
            write!(f, "y")?;
        }
        if instr.field_r5900_xyzw_z_unchecked() {
            write!(f, "z")?;
        }
        if instr.field_r5900_xyzw_w_unchecked() {
            write!(f, "w")?;
        }
        */

        Ok(())
    }
    pub(crate) fn display_r5900_vftxyzw(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_r5900_vft(myself, f)?;

        /*
        let instr = myself.instr;
        if instr.field_r5900_xyzw_x_unchecked() {
            write!(f, "x")?;
        }
        if instr.field_r5900_xyzw_y_unchecked() {
            write!(f, "y")?;
        }
        if instr.field_r5900_xyzw_z_unchecked() {
            write!(f, "z")?;
        }
        if instr.field_r5900_xyzw_w_unchecked() {
            write!(f, "w")?;
        }
        */

        Ok(())
    }
    pub(crate) fn display_r5900_vfdxyzw(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        Self::display_r5900_vfd(myself, f)?;

        /*
        let instr = myself.instr;
        if instr.field_r5900_xyzw_x_unchecked() {
            write!(f, "x")?;
        }
        if instr.field_r5900_xyzw_y_unchecked() {
            write!(f, "y")?;
        }
        if instr.field_r5900_xyzw_z_unchecked() {
            write!(f, "z")?;
        }
        if instr.field_r5900_xyzw_w_unchecked() {
            write!(f, "w")?;
        }
        */

        Ok(())
    }
    pub(crate) fn display_r5900_vftn(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let n = instr.field_r5900_n_unchecked();

        Self::display_r5900_vft(myself, f)?;
        write!(f, "{}", ['x', 'y', 'z', 'w'][n as usize])
    }
    pub(crate) fn display_r5900_vfsl(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let n = instr.field_r5900_l_unchecked();

        Self::display_r5900_vfs(myself, f)?;
        write!(f, "{}", ['x', 'y', 'z', 'w'][n as usize])
    }
    pub(crate) fn display_r5900_vftm(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let instr = myself.instr;
        let n = instr.field_r5900_m_unchecked();

        Self::display_r5900_vft(myself, f)?;
        write!(f, "{}", ['x', 'y', 'z', 'w'][n as usize])
    }

    pub(crate) fn display_r5900_vis_predecr(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "(--")?;
        Self::display_r5900_vis(myself, f)?;
        write!(f, ")")
    }
    pub(crate) fn display_r5900_vit_predecr(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "(--")?;
        Self::display_r5900_vit(myself, f)?;
        write!(f, ")")
    }
    pub(crate) fn display_r5900_vis_postincr(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "(")?;
        Self::display_r5900_vis(myself, f)?;
        write!(f, "++)")
    }
    pub(crate) fn display_r5900_vit_postincr(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "(")?;
        Self::display_r5900_vit(myself, f)?;
        write!(f, "++)")
    }
    pub(crate) fn display_r5900_vis_parenthesis(
        myself: &OperandDisplay<T>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "(")?;
        Self::display_r5900_vis(myself, f)?;
        write!(f, ")")
    }
}
