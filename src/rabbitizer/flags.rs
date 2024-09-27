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

        const sn64_div_fix = 1 << 9; // TODO: rework (?)
        const gnu_mode = 1 << 10;
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
    pub(crate) abi: Abi,
    pub(crate) decoding_flags: DecodingFlags,
    pub(crate) treat_j_as_unconditional_branch: bool,
}

impl InstructionFlags {
    #[must_use]
    pub const fn default() -> Self {
        Self {
            abi: Abi::O32,
            decoding_flags: DecodingFlags::default(),
            treat_j_as_unconditional_branch: true,
        }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }
}

impl InstructionFlags {
    #[must_use]
    pub const fn abi(&self) -> Abi {
        self.abi
    }
    pub fn abi_mut(&mut self) -> &mut Abi {
        &mut self.abi
    }

    #[must_use]
    pub const fn decoding_flags(&self) -> &DecodingFlags {
        &self.decoding_flags
    }
    pub fn decoding_flags_mut(&mut self) -> &mut DecodingFlags {
        &mut self.decoding_flags
    }

    #[must_use]
    pub const fn treat_j_as_unconditional_branch(&self) -> bool {
        self.treat_j_as_unconditional_branch
    }
    pub fn treat_j_as_unconditional_branch_mut(&mut self) -> &mut bool {
        &mut self.treat_j_as_unconditional_branch
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
    omit_0x_on_small_imm: bool,
    // upper_case_imm: bool,
    expand_jalr: bool,

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
            // upper_case_imm: true,
            expand_jalr: false,

            r5900_modern_gas_instrs_workarounds: false,
            r5900_use_dollar: false,
        }
    }

    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }
}

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
    pub const fn named_gpr(&self) -> bool {
        self.named_registers && self.named_gpr
    }
    /// Use the ABI names for the general purpose registers when disassembling the main processor's instructions
    pub fn named_gpr_mut(&mut self) -> &mut bool {
        &mut self.named_gpr
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
    pub const fn named_vr4300_cop0(&self) -> bool {
        self.named_registers && self.named_vr4300_cop0
    }
    /// Use named registers for VR4300's coprocessor 0 registers
    pub fn named_vr4300_cop0_mut(&mut self) -> &mut bool {
        &mut self.named_vr4300_cop0
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
    pub const fn named_r4000allegrex_vfpucontrol(&self) -> bool {
        self.named_registers && self.named_r4000allegrex_vfpucontrol
    }
    /// Use named registers for R4000 Allegrex's VFPU control registers
    pub fn named_r4000allegrex_vfpucontrol_mut(&mut self) -> &mut bool {
        &mut self.named_r4000allegrex_vfpucontrol
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
    pub const fn unknown_instr_comment(&self) -> bool {
        self.unknown_instr_comment
    }
    /// Generate a pseudo-disassembly comment when disassembling non implemented instructions
    pub fn unknown_instr_comment_mut(&mut self) -> &mut bool {
        &mut self.unknown_instr_comment
    }

    #[must_use]
    pub const fn omit_0x_on_small_imm(&self) -> bool {
        self.omit_0x_on_small_imm
    }
    pub fn omit_0x_on_small_imm_mut(&mut self) -> &mut bool {
        &mut self.omit_0x_on_small_imm
    }

    #[must_use]
    pub const fn expand_jalr(&self) -> bool {
        self.expand_jalr
    }
    pub fn expand_jalr_mut(&mut self) -> &mut bool {
        &mut self.expand_jalr
    }

    #[must_use]
    pub const fn r5900_modern_gas_instrs_workarounds(&self) -> bool {
        self.r5900_modern_gas_instrs_workarounds
    }
    pub fn r5900_modern_gas_instrs_workarounds_mut(&mut self) -> &mut bool {
        &mut self.r5900_modern_gas_instrs_workarounds
    }

    #[must_use]
    pub const fn r5900_use_dollar(&self) -> bool {
        self.r5900_use_dollar
    }
    pub fn r5900_use_dollar_mut(&mut self) -> &mut bool {
        &mut self.r5900_use_dollar
    }
}

impl Default for DisplayFlags {
    fn default() -> Self {
        Self::default()
    }
}
