/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct DecodingFlags: u32 {
        /// Produce pseudo instructions (like `neg` or `b`).
        ///
        /// Turning off this flag disables all the other pseudos.
        const enable_pseudos = 1 << 0;
        const pseudo_beqz = 1 << 1;
        const pseudo_bnez = 1 << 2;
        #[cfg(feature = "MIPS_II")]
        const pseudo_beqzl = 1 << 3;
        #[cfg(feature = "MIPS_II")]
        const pseudo_bnezl = 1 << 4;
        const pseudo_b = 1 << 5;
        const pseudo_bal = 1 << 6;
        const pseudo_not = 1 << 7;
        const pseudo_neg = 1 << 8;
        const pseudo_negu = 1 << 9;

        /// Allow decoding VICE MSP instructions as part of the RSP ones.
        #[cfg(feature="RspViceMsp")]
        const gated_rsp_vice_msp = 1 << 26;
    }
}

impl DecodingFlags {
    /// Returns a default value.
    ///
    /// Enables most pseudo instructions by default.
    #[must_use]
    pub const fn default() -> Self {
        #[allow(unused_mut)]
        let mut out = Self::enable_pseudos
            .union(Self::pseudo_beqz)
            .union(Self::pseudo_bnez)
            .union(Self::pseudo_b)
            .union(Self::pseudo_bal)
            .union(Self::pseudo_not)
            .union(Self::pseudo_neg)
            .union(Self::pseudo_negu);

        #[cfg(feature = "MIPS_II")]
        {
            out = out.union(Self::pseudo_beqzl).union(Self::pseudo_bnezl);
        }

        out
    }

    pub fn set_all_pseudos(&mut self, turn_on: bool) {
        if turn_on {
            self.insert(Self::enable_pseudos);
            self.insert(Self::pseudo_beqz);
            self.insert(Self::pseudo_bnez);
            #[cfg(feature = "MIPS_II")]
            self.insert(Self::pseudo_beqzl);
            #[cfg(feature = "MIPS_II")]
            self.insert(Self::pseudo_bnezl);
            self.insert(Self::pseudo_b);
            self.insert(Self::pseudo_bal);
            self.insert(Self::pseudo_not);
            self.insert(Self::pseudo_neg);
            self.insert(Self::pseudo_negu);
        } else {
            self.remove(Self::enable_pseudos);
            self.remove(Self::pseudo_beqz);
            self.remove(Self::pseudo_bnez);
            #[cfg(feature = "MIPS_II")]
            self.remove(Self::pseudo_beqzl);
            #[cfg(feature = "MIPS_II")]
            self.remove(Self::pseudo_bnezl);
            self.remove(Self::pseudo_b);
            self.remove(Self::pseudo_bal);
            self.remove(Self::pseudo_not);
            self.remove(Self::pseudo_neg);
            self.remove(Self::pseudo_negu);
        }
    }
    #[must_use]
    pub const fn with_all_pseudos(self, turn_on: bool) -> Self {
        if turn_on {
            #[allow(unused_mut)]
            let mut out = self
                .union(Self::enable_pseudos)
                .union(Self::pseudo_beqz)
                .union(Self::pseudo_bnez)
                .union(Self::pseudo_b)
                .union(Self::pseudo_bal)
                .union(Self::pseudo_not)
                .union(Self::pseudo_neg)
                .union(Self::pseudo_negu);

            #[cfg(feature = "MIPS_II")]
            {
                out = out.union(Self::pseudo_beqzl).union(Self::pseudo_bnezl);
            }

            out
        } else {
            #[allow(unused_mut)]
            let mut out = self
                .intersection(Self::enable_pseudos.complement())
                .intersection(Self::pseudo_beqz.complement())
                .intersection(Self::pseudo_bnez.complement())
                .intersection(Self::pseudo_b.complement())
                .intersection(Self::pseudo_bal.complement())
                .intersection(Self::pseudo_not.complement())
                .intersection(Self::pseudo_neg.complement())
                .intersection(Self::pseudo_negu.complement());

            #[cfg(feature = "MIPS_II")]
            {
                out = out
                    .intersection(Self::pseudo_beqzl.complement())
                    .intersection(Self::pseudo_bnezl.complement())
            }

            out
        }
    }
}

impl Default for DecodingFlags {
    fn default() -> Self {
        Self::default()
    }
}
