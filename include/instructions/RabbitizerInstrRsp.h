/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTR_RSP_H
#define RABBITIZER_INSTR_RSP_H
#pragma once

#include "RabbitizerInstr.h"


#define RAB_INSTR_RSP_GET_ELEMENT_LOW(self) ((((self)->sa) >> 1) & 0xF)
#define RAB_INSTR_RSP_GET_OFFSET_VECTOR_RAW(self) (RAB_INSTR_GET_IMMEDIATE(self) & 0x7F)


void RabbitizerInstrRsp_init(RabbitizerInstr *self, uint32_t word);
void RabbitizerInstrRsp_destroy(RabbitizerInstr *self);


void RabbitizerInstrRsp_processUniqueId_Normal(RabbitizerInstr *self);
void RabbitizerInstrRsp_processUniqueId_Special(RabbitizerInstr *self);
void RabbitizerInstrRsp_processUniqueId_Regimm(RabbitizerInstr *self);

void RabbitizerInstrRsp_processUniqueId(RabbitizerInstr *self);


uint16_t RabbitizerInstrRsp_GetOffsetVector(const RabbitizerInstr *self);

uint8_t RabbitizerInstrRsp_processVectorElement(const RabbitizerInstr *self, uint8_t element);


#endif
