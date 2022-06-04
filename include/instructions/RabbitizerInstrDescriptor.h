/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRDESCRIPTOR_H
#define RABBITIZER_INSTRDESCRIPTOR_H
#pragma once

#include <stdbool.h>

#include "RabbitizerRegisterType.h"
#include "RabbitizerInstrId.h"

typedef enum RabbitizerInstrType {
    RABBITIZER_INSTR_TYPE_UNKNOWN,
    RABBITIZER_INSTR_TYPE_J,
    RABBITIZER_INSTR_TYPE_I,
    RABBITIZER_INSTR_TYPE_R,
    RABBITIZER_INSTR_TYPE_REGIMM,
    RABBITIZER_INSTR_TYPE_MAX,
} RabbitizerInstrType;

typedef struct RabbitizerInstrDescriptor {
    RabbitizerRegisterType operands[4];
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

    int mipsVersion;
    // bool isRsp;
} RabbitizerInstrDescriptor;

// TODO: less redundant name
extern const RabbitizerInstrDescriptor RabbitizerInstrDescriptor_Descriptors[];


bool RabbitizerInstrDescriptor_IsJType(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_IsIType(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_IsRType(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_IsBranch(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_IsBranchLikely(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_IsJump(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_IsTrap(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_IsFloat(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_IsDouble(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_IsUnsigned(const RabbitizerInstrDescriptor *self);

bool RabbitizerInstrDescriptor_ModifiesRt(const RabbitizerInstrDescriptor *self);
bool RabbitizerInstrDescriptor_ModifiesRd(const RabbitizerInstrDescriptor *self);

#endif
