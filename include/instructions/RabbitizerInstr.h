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

    uint32_t vram;
    bool _handwrittenCategory;
    bool inHandwrittenFunction;
} RabbitizerInstr;


void RabbitizerInstr_init(RabbitizerInstr *self, uint32_t word);
void RabbitizerInstr_destroy(RabbitizerInstr* self);


/* Process uniqueId */

void RabbitizerInstr_processUniqueId_Normal(RabbitizerInstr *self);
void RabbitizerInstr_processUniqueId_Special(RabbitizerInstr *self);
void RabbitizerInstr_processUniqueId_Regimm(RabbitizerInstr *self);
void RabbitizerInstr_processUniqueId_Coprocessor0(RabbitizerInstr *self);
void RabbitizerInstr_processUniqueId_Coprocessor1(RabbitizerInstr *self);
void RabbitizerInstr_processUniqueId_Coprocessor2(RabbitizerInstr *self);
void RabbitizerInstr_processUniqueId(RabbitizerInstr *self);

/* Process uniqueId */


/* Register getters */

uint8_t RabbitizerInstr_getFs(const RabbitizerInstr* self);
uint8_t RabbitizerInstr_getFt(const RabbitizerInstr* self);
uint8_t RabbitizerInstr_getFd(const RabbitizerInstr* self);

/* Register getters */


/* Coprocessor stuffs */

uint8_t RabbitizerInstr_getFmt(const RabbitizerInstr *self);
uint8_t RabbitizerInstr_getTf(const RabbitizerInstr *self);
uint8_t RabbitizerInstr_getNd(const RabbitizerInstr *self);
uint8_t RabbitizerInstr_getFc(const RabbitizerInstr *self);
uint8_t RabbitizerInstr_getCond(const RabbitizerInstr *self);

/* Coprocessor stuffs */


/* General getters */

uint32_t RabbitizerInstr_getRaw(const RabbitizerInstr *self);

uint32_t RabbitizerInstr_getImmediate(const RabbitizerInstr *self);
uint32_t RabbitizerInstr_getInstrIndex(const RabbitizerInstr *self);
uint32_t RabbitizerInstr_getInstrIndexAsVram(const RabbitizerInstr *self);

int32_t RabbitizerInstr_getBranchOffset(const RabbitizerInstr *self);

/* General getters */


void RabbitizerInstr_blankOut(RabbitizerInstr *self);


/* Instruction examination */

bool RabbitizerInstr_isImplemented(const RabbitizerInstr *self);
bool RabbitizerInstr_isLikelyHandwritten(const RabbitizerInstr *self);
bool RabbitizerInstr_isNop(const RabbitizerInstr *self);
bool RabbitizerInstr_isUnconditionalBranch(const RabbitizerInstr *self);
bool RabbitizerInstr_isJrRa(const RabbitizerInstr *self);
bool RabbitizerInstr_isJrNotRa(const RabbitizerInstr *self);

const char *RabbitizerInstr_mapInstrToType(const RabbitizerInstr *self);

bool RabbitizerInstr_sameOpcode(const RabbitizerInstr *self, const RabbitizerInstr *other);
bool RabbitizerInstr_sameOpcodeButDifferentArguments(const RabbitizerInstr *self, const RabbitizerInstr *other);

/* Instruction examination */


/* Disassembly */

bool RabbitizerInstr_mustDisasmAsData(const RabbitizerInstr *self);

size_t RabbitizerInstr_getSizeForBufferInstrDisasm(const RabbitizerInstr *self, size_t immOverrideLength, int extraLJust);
size_t RabbitizerInstr_disassembleInstruction(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust);

size_t RabbitizerInstr_getSizeForBufferDataDisasm(const RabbitizerInstr *self, int extraLJust);
size_t RabbitizerInstr_disassembleAsData(const RabbitizerInstr *self, char *dst, int extraLJust);

size_t RabbitizerInstr_getSizeForBuffer(const RabbitizerInstr *self, size_t immOverrideLength, int extraLJust);
size_t RabbitizerInstr_disassemble(const RabbitizerInstr *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust);

/* Disassembly */

#endif
