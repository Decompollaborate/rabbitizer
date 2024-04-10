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
void RabbitizerInstructionAllegrex_processUniqueId_Special_Rs(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Special_Sa(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Regimm(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Special2(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Special3(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Special3_Bshfl(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor2(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Coprocessor3(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Vfpu0(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Vfpu1(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Vfpu3(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Vfpu4(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Vfpu5(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Vfpu6(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId_Vfpu7(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionAllegrex_processUniqueId(RabbitizerInstruction *self);


#ifdef __cplusplus
}
#endif

#endif
