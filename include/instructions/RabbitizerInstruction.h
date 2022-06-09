/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRUCTION_H
#define RABBITIZER_INSTRUCTION_H
#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "RabbitizerInstrId.h"
#include "RabbitizerInstrDescriptor.h"


#define RABBITIZER_DEF_INSTR_CATEGORY(name) RABBITIZER_INSTRCAT_##name

typedef enum RabbitizerInstrCategory {
    #include "instructions/InstrCategory.inc"

    RABBITIZER_DEF_INSTR_CATEGORY(MAX),
} RabbitizerInstrCategory;

#undef RABBITIZER_DEF_INSTR_CATEGORY


typedef struct RabbitizerInstruction {
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
    RabbitizerInstrCategory category;
} RabbitizerInstruction;

#define RAB_INSTR_GET_IMMEDIATE(self) (((self)->rd << 11) | ((self)->sa << 6) | ((self)->function))


void RabbitizerInstruction_init(RabbitizerInstruction *self, uint32_t word);
void RabbitizerInstruction_destroy(RabbitizerInstruction* self);


/* Process uniqueId */

void RabbitizerInstruction_processUniqueId_Normal(RabbitizerInstruction *self);
void RabbitizerInstruction_processUniqueId_Special(RabbitizerInstruction *self);
void RabbitizerInstruction_processUniqueId_Regimm(RabbitizerInstruction *self);
void RabbitizerInstruction_processUniqueId_Coprocessor0(RabbitizerInstruction *self);
void RabbitizerInstruction_processUniqueId_Coprocessor1(RabbitizerInstruction *self);
void RabbitizerInstruction_processUniqueId_Coprocessor2(RabbitizerInstruction *self);
void RabbitizerInstruction_processUniqueId(RabbitizerInstruction *self);

/* Process uniqueId */


/* Register getters */

uint8_t RabbitizerInstruction_getFs(const RabbitizerInstruction* self);
uint8_t RabbitizerInstruction_getFt(const RabbitizerInstruction* self);
uint8_t RabbitizerInstruction_getFd(const RabbitizerInstruction* self);

/* Register getters */


/* Coprocessor stuffs */

uint8_t RabbitizerInstruction_getFmt(const RabbitizerInstruction *self);
uint8_t RabbitizerInstruction_getTf(const RabbitizerInstruction *self);
uint8_t RabbitizerInstruction_getNd(const RabbitizerInstruction *self);
uint8_t RabbitizerInstruction_getFc(const RabbitizerInstruction *self);
uint8_t RabbitizerInstruction_getCond(const RabbitizerInstruction *self);

/* Coprocessor stuffs */


/* General getters */

uint32_t RabbitizerInstruction_getRaw(const RabbitizerInstruction *self);

uint32_t RabbitizerInstruction_getImmediate(const RabbitizerInstruction *self);
uint32_t RabbitizerInstruction_getInstrIndex(const RabbitizerInstruction *self);
uint32_t RabbitizerInstruction_getInstrIndexAsVram(const RabbitizerInstruction *self);

int32_t RabbitizerInstruction_getBranchOffset(const RabbitizerInstruction *self);
int32_t RabbitizerInstruction_getGenericBranchOffset(const RabbitizerInstruction *self, uint32_t currentVram);

/* General getters */


void RabbitizerInstruction_blankOut(RabbitizerInstruction *self);


/* Instruction examination */

bool RabbitizerInstruction_isImplemented(const RabbitizerInstruction *self);
bool RabbitizerInstruction_isLikelyHandwritten(const RabbitizerInstruction *self);
bool RabbitizerInstruction_isNop(const RabbitizerInstruction *self);
bool RabbitizerInstruction_isUnconditionalBranch(const RabbitizerInstruction *self);
bool RabbitizerInstruction_isJrRa(const RabbitizerInstruction *self);
bool RabbitizerInstruction_isJrNotRa(const RabbitizerInstruction *self);

const char *RabbitizerInstruction_mapInstrToType(const RabbitizerInstruction *self);

bool RabbitizerInstruction_sameOpcode(const RabbitizerInstruction *self, const RabbitizerInstruction *other);
bool RabbitizerInstruction_sameOpcodeButDifferentArguments(const RabbitizerInstruction *self, const RabbitizerInstruction *other);

/* Instruction examination */


/* Disassembly */

bool RabbitizerInstruction_mustDisasmAsData(const RabbitizerInstruction *self);

size_t RabbitizerInstruction_getSizeForBufferInstrDisasm(const RabbitizerInstruction *self, size_t immOverrideLength, int extraLJust);
size_t RabbitizerInstruction_disassembleInstruction(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust);

size_t RabbitizerInstruction_getSizeForBufferDataDisasm(const RabbitizerInstruction *self, int extraLJust);
size_t RabbitizerInstruction_disassembleAsData(const RabbitizerInstruction *self, char *dst, int extraLJust);

size_t RabbitizerInstruction_getSizeForBuffer(const RabbitizerInstruction *self, size_t immOverrideLength, int extraLJust);
size_t RabbitizerInstruction_disassemble(const RabbitizerInstruction *self, char *dst, const char *immOverride, size_t immOverrideLength, int extraLJust);

/* Disassembly */

#endif
