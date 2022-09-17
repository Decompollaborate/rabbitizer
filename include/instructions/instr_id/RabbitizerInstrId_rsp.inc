/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// RSP instructions
RABBITIZER_DEF_INSTR_ID(rsp, , INVALID,               .operands={RAB_OPERAND_rsp_vd, RAB_OPERAND_rsp_vs, RAB_OPERAND_rsp_vt_elementhigh})

#include "instructions/instr_id/rsp/rsp_cop2.inc"
#include "instructions/instr_id/rsp/rsp_cop2_vu.inc"

#include "instructions/instr_id/rsp/rsp_normal_lwc2.inc"
#include "instructions/instr_id/rsp/rsp_normal_swc2.inc"

// CPU instructions but with rsp registers

#include "instructions/instr_id/rsp/rsp_normal.inc"
#include "instructions/instr_id/rsp/rsp_special.inc"
#include "instructions/instr_id/rsp/rsp_regimm.inc"
#include "instructions/instr_id/rsp/rsp_cop0.inc"

RABBITIZER_DEF_INSTR_ID(rsp, , MAX, .operands={0})
