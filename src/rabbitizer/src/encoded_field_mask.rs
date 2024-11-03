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
        const cop0cd = utils::bitmask(11,  5);

        const fs = utils::bitmask( 11,  5);
        const ft = utils::bitmask( 16,  5);
        const fd = utils::bitmask(  6,  5);
        const cop1cs = utils::bitmask( 11,  5);

        const cop2t = utils::bitmask( 16,  5);
        const cop2d = utils::bitmask( 11,  5);
        const cop2cd = utils::bitmask( 11,  5);

        const instr_index = utils::bitmask(  0, 26);
        const immediate = utils::bitmask(  0, 16);

        const op = utils::bitmask( 16,  5);
        const hint = utils::bitmask( 16,  5);

        const code = utils::bitmask(  6, 20);
        const code_upper = utils::bitmask( 16, 10);
        const code_lower = utils::bitmask(  6, 10);

        const copraw = utils::bitmask(  0, 25);

        const fmt = utils::bitmask( 21,  5);
        const fc = utils::bitmask(  4,  2);
        const cond = utils::bitmask(  0,  4);

        const tf = utils::bitmask( 16,  1);
        const nd = utils::bitmask( 17,  1);
        const bc_fmt = utils::bitmask( 16,  5);

        const stype = utils::bitmask(  6,  5);


        /* rsp */

        const rsp_vs = utils::bitmask( 11,  5);
        const rsp_vt = utils::bitmask( 16,  5);
        const rsp_vd = utils::bitmask(  6,  5);

        const rsp_elementhigh = utils::bitmask( 21,  4);
        const rsp_elementlow = utils::bitmask(  7,  4);

        const rsp_index = utils::bitmask(  7,  4);
        const rsp_offset = utils::bitmask(  0,  7);
        const rsp_de = utils::bitmask( 11,  5);

        const rsp_vu = utils::bitmask( 25,  1);


        /* r3000gte */

        const r3000gte_fake_opcode = utils::bitmask( 20,  5);
        const r3000gte_sf = utils::bitmask( 19,  1);
        const r3000gte_mx = utils::bitmask( 17,  2);
        const r3000gte_v = utils::bitmask( 15,  2);
        const r3000gte_cv = utils::bitmask( 13,  2);
        const r3000gte_lm = utils::bitmask( 10,  1);

        /* r4000allegrex */

        const r4000allegrex_vs = utils::bitmask(  8,  7);
        const r4000allegrex_vt = utils::bitmask( 16,  7);
        const r4000allegrex_vd = utils::bitmask(  0,  7);

        // Upper and lower bits the other way around for some reason
        const r4000allegrex_vt_imm_upper = utils::bitmask(  0,  2);
        const r4000allegrex_vt_imm_lower = utils::bitmask( 16,  5);
        const r4000allegrex_vt_imm = Self::r4000allegrex_vt_imm_upper.bits() | Self::r4000allegrex_vt_imm_lower.bits();

        const r4000allegrex_vd_imm = utils::bitmask( 16,  7);

        const r4000allegrex_vt_6_imm_upper = utils::bitmask(  0,  1);
        const r4000allegrex_vt_6_imm = Self::r4000allegrex_vt_6_imm_upper.bits() | Self::r4000allegrex_vt_imm_lower.bits();

        const r4000allegrex_cop2cs = utils::bitmask(  8,  7);
        const r4000allegrex_cop2cd = utils::bitmask(  0,  7);

        const r4000allegrex_pos = utils::bitmask(  6,  5);
        const r4000allegrex_size = utils::bitmask( 11,  5);
        const r4000allegrex_size_plus_pos = Self::r4000allegrex_size.bits();

        const r4000allegrex_bc2_fmt = utils::bitmask( 16,  2);
        const r4000allegrex_mxhc2 = utils::bitmask(  7,  1);
        const r4000allegrex_mfhc2_p_fmt = utils::bitmask(  4,  3);
        const r4000allegrex_mfhc2_p_s_fmt = utils::bitmask(  0,  4);

        const r4000allegrex_imm3 = utils::bitmask( 18,  3);
        const r4000allegrex_offset14 = utils::bitmask(  2, 14);
        const r4000allegrex_wb = utils::bitmask(  1,  1);

        const r4000allegrex_tp = utils::bitmask( 15,  1) | utils::bitmask( 7,  1);
        const r4000allegrex_vfpu0_fmt = utils::bitmask( 23,  3) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu4_fmt = utils::bitmask( 24,  2) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu4_fmt0_fmt = utils::bitmask( 19,  5);
        const r4000allegrex_vfpu4_fmt0_fmt0_fmt = utils::bitmask( 16,  3) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu4_fmt2_fmt = utils::bitmask( 21,  3) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu4_fmt2_cndmove_fmt = utils::bitmask( 19,  2) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu5_fmt = utils::bitmask( 23,  3);
        const r4000allegrex_vfpu6_fmt = utils::bitmask( 23,  3) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu6_fmt7_fmt = utils::bitmask( 21,  2) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu6_fmt7_fmt0_fmt = utils::bitmask( 16,  3) | Self::r4000allegrex_tp.bits();
        const r4000allegrex_vfpu7_fmt = utils::bitmask(  0, 26);

        const r4000allegrex_vcmp_cond = utils::bitmask(  0,  4);
        const r4000allegrex_vconstant = utils::bitmask( 16,  5);
        const r4000allegrex_power_of_two = utils::bitmask( 16,  5);
        const r4000allegrex_vfpu_cc_bit = utils::bitmask( 16,  3);
        const r4000allegrex_bn = utils::bitmask( 16,  8);

        const r4000allegrex_intfloat16 = utils::bitmask(  0, 16);
        const r4000allegrex_vrot_code = utils::bitmask( 16,  5);
        const r4000allegrex_wpx = utils::bitmask(  8,  1) | utils::bitmask(  0,  2);
        const r4000allegrex_wpy = utils::bitmask(  9,  1) | utils::bitmask(  2,  2);
        const r4000allegrex_wpz = utils::bitmask( 10,  1) | utils::bitmask(  4,  2);
        const r4000allegrex_wpw = utils::bitmask( 11,  1) | utils::bitmask(  6,  2);
        const r4000allegrex_rpx = utils::bitmask( 16,  1) | utils::bitmask( 12,  1) | utils::bitmask(  8,  1) | utils::bitmask(  0,  2);
        const r4000allegrex_rpy = utils::bitmask( 17,  1) | utils::bitmask( 13,  1) | utils::bitmask(  9,  1) | utils::bitmask(  2,  2);
        const r4000allegrex_rpz = utils::bitmask( 18,  1) | utils::bitmask( 14,  1) | utils::bitmask( 10,  1) | utils::bitmask(  4,  2);
        const r4000allegrex_rpw = utils::bitmask( 19,  1) | utils::bitmask( 15,  1) | utils::bitmask( 11,  1) | utils::bitmask(  6,  2);


        /* r5000 */

        const r5900_cop2_highbit = utils::bitmask(25,  1);
        const r5900_cop2_nohighbit_fmt = utils::bitmask(21,  4);

        const r5900_mmi_function = utils::bitmask(  6,  5);
        const r5900_fhi_flo = utils::bitmask(  6,  5) | utils::bitmask(  0,  2);
        const r5900_viwr_fhilo = utils::bitmask( 21,  4) | utils::bitmask(  0,  2);

        const r5900_immediate5 = utils::bitmask(  6,  5);
        const r5900_immediate15 = utils::bitmask(  6, 15);

        const r5900_vfs = utils::bitmask( 11,  5);
        const r5900_vft = utils::bitmask( 16,  5);
        const r5900_vfd = utils::bitmask(  6,  5);

        const r5900_vis = utils::bitmask( 11,  5);
        const r5900_vit = utils::bitmask( 16,  5);
        const r5900_vid = utils::bitmask(  6,  5);

        const r5900_xyzw_x = utils::bitmask( 24,  1);
        const r5900_xyzw_y = utils::bitmask( 23,  1);
        const r5900_xyzw_z = utils::bitmask( 22,  1);
        const r5900_xyzw_w = utils::bitmask( 21,  1);
        const r5900_xyzw_xyzw = Self::r5900_xyzw_x.union(Self::r5900_xyzw_y).union(Self::r5900_xyzw_z).union(Self::r5900_xyzw_w).bits();
        const r5900_n = utils::bitmask(  0,  2);
        const r5900_l = utils::bitmask( 21,  2);
        const r5900_m = utils::bitmask( 23,  2);

        const _ = !0;
    }
}

impl EncodedFieldMask {
    /// Returns a default value.
    #[must_use]
    pub const fn default() -> Self {
        Self::empty()
    }

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

impl Default for EncodedFieldMask {
    fn default() -> Self {
        Self::default()
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
