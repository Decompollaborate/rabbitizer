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

#endif
