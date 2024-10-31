/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct DecodingFlags: u32 {
        /// Produce pseudo instructions (like `move` or `b`) whenever those may match the desired original instruction.
        ///
        /// Turning off this flag disables all the other pseudos.
        const enable_pseudos = 1 << 0;
        const pseudo_move = 1 << 1; // TODO: consider removing
        const pseudo_beqz = 1 << 2;
        const pseudo_bnez = 1 << 3;
        const pseudo_b = 1 << 4;
        const pseudo_bal = 1 << 5;
        const pseudo_not = 1 << 6;
        const pseudo_neg = 1 << 7;
        const pseudo_negu = 1 << 8;

        const gnu_mode = 1 << 9;
    }
}

impl DecodingFlags {
    #[must_use]
    pub const fn default() -> Self {
        Self::enable_pseudos
            .union(Self::pseudo_beqz)
            .union(Self::pseudo_bnez)
            .union(Self::pseudo_b)
            .union(Self::pseudo_bal)
            .union(Self::pseudo_not)
            .union(Self::pseudo_neg)
            .union(Self::pseudo_negu)
            .union(Self::gnu_mode)
    }

    pub fn set_all_pseudos(&mut self, turn_on: bool) {
        if turn_on {
            self.insert(Self::enable_pseudos);
            self.insert(Self::pseudo_move);
            self.insert(Self::pseudo_beqz);
            self.insert(Self::pseudo_bnez);
            self.insert(Self::pseudo_b);
            self.insert(Self::pseudo_bal);
            self.insert(Self::pseudo_not);
            self.insert(Self::pseudo_neg);
            self.insert(Self::pseudo_negu);
        } else {
            self.remove(Self::enable_pseudos);
            self.remove(Self::pseudo_move);
            self.remove(Self::pseudo_beqz);
            self.remove(Self::pseudo_bnez);
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
            self.union(Self::enable_pseudos)
                .union(Self::pseudo_move)
                .union(Self::pseudo_beqz)
                .union(Self::pseudo_bnez)
                .union(Self::pseudo_b)
                .union(Self::pseudo_bal)
                .union(Self::pseudo_not)
                .union(Self::pseudo_neg)
                .union(Self::pseudo_negu)
        } else {
            self.intersection(Self::enable_pseudos.complement())
                .intersection(Self::pseudo_move.complement())
                .intersection(Self::pseudo_beqz.complement())
                .intersection(Self::pseudo_bnez.complement())
                .intersection(Self::pseudo_b.complement())
                .intersection(Self::pseudo_bal.complement())
                .intersection(Self::pseudo_not.complement())
                .intersection(Self::pseudo_neg.complement())
                .intersection(Self::pseudo_negu.complement())
        }
    }
}

impl Default for DecodingFlags {
    fn default() -> Self {
        Self::default()
    }
}
