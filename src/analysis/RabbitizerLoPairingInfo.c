/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "analysis/RabbitizerLoPairingInfo.h"


void RabbitizerLoPairingInfo_Init(RabbitizerLoPairingInfo *self) {
    *self = (RabbitizerLoPairingInfo){ 
        .shouldProcess = false,
        .instrOffset = 0,
        .isGpRel = false,
        .isGpLoad = false,
    };
}
