/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_INSTRCATEGORY_H
#define RABBITIZER_INSTRCATEGORY_H
#pragma once

#ifdef __cplusplus
extern "C" {
#endif


#include "InstrCategory_enum.table.h"

extern const char *const RabbitizerInstrCategory_Names[];

RabbitizerInstrCategory RabbitizerInstrCategory_fromStr(const char *name);


#ifdef __cplusplus
}
#endif

#endif
