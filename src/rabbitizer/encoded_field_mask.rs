/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use bitflags::bitflags;

use crate::utils;

bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EncodedFieldMask: u32 {
        const opcode = utils::bitmask(26, 6);
        const rs = utils::bitmask(21, 5);

        const rt = utils::bitmask(16,  5);
        const rd = utils::bitmask(11,  5);
        const sa = utils::bitmask( 6,  5);
        const function = utils::bitmask( 0,  6);

        const cop0d = utils::bitmask(11,  5);

        const instr_index = utils::bitmask(  0, 26);
        const immediate = utils::bitmask(  0, 16);

        const fs = utils::bitmask( 11,  5);
        const ft = utils::bitmask( 16,  5);
        const fd = utils::bitmask(  6,  5);
        const cop1cs = utils::bitmask( 11,  5);

        const op = utils::bitmask( 16,  5);
        const hint = utils::bitmask( 16,  5);

        const code = utils::bitmask(  6, 20);
        const code_upper = utils::bitmask( 16, 10);
        const code_lower = utils::bitmask(  6, 10);

        const copraw = utils::bitmask(  0, 25);

        const fmt = utils::bitmask( 21,  5);
        const fc = utils::bitmask(  4,  2);
        const cond = utils::bitmask(  0,  4);

        const cop2t = utils::bitmask( 16,  5);
        const cop2cd = utils::bitmask( 11,  5);

        const tf = utils::bitmask( 16,  1);
        const nd = utils::bitmask( 17,  1);
        const bc_fmt = utils::bitmask( 16,  5);

        const stype = utils::bitmask(  6,  5);

        /* r3000gte */

        const r3000gte_fake_opcode = utils::bitmask( 20,  5);
        const r3000gte_sf = utils::bitmask( 19,  1);
        const r3000gte_mx = utils::bitmask( 17,  2);
        const r3000gte_v = utils::bitmask( 15,  2);
        const r3000gte_cv = utils::bitmask( 13,  2);
        const r3000gte_lm = utils::bitmask( 10,  1);

        /* r5000 */

        const r5900_cop2_highbit = utils::bitmask(25,  1);
        const r5900_cop2_nohighbit_fmt = utils::bitmask(21,  4);

        const r5900_mmi_function = utils::bitmask(  6,  5);
        const r5900_fhi_flo = utils::bitmask(  6,  5) | utils::bitmask(  0,  2);
        const r5900_viwr_fhilo = utils::bitmask( 21,  4) | utils::bitmask(  0,  2);

        const r5900_vfs = utils::bitmask( 11,  5);
        const r5900_vft = utils::bitmask( 16,  5);
        const r5900_vfd = utils::bitmask(  6,  5);

        const r5900_xyzw_x = utils::bitmask( 24,  1);
        const r5900_xyzw_y = utils::bitmask( 23,  1);
        const r5900_xyzw_z = utils::bitmask( 22,  1);
        const r5900_xyzw_w = utils::bitmask( 21,  1);
        const r5900_n = utils::bitmask(  0,  2);
        const r5900_l = utils::bitmask( 21,  2);
        const r5900_m = utils::bitmask( 23,  2);

        const r5900_vis = utils::bitmask( 11,  5);
        const r5900_vit = utils::bitmask( 16,  5);
        const r5900_vid = utils::bitmask(  6,  5);

        const r5900_imm5 = utils::bitmask(  6,  5);
        const r5900_imm15 = utils::bitmask(  6, 15);

        const _ = !0;
    }
}

impl EncodedFieldMask {
    #[must_use]
    pub const fn get_shifted(&self, value: u32) -> u32 {
        let bits = self.bits();

        (value & bits) >> bits.trailing_zeros()
    }

    #[must_use]
    pub const fn mask_value(&self, value: u32) -> Self {
        let bits = self.bits();

        Self::from_bits_retain(value & bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bits() {
        // TODO: the rest of cases
        // TODO: can this be checked at compile time?
        assert_eq!(
            0b11111100000000000000000000000000,
            EncodedFieldMask::opcode.bits()
        );
        assert_eq!(
            0b00000011111000000000000000000000,
            EncodedFieldMask::rs.bits()
        );
    }
}
