/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums_utils.h"
#include "instructions/RabbitizerOperandType.h"


#define RAB_DEF_OPERAND(prefix, operand) { "OperandType", #prefix "_" #operand, RAB_OPERAND_##prefix##_##operand, false, NULL },

RabbitizerEnumMetadata rabbitizer_enum_OperandType_enumvalues[] = {
    RAB_DEF_OPERAND(ALL, INVALID)

#include "instructions/operands/RabbitizerOperandType_cpu.inc"
#include "instructions/operands/RabbitizerOperandType_rsp.inc"
#include "instructions/operands/RabbitizerOperandType_r5900.inc"

    RAB_DEF_OPERAND(ALL, MAX)

    { 0 },
};

#undef RAB_DEF_OPERAND

static PyMethodDef rabbitizer_enum_OperandType_methods[] = {
    { 0 },
};

DEF_ENUM(OperandType, "")
