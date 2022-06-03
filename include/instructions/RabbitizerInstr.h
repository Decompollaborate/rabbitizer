/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTR_H
#define RABBITIZER_INSTR_H
#pragma once

#include <stdbool.h>
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
    RabbitizerInstrDescriptor *descriptor;

    int extraLjustWidthOpcode;
    uint32_t vram;
    bool _handwrittenCategory;
    bool inHandwrittenFunction;
} RabbitizerInstr;

#endif
