/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_REGISTERS_TRACKER_H
#define RABBITIZER_REGISTERS_TRACKER_H
#pragma once

#include "common/Utils.h"
#include "RabbitizerTrackedRegisterState.h"
#include "instructions/RabbitizerInstruction.h"


typedef struct RabbitizerRegistersTracker {
    RabbitizerTrackedRegisterState registers[32];
} RabbitizerRegistersTracker;


NON_NULL(1)
void RabbitizerRegistersTracker_init(RabbitizerRegistersTracker *self, const RabbitizerRegistersTracker *other);
NON_NULL(1)
void RabbitizerRegistersTracker_destroy(RabbitizerRegistersTracker *self);

NON_NULL(1, 2)
bool RabbitizerRegistersTracker_moveRegisters(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr);
NON_NULL(1, 2)
void RabbitizerRegistersTracker_overwriteRegisters(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset);
NON_NULL(1, 2, 3)
void RabbitizerRegistersTracker_unsetRegistersAfterFuncCall(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, const RabbitizerInstruction *prevInstr);
NON_NULL(1, 2, 4)
bool RabbitizerRegistersTracker_getAddressIfCanSetType(const RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, uint32_t *dstAddress);
NON_NULL(1, 2, 3, 4)
bool RabbitizerRegistersTracker_getJrInfo(const RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int *dstOffset, uint32_t *dstAddress);

// prevInstr can be NULL
NON_NULL(1, 2)
void RabbitizerRegistersTracker_processLui(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, const RabbitizerInstruction *prevInstr);
NON_NULL(1, 2, 3)
bool RabbitizerRegistersTracker_getLuiOffsetForConstant(const RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int *dstOffset);
NON_NULL(1, 2)
void RabbitizerRegistersTracker_processConstant(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, uint32_t value, int offset);
NON_NULL(1, 2, 4, 5)
bool RabbitizerRegistersTracker_getLuiOffsetForLo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, int *dstOffset, bool *dstIsGp);
NON_NULL(1, 2)
void RabbitizerRegistersTracker_processLo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, uint32_t value, int offset);
NON_NULL(1, 2)
bool RabbitizerRegistersTracker_hasLoButNoHi(const RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr);


#endif
