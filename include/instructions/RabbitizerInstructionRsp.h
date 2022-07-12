/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_RSP_H
#define RABBITIZER_INSTRUCTION_RSP_H
#pragma once

#include "RabbitizerInstruction.h"


#define RAB_INSTR_RSP_GET_VS(self)                  (SHIFTR((self)->word, 11,  5))
#define RAB_INSTR_RSP_GET_VT(self)                  (SHIFTR((self)->word, 16,  5))
#define RAB_INSTR_RSP_GET_VD(self)                  (SHIFTR((self)->word,  6,  5))

#define RAB_INSTR_RSP_GET_elementhigh(self)         (SHIFTR((self)->word, 21,  4))
#define RAB_INSTR_RSP_GET_elementlow(self)          (SHIFTR((self)->word,  7,  4))
#define RAB_INSTR_RSP_GET_OFFSET_VECTOR_RAW(self)   (SHIFTR((self)->word,  0,  7))

#define RAB_INSTR_RSP_GET_index(self)               (SHIFTR((self)->word,  7,  4))


#define RAB_INSTR_RSP_PACK_vs(word, value)          (BITREPACK((word), value, 11,  5))
#define RAB_INSTR_RSP_PACK_vt(word, value)          (BITREPACK((word), value, 16,  5))
#define RAB_INSTR_RSP_PACK_vd(word, value)          (BITREPACK((word), value,  6,  5))

#define RAB_INSTR_RSP_PACK_elementhigh(word, value) (BITREPACK((word), value, 16,  4))
#define RAB_INSTR_RSP_PACK_elementlow(word, value)  (BITREPACK((word), value,  7,  4))

#define RAB_INSTR_RSP_PACK_index(word, value)       (BITREPACK((word), value,  7,  4))
#define RAB_INSTR_RSP_PACK_offset(word, value)      (BITREPACK((word), value,  0,  7))


NON_NULL(1)
void RabbitizerInstructionRsp_init(RabbitizerInstruction *self, uint32_t word, uint32_t vram);
NON_NULL(1)
void RabbitizerInstructionRsp_destroy(RabbitizerInstruction *self);


NON_NULL(1)
void RabbitizerInstructionRsp_processUniqueId_Normal(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionRsp_processUniqueId_Special(RabbitizerInstruction *self);
NON_NULL(1)
void RabbitizerInstructionRsp_processUniqueId_Regimm(RabbitizerInstruction *self);

NON_NULL(1)
void RabbitizerInstructionRsp_processUniqueId(RabbitizerInstruction *self);


NODISCARD NON_NULL(1) PURE
uint16_t RabbitizerInstructionRsp_GetOffsetVector(const RabbitizerInstruction *self);

NODISCARD NON_NULL(1) PURE
uint8_t RabbitizerInstructionRsp_processVectorElement(const RabbitizerInstruction *self, uint8_t element);


#endif
