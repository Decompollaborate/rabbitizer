/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_REGISTERS_TRACKER_H
#define RABBITIZER_REGISTERS_TRACKER_H
#pragma once

#include "RabbitizerTrackedRegisterState.h"
#include "instructions/RabbitizerInstruction.h"


typedef struct RabbitizerRegistersTracker {
    RabbitizerTrackedRegisterState registers[32];
} RabbitizerRegistersTracker;


void RabbitizerRegistersTracker_init(RabbitizerRegistersTracker *self, const RabbitizerRegistersTracker *other);
void RabbitizerRegistersTracker_destroy(RabbitizerRegistersTracker *self);

bool RabbitizerRegistersTracker_moveRegisters(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr);
void RabbitizerRegistersTracker_overwriteRegisters(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset);
void RabbitizerRegistersTracker_unsetRegistersAfterFuncCall(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, const RabbitizerInstruction *prevInstr);
bool RabbitizerRegistersTracker_getAddressIfCanSetType(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, uint32_t *dstAddress);
bool RabbitizerRegistersTracker_getJrInfo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int *dstOffset, uint32_t *dstAddress);

// prevInstr can be NULL
void RabbitizerRegistersTracker_processLui(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, const RabbitizerInstruction *prevInstr);
bool RabbitizerRegistersTracker_getLuiOffsetForConstant(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int *dstOffset);
void RabbitizerRegistersTracker_processConstant(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, uint32_t value, int offset);
bool RabbitizerRegistersTracker_getLuiOffsetForLo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, int instrOffset, int *dstOffset, bool *dstIsGp);
void RabbitizerRegistersTracker_processLo(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr, uint32_t value, int offset);
bool RabbitizerRegistersTracker_hasLoButNoHi(RabbitizerRegistersTracker *self, const RabbitizerInstruction *instr);


#endif
