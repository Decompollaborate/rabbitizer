/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

RABBITIZER_DEF_INSTR_ID(cpu, , INVALID, .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM})

#include "instructions/instr_id/cpu/cpu_normal.inc"
#include "instructions/instr_id/cpu/cpu_special.inc"
#include "instructions/instr_id/cpu/cpu_regimm.inc"
#include "instructions/instr_id/cpu/cpu_cop0.inc"
#include "instructions/instr_id/cpu/cpu_cop0_bc0.inc"
#include "instructions/instr_id/cpu/cpu_cop0_tlb.inc"
#include "instructions/instr_id/cpu/cpu_cop1.inc"
#include "instructions/instr_id/cpu/cpu_cop1_bc1.inc"
#include "instructions/instr_id/cpu/cpu_cop1_fpu_s.inc"
#include "instructions/instr_id/cpu/cpu_cop1_fpu_d.inc"
#include "instructions/instr_id/cpu/cpu_cop1_fpu_w.inc"
#include "instructions/instr_id/cpu/cpu_cop1_fpu_l.inc"

RABBITIZER_DEF_INSTR_ID(cpu, , MAX, 0)
