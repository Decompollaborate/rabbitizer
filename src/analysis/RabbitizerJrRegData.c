/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "analysis/RabbitizerJrRegData.h"

void RabbitizerJrRegData_init(RabbitizerJrRegData *self) {
    *self = (RabbitizerJrRegData){ 0 };
}

void RabbitizerJrRegData_copy(RabbitizerJrRegData *self, const RabbitizerJrRegData *other) {
    *self = *other;
}

void RabbitizerJrRegData_initFromRegisterState(RabbitizerJrRegData *self, const RabbitizerTrackedRegisterState *state) {
    RabbitizerJrRegData_init(self);

    if (!state->hasLoValue || !state->dereferenced) {
        return;
    }

    self->hasInfo = true;
    self->offset = state->loOffset;
    self->address = state->value;
    self->checkedForBranching = state->checkedForBranching;
    self->lastBranchOffset = state->lastBranchOffset;
}
