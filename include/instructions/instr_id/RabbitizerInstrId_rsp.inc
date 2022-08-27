/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// RSP instructions
RABBITIZER_DEF_INSTR_ID(rsp, , INVALID,               .operands={RABBITIZER_OPERAND_TYPE_RSP_vd, RABBITIZER_OPERAND_TYPE_RSP_vs, RABBITIZER_OPERAND_TYPE_RSP_vt_elementhigh})

#include "instructions/instr_id/rsp/rsp_cop2.inc"
#include "instructions/instr_id/rsp/rsp_cop2_vu.inc"

#include "instructions/instr_id/rsp/rsp_normal_lwc2.inc"
#include "instructions/instr_id/rsp/rsp_normal_swc2.inc"

// CPU instructions but with rsp registers

#include "instructions/instr_id/rsp/rsp_normal.inc"
#include "instructions/instr_id/rsp/rsp_special.inc"
#include "instructions/instr_id/rsp/rsp_regimm.inc"
#include "instructions/instr_id/rsp/rsp_cop0.inc"

RABBITIZER_DEF_INSTR_ID(rsp, , MAX, 0)
