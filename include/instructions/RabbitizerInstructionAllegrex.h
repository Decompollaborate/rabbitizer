/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_ALLEGREX_H
#define RABBITIZER_INSTRUCTION_ALLEGREX_H
#pragma once

#include "RabbitizerInstruction.h"

#ifdef __cplusplus
extern "C" {
#endif


NON_NULL(1)
void RabbitizerInstructionAllegrex_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
NON_NULL(1)
void RabbitizerInstructionAllegrex_destroy(RabbitizerInstruction *self);


NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Normal(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Special(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Regimm(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId(RabbitizerInstruction *self);


#ifdef __cplusplus
}
#endif

#endif
