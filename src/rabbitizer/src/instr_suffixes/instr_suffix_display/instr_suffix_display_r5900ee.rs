/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

use super::InstrSuffixDisplay;

impl InstrSuffixDisplay<'_, '_> {
    #[allow(non_snake_case)]
    pub(crate) fn display_r5900ee_xyzw(
        myself: &InstrSuffixDisplay,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let field = myself.instr.field();

        let x = field.r5900ee_xyzw_x_impl();
        let y = field.r5900ee_xyzw_y_impl();
        let z = field.r5900ee_xyzw_z_impl();
        let w = field.r5900ee_xyzw_w_impl();

        if x || y || z || w {
            write!(f, ".")?;
        }

        if x {
            write!(f, "x")?;
        }
        if y {
            write!(f, "y")?;
        }
        if z {
            write!(f, "z")?;
        }
        if w {
            write!(f, "w")?;
        }

        Ok(())
    }
}
