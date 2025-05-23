/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_VERSION_H
#define RABBITIZER_VERSION_H

#include "Utils.h"

#ifdef __cplusplus
extern "C" {
#endif


// Header version
#define RAB_VERSION_MAJOR 1
#define RAB_VERSION_MINOR 13
#define RAB_VERSION_PATCH 0

#define RAB_VERSION_STR RAB_STRINGIFY(RAB_VERSION_MAJOR) "." RAB_STRINGIFY(RAB_VERSION_MINOR) "." RAB_STRINGIFY(RAB_VERSION_PATCH)

#define RAB_VERSION_AUTHOR "Decompollaborate"


// Compiled library version
extern const int RabVersion_Major;
extern const int RabVersion_Minor;
extern const int RabVersion_Patch;

extern const char RabVersion_Str[];

extern const char RabVersion_Author[];


#ifdef __cplusplus
}
#endif

#endif
