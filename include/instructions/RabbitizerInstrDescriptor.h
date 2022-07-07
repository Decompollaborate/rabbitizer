/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRDESCRIPTOR_H
#define RABBITIZER_INSTRDESCRIPTOR_H
#pragma once

#include <stdbool.h>

#include "RabbitizerOperandType.h"
#include "RabbitizerInstrId.h"

typedef enum RabbitizerInstrType {
    RABBITIZER_INSTR_TYPE_UNKNOWN,
    RABBITIZER_INSTR_TYPE_J,
    RABBITIZER_INSTR_TYPE_I,
    RABBITIZER_INSTR_TYPE_R,
    RABBITIZER_INSTR_TYPE_REGIMM,
    RABBITIZER_INSTR_TYPE_MAX,
} RabbitizerInstrType;

typedef enum RabbitizerArchitectureVersion {
    RABBITIZER_ARCHVERSION_INVALID=-1,
    RABBITIZER_ARCHVERSION_UNKNOWN,
    RABBITIZER_ARCHVERSION_MIPS_I,
    RABBITIZER_ARCHVERSION_MIPS_II,
    RABBITIZER_ARCHVERSION_MIPS_III,
    RABBITIZER_ARCHVERSION_MIPS_IV
} RabbitizerArchitectureVersion;

typedef struct RabbitizerInstrDescriptor {
    RabbitizerOperandType operands[4];
    RabbitizerInstrType instrType;

    bool isBranch;
    bool isBranchLikely;
    bool isJump;
    bool isTrap;

    bool isFloat;
    bool isDouble;

    bool isUnsigned;

    bool modifiesRt;
    bool modifiesRd;

    bool notEmitedByCompilers;

    bool canBeHi;
    bool canBeLo;
    bool doesLink; // "and link" family of instructions
    bool doesDereference;
    bool doesLoad; // loads data from memory
    bool doesStore; // stores data to memory
    bool maybeIsMove;

    bool isPseudo;

    RabbitizerArchitectureVersion architectureVersion;
} RabbitizerInstrDescriptor;

// TODO: less redundant name
extern const RabbitizerInstrDescriptor RabbitizerInstrDescriptor_Descriptors[];


bool RabbitizerInstrDescriptor_isUnknownType(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isJType(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isIType(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isRType(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isRegimmType(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_isBranch(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isBranchLikely(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isJump(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isTrap(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_isFloat(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_isDouble(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_isUnsigned(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_modifiesRt(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_modifiesRd(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_notEmitedByCompilers(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_canBeHi(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_canBeLo(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_doesLink(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_doesDereference(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_doesLoad(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_doesStore(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_maybeIsMove(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_isPseudo(const RabbitizerInstrDescriptor *self);

RabbitizerArchitectureVersion RabbitizerInstrDescriptor_getArchitectureVersion(const RabbitizerInstrDescriptor *self);

#endif
