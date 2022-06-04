/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTR_H
#define RABBITIZER_INSTR_H
#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "RabbitizerInstrId.h"
#include "RabbitizerInstrDescriptor.h"

typedef struct RabbitizerInstr {
    uint8_t opcode;
    uint8_t rs;
    uint8_t rt;
    uint8_t rd;
    uint8_t sa;
    uint8_t function;

    RabbitizerInstrId uniqueId;
    const RabbitizerInstrDescriptor *descriptor;

    int extraLjustWidthOpcode;
    uint32_t vram;
    bool _handwrittenCategory;
    bool inHandwrittenFunction;
} RabbitizerInstr;


void RabbitizerInstr_Init(RabbitizerInstr *self, uint32_t word);
void RabbitizerInstr_Destroy(RabbitizerInstr* self);

void RabbitizerInstr_ProcessUniqueId_Normal(RabbitizerInstr *self);
void RabbitizerInstr_ProcessUniqueId_Special(RabbitizerInstr *self);
void RabbitizerInstr_ProcessUniqueId_Regimm(RabbitizerInstr *self);
void RabbitizerInstr_ProcessUniqueId_Coprocessor0(RabbitizerInstr *self);

const char *RabbitizerInstr_GetOpcodeName(const RabbitizerInstr *self);

uint8_t RabbitizerInstr_GetFs(const RabbitizerInstr* self);
uint8_t RabbitizerInstr_GetFt(const RabbitizerInstr* self);
uint8_t RabbitizerInstr_GetFd(const RabbitizerInstr* self);

uint8_t RabbitizerInstr_GetFmt(const RabbitizerInstr *self);

uint8_t RabbitizerInstr_GetNd(const RabbitizerInstr *self);
uint8_t RabbitizerInstr_GetTf(const RabbitizerInstr *self);
uint8_t RabbitizerInstr_GetFc(const RabbitizerInstr *self);

uint32_t RabbitizerInstr_GetImmediate(const RabbitizerInstr *self);
uint32_t RabbitizerInstr_GetInstrIndex(const RabbitizerInstr *self);
uint32_t RabbitizerInstr_GetInstrIndexAsVram(const RabbitizerInstr *self);

bool RabbitizerInstr_IsNop(const RabbitizerInstr *self);

void RabbitizerInstr_DisassembleInstruction(const RabbitizerInstr* self, char *dst, const char *immOverride, size_t immOverrideLength);

#endif
