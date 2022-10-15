/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_OPERAND_TYPE_H
#define RABBITIZER_OPERAND_TYPE_H
#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif


#define RAB_DEF_OPERAND(prefix, operand) \
    RAB_OPERAND_##prefix##_##operand,

typedef enum RabbitizerOperandType {
    RAB_DEF_OPERAND(ALL, INVALID)

#include "operands/RabbitizerOperandType_cpu.inc"
#include "operands/RabbitizerOperandType_rsp.inc"
#include "operands/RabbitizerOperandType_r5900.inc"

    RAB_DEF_OPERAND(ALL, MAX)
} RabbitizerOperandType;

#undef RAB_DEF_OPERAND


struct RabbitizerInstruction;

typedef size_t (*OperandCallback)(const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);

extern const OperandCallback instrOpercandCallbacks[];


#define RAB_DEF_OPERAND(prefix, operand) size_t RabbitizerOperandType_process_##prefix##_##operand (const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);

#include "instructions/operands/RabbitizerOperandType_cpu.inc"
#include "instructions/operands/RabbitizerOperandType_rsp.inc"
#include "instructions/operands/RabbitizerOperandType_r5900.inc"

#undef RAB_DEF_OPERAND


#ifdef __cplusplus
}
#endif

#endif
