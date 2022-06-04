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

uint32_t RabbitizerInstr_GetInstrIndex(RabbitizerInstr *self);
uint32_t RabbitizerInstr_GetInstrIndexAsVram(RabbitizerInstr *self);

void RabbitizerInstr_DisassembleInstruction(RabbitizerInstr* self, char *dst, const char *immOverride, size_t immOverrideLength);

#endif
