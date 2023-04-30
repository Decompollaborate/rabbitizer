/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_R3000GTE_H
#define RABBITIZER_INSTRUCTION_R3000GTE_H
#pragma once

#include "RabbitizerInstruction.h"

#ifdef __cplusplus
extern "C" {
#endif



NON_NULL(1)
void RabbitizerInstructionR3000GTE_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
NON_NULL(1)
void RabbitizerInstructionR3000GTE_destroy(RabbitizerInstruction *self);

#if 0
NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId_Normal(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId_Special(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId_Regimm(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId_Coprocessor0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId_Coprocessor1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId_Coprocessor2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId_MMI(RabbitizerInstruction *self);
#endif

NON_NULL(1)
void RabbitizerInstructionR3000GTE_processUniqueId(RabbitizerInstruction *self);


#ifdef __cplusplus
}
#endif

#endif
