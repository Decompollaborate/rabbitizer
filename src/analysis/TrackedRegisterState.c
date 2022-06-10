/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "analysis/TrackedRegisterState.h"

#include <assert.h>

#include "common/Utils.h"


void TrackedRegisterState_init(TrackedRegisterState *self, int registerNum) {
    self->registerNum = registerNum;

    self->hasLuiValue = false;
    self->luiOffset = 0;
    self->luiSetOnBranchLikely = false;

    self->hasLoValue = false;
    self->loOffset = 0;
    self->dereferenced = false;
    self->dereferenceOffset = 0;

    self->value = 0;
}

void TrackedRegisterState_destroy(UNUSED TrackedRegisterState *self) {
}


void TrackedRegisterState_clear(TrackedRegisterState *self) {
    self->hasLuiValue = false;
    self->luiOffset = 0;
    self->luiSetOnBranchLikely = false;
    self->hasLoValue = false;
    self->loOffset = 0;
    self->dereferenced = false;
    self->dereferenceOffset = 0;
    self->value = 0;
}

void TrackedRegisterState_clearHi(TrackedRegisterState *self) {
    self->hasLuiValue = false;
    self->luiOffset = 0;
    self->luiSetOnBranchLikely = false;
}

void TrackedRegisterState_clearLo(TrackedRegisterState *self) {
    self->hasLoValue = false;
    self->loOffset = 0;
    self->dereferenced = false;
    self->dereferenceOffset = 0;
    self->value = 0;
}


void TrackedRegisterState_copyState(TrackedRegisterState *self, const TrackedRegisterState *other) {
    self->hasLuiValue = other->hasLuiValue;
    self->luiOffset = other->luiOffset;
    self->luiSetOnBranchLikely = other->luiSetOnBranchLikely;

    self->hasLoValue = other->hasLoValue;
    self->loOffset = other->loOffset;
    self->dereferenced = other->dereferenced;
    self->dereferenceOffset = other->dereferenceOffset;

    self->value = other->value;
}


void TrackedRegisterState_setHi(TrackedRegisterState *self, int value, int offset) {
    self->hasLuiValue = true;
    self->luiOffset = offset;
    self->value = value << 16;
}

void TrackedRegisterState_setLo(TrackedRegisterState *self, int value, int offset) {
    self->value = value;
    self->loOffset = offset;
    self->hasLoValue = true;
    self->dereferenced = false;
    self->dereferenceOffset = 0;
}


void TrackedRegisterState_deref(TrackedRegisterState *self, int offset) {
    self->dereferenced = true;
    self->dereferenceOffset = offset;
}

void TrackedRegisterState_dereferenceState(TrackedRegisterState *self, const TrackedRegisterState *other, int offset) {
    assert(other->hasLoValue);
    assert(!other->dereferenced);

    TrackedRegisterState_copyState(self, other);
    TrackedRegisterState_deref(self, offset);
}


bool TrackedRegisterState_hasAnyValue(const TrackedRegisterState *self) {
    return self->hasLuiValue || self->hasLoValue;
}

bool TrackedRegisterState_wasSetInCurrentOffset(const TrackedRegisterState *self, int offset) {
    return self->loOffset == offset || self->dereferenceOffset == offset;
}
