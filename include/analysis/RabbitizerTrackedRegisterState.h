/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_TRACKED_REGISTER_STATE_H
#define RABBITIZER_TRACKED_REGISTER_STATE_H
#pragma once

#include <stdbool.h>


typedef struct RabbitizerTrackedRegisterState {
    int registerNum;

    bool hasLuiValue;
    int luiOffset; // The offset of last lui which set a value to this register
    bool luiSetOnBranchLikely;

    bool hasLoValue;
    int loOffset;
    bool dereferenced;
    int dereferenceOffset;

    int value;
} RabbitizerTrackedRegisterState;


void RabbitizerTrackedRegisterState_init(RabbitizerTrackedRegisterState *self, int registerNum);
void RabbitizerTrackedRegisterState_destroy(RabbitizerTrackedRegisterState *self);

void RabbitizerTrackedRegisterState_clear(RabbitizerTrackedRegisterState *self);
void RabbitizerTrackedRegisterState_clearHi(RabbitizerTrackedRegisterState *self);
void RabbitizerTrackedRegisterState_clearLo(RabbitizerTrackedRegisterState *self);

void RabbitizerTrackedRegisterState_copyState(RabbitizerTrackedRegisterState *self, const RabbitizerTrackedRegisterState *other);

void RabbitizerTrackedRegisterState_setHi(RabbitizerTrackedRegisterState *self, int value, int offset);
void RabbitizerTrackedRegisterState_setLo(RabbitizerTrackedRegisterState *self, int value, int offset);

void RabbitizerTrackedRegisterState_deref(RabbitizerTrackedRegisterState *self, int offset);
void RabbitizerTrackedRegisterState_dereferenceState(RabbitizerTrackedRegisterState *self, const RabbitizerTrackedRegisterState *other, int offset);

bool RabbitizerTrackedRegisterState_hasAnyValue(const RabbitizerTrackedRegisterState *self);
bool RabbitizerTrackedRegisterState_wasSetInCurrentOffset(const RabbitizerTrackedRegisterState *self, int offset);


#endif
