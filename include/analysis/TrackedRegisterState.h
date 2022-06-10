/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef TRACKED_REGISTER_STATE_H
#define TRACKED_REGISTER_STATE_H
#pragma once

#include <stdbool.h>


typedef struct TrackedRegisterState {
    int registerNum;

    bool hasLuiValue;
    int luiOffset; // The offset of last lui which set a value to this register
    bool luiSetOnBranchLikely;

    bool hasLoValue;
    int loOffset;
    bool dereferenced;
    int dereferenceOffset;

    int value;
} TrackedRegisterState;


void TrackedRegisterState_init(TrackedRegisterState *self, int registerNum);
void TrackedRegisterState_destroy(TrackedRegisterState *self);

void TrackedRegisterState_clear(TrackedRegisterState *self);
void TrackedRegisterState_clearHi(TrackedRegisterState *self);
void TrackedRegisterState_clearLo(TrackedRegisterState *self);

void TrackedRegisterState_copyState(TrackedRegisterState *self, const TrackedRegisterState *other);

void TrackedRegisterState_setHi(TrackedRegisterState *self, int value, int offset);
void TrackedRegisterState_setLo(TrackedRegisterState *self, int value, int offset);

void TrackedRegisterState_deref(TrackedRegisterState *self, int offset);
void TrackedRegisterState_dereferenceState(TrackedRegisterState *self, const TrackedRegisterState *other, int offset);

bool TrackedRegisterState_hasAnyValue(const TrackedRegisterState *self);
bool TrackedRegisterState_wasSetInCurrentOffset(const TrackedRegisterState *self, int offset);


#endif
