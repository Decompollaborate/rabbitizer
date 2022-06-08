/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_RSP_H
#define RABBITIZER_INSTRUCTION_RSP_H
#pragma once

#include "RabbitizerInstruction.h"


#define RAB_INSTR_RSP_GET_VS(self) ((self)->rd)
#define RAB_INSTR_RSP_GET_VT(self) ((self)->rt)
#define RAB_INSTR_RSP_GET_VD(self) ((self)->sa)

#define RAB_INSTR_RSP_GET_ELEMENT_HIGH(self) ((((self)->rs)) & 0xF)
#define RAB_INSTR_RSP_GET_ELEMENT_LOW(self) ((((self)->sa) >> 1) & 0xF)
#define RAB_INSTR_RSP_GET_OFFSET_VECTOR_RAW(self) (RAB_INSTR_GET_IMMEDIATE(self) & 0x7F)


void RabbitizerInstructionRsp_init(RabbitizerInstruction *self, uint32_t word);
void RabbitizerInstructionRsp_destroy(RabbitizerInstruction *self);


void RabbitizerInstructionRsp_processUniqueId_Normal(RabbitizerInstruction *self);
void RabbitizerInstructionRsp_processUniqueId_Special(RabbitizerInstruction *self);
void RabbitizerInstructionRsp_processUniqueId_Regimm(RabbitizerInstruction *self);

void RabbitizerInstructionRsp_processUniqueId(RabbitizerInstruction *self);


uint16_t RabbitizerInstructionRsp_GetOffsetVector(const RabbitizerInstruction *self);

uint8_t RabbitizerInstructionRsp_processVectorElement(const RabbitizerInstruction *self, uint8_t element);


#endif
