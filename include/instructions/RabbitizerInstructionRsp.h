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


#define RAB_INSTR_RSP_SET_vs(self, value)           ((self)->word = BITREPACK((self)->word, value, 11,  5))
#define RAB_INSTR_RSP_SET_vt(self, value)           ((self)->word = BITREPACK((self)->word, value, 16,  5))
#define RAB_INSTR_RSP_SET_vd(self, value)           ((self)->word = BITREPACK((self)->word, value,  6,  5))

#define RAB_INSTR_RSP_SET_elementhigh(self, value)  ((self)->word = BITREPACK((self)->word, value, 16,  4))
#define RAB_INSTR_RSP_SET_elementlow(self, value)   ((self)->word = BITREPACK((self)->word, value,  7,  4))

#define RAB_INSTR_RSP_SET_index(self, value)        ((self)->word = BITREPACK((self)->word, value,  7,  4))
#define RAB_INSTR_RSP_SET_offset(self, value)       ((self)->word = BITREPACK((self)->word, value,  0,  7))


void RabbitizerInstructionRsp_init(RabbitizerInstruction *self, uint32_t word);
void RabbitizerInstructionRsp_destroy(RabbitizerInstruction *self);


void RabbitizerInstructionRsp_processUniqueId_Normal(RabbitizerInstruction *self);
void RabbitizerInstructionRsp_processUniqueId_Special(RabbitizerInstruction *self);
void RabbitizerInstructionRsp_processUniqueId_Regimm(RabbitizerInstruction *self);

void RabbitizerInstructionRsp_processUniqueId(RabbitizerInstruction *self);


uint16_t RabbitizerInstructionRsp_GetOffsetVector(const RabbitizerInstruction *self);

uint8_t RabbitizerInstructionRsp_processVectorElement(const RabbitizerInstruction *self, uint8_t element);


#endif
