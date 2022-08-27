/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

RABBITIZER_DEF_INSTR_ID(r5900, , INVALID,     .operands={RABBITIZER_OPERAND_TYPE_rs, RABBITIZER_OPERAND_TYPE_rt, RABBITIZER_OPERAND_TYPE_IMM}, .instrType=RABBITIZER_INSTR_TYPE_UNKNOWN, .architectureVersion=-1)

#include "instructions/instr_id/r5900/r5900_normal.inc"
#include "instructions/instr_id/r5900/r5900_special.inc"
#include "instructions/instr_id/r5900/r5900_regimm.inc"

#include "instructions/instr_id/r5900/r5900_mmi.inc"
#include "instructions/instr_id/r5900/r5900_mmi_0.inc"
#include "instructions/instr_id/r5900/r5900_mmi_1.inc"
#include "instructions/instr_id/r5900/r5900_mmi_2.inc"
#include "instructions/instr_id/r5900/r5900_mmi_3.inc"

#include "instructions/instr_id/r5900/r5900_cop0_tlb.inc"

#include "instructions/instr_id/r5900/r5900_cop1_fpu_s.inc"

#include "instructions/instr_id/r5900/r5900_cop2.inc"
#include "instructions/instr_id/r5900/r5900_cop2_bc2.inc"
#include "instructions/instr_id/r5900/r5900_cop2_special1.inc"
#include "instructions/instr_id/r5900/r5900_cop2_special2.inc"


RABBITIZER_DEF_INSTR_ID(r5900, , MAX, 0)
