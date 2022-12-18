/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_OPERAND_TYPE_H
#define RABBITIZER_OPERAND_TYPE_H
#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#include "OperandType_enum.table.h"

struct RabbitizerInstruction;

typedef size_t (*OperandCallback)(const struct RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength);

extern const OperandCallback instrOpercandCallbacks[];

#include "OperandType_function_declarations.table.h"

#ifdef __cplusplus
}
#endif

#endif
