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


#endif
