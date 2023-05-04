/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_R4000ALLEGREX_H
#define RABBITIZER_INSTRUCTION_R4000ALLEGREX_H
#pragma once

#include "RabbitizerInstruction.h"

#ifdef __cplusplus
extern "C" {
#endif



NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_destroy(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Normal(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Special(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Regimm(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_processUniqueId_Coprocessor2(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionR4000ALLEGREX_processUniqueId(RabbitizerInstruction *self);


#ifdef __cplusplus
}
#endif

#endif
