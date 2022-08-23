/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// OP vt[elementlow], offset(vs)
RABBITIZER_DEF_INSTR_ID(rsp, 0x00, sbv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x01, ssv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x02, slv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x03, sdv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x04, sqv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x05, srv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x06, spv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x07, suv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x08, shv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x09, sfv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
RABBITIZER_DEF_INSTR_ID(rsp, 0x0B, stv,     .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})

RABBITIZER_DEF_INSTR_ID(rsp, -0x07, swv,    .operands={RABBITIZER_OPERAND_TYPE_RSP_vt_elementlow, RABBITIZER_OPERAND_TYPE_RSP_offset_rs})
