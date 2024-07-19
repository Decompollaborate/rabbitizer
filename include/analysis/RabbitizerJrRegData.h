/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_JR_REG_DATA_H
#define RABBITIZER_JR_REG_DATA_H
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "common/Utils.h"

#include "RabbitizerTrackedRegisterState.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef struct RabbitizerJrRegData {
    bool hasInfo;

    int offset;
    uint32_t address;
    bool checkedForBranching;
    int lastBranchOffset;
} RabbitizerJrRegData;


NON_NULL(1)
void RabbitizerJrRegData_init(RabbitizerJrRegData *self);
NON_NULL(1, 2)
void RabbitizerJrRegData_copy(RabbitizerJrRegData *self, const RabbitizerJrRegData *other);

NON_NULL(1, 2)
void RabbitizerJrRegData_initFromRegisterState(RabbitizerJrRegData *self, const RabbitizerTrackedRegisterState *state);


#ifdef __cplusplus
}
#endif

#endif
