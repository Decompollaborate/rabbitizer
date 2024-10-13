/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use bitflags::bitflags;

use crate::Abi;

bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DecodingFlags: u32 {
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
}

impl Default for DecodingFlags {
    fn default() -> Self {
        Self::default()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionFlags {
    pub(crate) decoding_flags: DecodingFlags,
    pub(crate) abi: Abi,
    pub(crate) j_as_branch: bool,
}

impl InstructionFlags {
    #[must_use]
    pub const fn default() -> Self {
        Self {
            decoding_flags: DecodingFlags::default(),
            abi: Abi::O32,
            j_as_branch: true,
        }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }
}

impl InstructionFlags {
    #[must_use]
    pub const fn decoding_flags(&self) -> &DecodingFlags {
        &self.decoding_flags
    }
    pub fn decoding_flags_mut(&mut self) -> &mut DecodingFlags {
        &mut self.decoding_flags
    }
    #[must_use]
    pub const fn with_decoding_flags(self, decoding_flags: DecodingFlags) -> Self {
        Self {
            decoding_flags,
            ..self
        }
    }

    #[must_use]
    pub const fn enable_pseudos(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::enable_pseudos)
    }
    pub fn set_enable_pseudos(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::enable_pseudos);
        } else {
            self.decoding_flags.remove(DecodingFlags::enable_pseudos);
        }
    }
    #[must_use]
    pub const fn with_enable_pseudos(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::enable_pseudos
        } else {
            DecodingFlags::enable_pseudos.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_move(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_move)
    }
    pub fn set_pseudo_move(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_move);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_move);
        }
    }
    #[must_use]
    pub const fn with_pseudo_move(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_move
        } else {
            DecodingFlags::pseudo_move.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_beqz(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_beqz)
    }
    pub fn set_pseudo_beqz(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_beqz);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_beqz);
        }
    }
    #[must_use]
    pub const fn with_pseudo_beqz(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_beqz
        } else {
            DecodingFlags::pseudo_beqz.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_bnez(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_bnez)
    }
    pub fn set_pseudo_bnez(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_bnez);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_bnez);
        }
    }
    #[must_use]
    pub const fn with_pseudo_bnez(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_bnez
        } else {
            DecodingFlags::pseudo_bnez.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_b(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_b)
    }
    pub fn set_pseudo_b(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_b);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_b);
        }
    }
    #[must_use]
    pub const fn with_pseudo_b(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_b
        } else {
            DecodingFlags::pseudo_b.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_bal(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_bal)
    }
    pub fn set_pseudo_bal(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_bal);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_bal);
        }
    }
    #[must_use]
    pub const fn with_pseudo_bal(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_bal
        } else {
            DecodingFlags::pseudo_bal.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_not(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_not)
    }
    pub fn set_pseudo_not(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_not);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_not);
        }
    }
    #[must_use]
    pub const fn with_pseudo_not(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_not
        } else {
            DecodingFlags::pseudo_not.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_neg(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_neg)
    }
    pub fn set_pseudo_neg(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_neg);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_neg);
        }
    }
    #[must_use]
    pub const fn with_pseudo_neg(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_neg
        } else {
            DecodingFlags::pseudo_neg.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn pseudo_negu(&self) -> bool {
        self.decoding_flags.contains(DecodingFlags::pseudo_negu)
    }
    pub fn set_pseudo_negu(&mut self, turn_on: bool) {
        if turn_on {
            self.decoding_flags.insert(DecodingFlags::pseudo_negu);
        } else {
            self.decoding_flags.remove(DecodingFlags::pseudo_negu);
        }
    }
    #[must_use]
    pub const fn with_pseudo_negu(self, turn_on: bool) -> Self {
        let other = if turn_on {
            DecodingFlags::pseudo_negu
        } else {
            DecodingFlags::pseudo_negu.complement()
        };
        self.with_decoding_flags(self.decoding_flags.intersection(other))
    }

    #[must_use]
    pub const fn abi(&self) -> Abi {
        self.abi
    }
    pub fn abi_mut(&mut self) -> &mut Abi {
        &mut self.abi
    }
    #[must_use]
    pub const fn with_abi(self, abi: Abi) -> Self {
        Self { abi, ..self }
    }

    #[must_use]
    pub const fn j_as_branch(&self) -> bool {
        self.j_as_branch
    }
    pub fn j_as_branch_mut(&mut self) -> &mut bool {
        &mut self.j_as_branch
    }
    #[must_use]
    pub const fn with_j_as_branch(self, j_as_branch: bool) -> Self {
        Self {
            j_as_branch,
            ..self
        }
    }
}

impl Default for InstructionFlags {
    fn default() -> Self {
        Self::default()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayFlags {
    /// Enables using named registers. This option takes precedence over the other named register options
    named_registers: bool,
    /// Use the ABI names for the general purpose registers when disassembling the main processor's instructions
    named_gpr: bool,
    /// Use the ABI names for the floating point registers when disassembling the floating point (coprocessor 1) instructions
    named_fpr: bool,
    /// Use named registers for VR4300's coprocessor 0 registers
    named_vr4300_cop0: bool,
    /// Use named registers for VR4300's RSP's coprocessor 0 registers
    named_rsp_cop0: bool,
    /// Use named registers for R4000 Allegrex's VFPU control registers
    named_r4000allegrex_vfpucontrol: bool,

    /// The minimal number of characters to left-align the opcode name
    opcode_ljust: u32,
    /// Generate a pseudo-disassembly comment when disassembling non implemented instructions
    unknown_instr_comment: bool,
    /// Omit the `0x` prefix on small immediates (values on the \[-9, 9\] inclusive range).
    omit_0x_on_small_imm: bool, // TODO: maybe remove?

    expand_jalr: bool,
    gnu_div: bool,
    sn64_break_fix: bool,

    r5900_modern_gas_instrs_workarounds: bool,
    r5900_use_dollar: bool,
}

impl DisplayFlags {
    #[must_use]
    pub const fn default() -> Self {
        Self {
            named_registers: true,
            named_gpr: true,
            named_fpr: false, // TODO: consider changing to True
            named_vr4300_cop0: false,
            named_rsp_cop0: false,
            named_r4000allegrex_vfpucontrol: false,

            opcode_ljust: 7 + 4,
            unknown_instr_comment: true,
            omit_0x_on_small_imm: false,

            expand_jalr: false,
            gnu_div: true,
            sn64_break_fix: false,

            r5900_modern_gas_instrs_workarounds: false,
            r5900_use_dollar: false,
        }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn new_gnu_as() -> Self {
        Self {
            gnu_div: true,
            sn64_break_fix: false,

            r5900_modern_gas_instrs_workarounds: true,
            r5900_use_dollar: true,
            ..Self::new()
        }
    }

    #[must_use]
    pub const fn new_legacy_as() -> Self {
        Self {
            named_fpr: false,

            gnu_div: false,
            sn64_break_fix: false,

            r5900_modern_gas_instrs_workarounds: false,
            r5900_use_dollar: false,
            ..Self::new()
        }
    }
}

/// Getters and setters
impl DisplayFlags {
    #[must_use]
    pub const fn named_registers(&self) -> bool {
        self.named_registers
    }
    /// Enables using named registers. This option takes precedence over the other named register options
    pub fn named_registers_mut(&mut self) -> &mut bool {
        &mut self.named_registers
    }
    #[must_use]
    pub const fn with_named_registers(self, named_registers: bool) -> Self {
        Self {
            named_registers,
            ..self
        }
    }

    #[must_use]
    pub const fn named_gpr(&self) -> bool {
        self.named_registers && self.named_gpr
    }
    /// Use the ABI names for the general purpose registers when disassembling the main processor's instructions
    pub fn named_gpr_mut(&mut self) -> &mut bool {
        &mut self.named_gpr
    }
    #[must_use]
    pub const fn with_named_gpr(self, named_gpr: bool) -> Self {
        Self { named_gpr, ..self }
    }

    #[must_use]
    pub const fn named_fpr(&self) -> bool {
        self.named_registers && self.named_fpr
    }
    /// Use the ABI names for the floating point registers when disassembling the floating point (coprocessor 1) instructions
    pub fn named_fpr_mut(&mut self) -> &mut bool {
        &mut self.named_fpr
    }
    #[must_use]
    pub const fn with_named_fpr(self, named_fpr: bool) -> Self {
        Self { named_fpr, ..self }
    }

    #[must_use]
    pub const fn named_vr4300_cop0(&self) -> bool {
        self.named_registers && self.named_vr4300_cop0
    }
    /// Use named registers for VR4300's coprocessor 0 registers
    pub fn named_vr4300_cop0_mut(&mut self) -> &mut bool {
        &mut self.named_vr4300_cop0
    }
    #[must_use]
    pub const fn with_named_vr4300_cop0(self, named_vr4300_cop0: bool) -> Self {
        Self {
            named_vr4300_cop0,
            ..self
        }
    }

    #[must_use]
    pub const fn named_rsp_cop0(&self) -> bool {
        self.named_registers && self.named_rsp_cop0
    }
    /// Use named registers for VR4300's RSP's coprocessor 0 registers
    pub fn named_rsp_cop0_mut(&mut self) -> &mut bool {
        &mut self.named_rsp_cop0
    }
    #[must_use]
    pub const fn with_named_rsp_cop0(self, named_rsp_cop0: bool) -> Self {
        Self {
            named_rsp_cop0,
            ..self
        }
    }

    #[must_use]
    pub const fn named_r4000allegrex_vfpucontrol(&self) -> bool {
        self.named_registers && self.named_r4000allegrex_vfpucontrol
    }
    /// Use named registers for R4000 Allegrex's VFPU control registers
    pub fn named_r4000allegrex_vfpucontrol_mut(&mut self) -> &mut bool {
        &mut self.named_r4000allegrex_vfpucontrol
    }
    #[must_use]
    pub const fn with_named_r4000allegrex_vfpucontrol(
        self,
        named_r4000allegrex_vfpucontrol: bool,
    ) -> Self {
        Self {
            named_r4000allegrex_vfpucontrol,
            ..self
        }
    }

    #[must_use]
    pub const fn opcode_ljust(&self) -> u32 {
        self.opcode_ljust
    }
    /// The minimal number of characters to left-align the opcode name
    pub fn opcode_ljust_mut(&mut self) -> &mut u32 {
        &mut self.opcode_ljust
    }
    #[must_use]
    pub const fn with_opcode_ljust(self, opcode_ljust: u32) -> Self {
        Self {
            opcode_ljust,
            ..self
        }
    }

    #[must_use]
    pub const fn unknown_instr_comment(&self) -> bool {
        self.unknown_instr_comment
    }
    /// Generate a pseudo-disassembly comment when disassembling non implemented instructions
    pub fn unknown_instr_comment_mut(&mut self) -> &mut bool {
        &mut self.unknown_instr_comment
    }
    #[must_use]
    pub const fn with_unknown_instr_comment(self, unknown_instr_comment: bool) -> Self {
        Self {
            unknown_instr_comment,
            ..self
        }
    }

    #[must_use]
    pub const fn omit_0x_on_small_imm(&self) -> bool {
        self.omit_0x_on_small_imm
    }
    /// Omit the `0x` prefix on small immediates (values on the \[-9, 9\] inclusive range).
    pub fn omit_0x_on_small_imm_mut(&mut self) -> &mut bool {
        &mut self.omit_0x_on_small_imm
    }
    #[must_use]
    pub const fn with_omit_0x_on_small_imm(self, omit_0x_on_small_imm: bool) -> Self {
        Self {
            omit_0x_on_small_imm,
            ..self
        }
    }

    #[must_use]
    pub const fn expand_jalr(&self) -> bool {
        self.expand_jalr
    }
    pub fn expand_jalr_mut(&mut self) -> &mut bool {
        &mut self.expand_jalr
    }
    #[must_use]
    pub const fn with_expand_jalr(self, expand_jalr: bool) -> Self {
        Self {
            expand_jalr,
            ..self
        }
    }

    #[must_use]
    pub const fn gnu_div(&self) -> bool {
        self.gnu_div
    }
    pub fn gnu_div_mut(&mut self) -> &mut bool {
        &mut self.gnu_div
    }
    #[must_use]
    pub const fn with_gnu_div(self, gnu_div: bool) -> Self {
        Self { gnu_div, ..self }
    }

    #[must_use]
    pub const fn sn64_break_fix(&self) -> bool {
        self.sn64_break_fix
    }
    pub fn sn64_break_fix_mut(&mut self) -> &mut bool {
        &mut self.sn64_break_fix
    }
    #[must_use]
    pub const fn with_sn64_break_fix(self, sn64_break_fix: bool) -> Self {
        Self {
            sn64_break_fix,
            ..self
        }
    }

    #[must_use]
    pub const fn r5900_modern_gas_instrs_workarounds(&self) -> bool {
        self.r5900_modern_gas_instrs_workarounds
    }
    pub fn r5900_modern_gas_instrs_workarounds_mut(&mut self) -> &mut bool {
        &mut self.r5900_modern_gas_instrs_workarounds
    }
    #[must_use]
    pub const fn with_r5900_modern_gas_instrs_workarounds(
        self,
        r5900_modern_gas_instrs_workarounds: bool,
    ) -> Self {
        Self {
            r5900_modern_gas_instrs_workarounds,
            ..self
        }
    }

    #[must_use]
    pub const fn r5900_use_dollar(&self) -> bool {
        self.r5900_use_dollar
    }
    pub fn r5900_use_dollar_mut(&mut self) -> &mut bool {
        &mut self.r5900_use_dollar
    }
    #[must_use]
    pub const fn with_r5900_use_dollar(self, r5900_use_dollar: bool) -> Self {
        Self {
            r5900_use_dollar,
            ..self
        }
    }
}

impl Default for DisplayFlags {
    fn default() -> Self {
        Self::default()
    }
}
