/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_ACCESS_TYPE_H
#define RABBITIZER_ACCESS_TYPE_H
#pragma once

#ifdef __cplusplus
extern "C" {
#endif


#define RAB_DEF_ACCESSTYPE(name) RAB_ACCESSTYPE_##name,

typedef enum RabbitizerAccessType {
    #include "instructions/AccessType.inc"

    RAB_DEF_ACCESSTYPE(MAX)
} RabbitizerAccessType;

#undef RAB_DEF_ACCESSTYPE


#ifdef __cplusplus
}
#endif

#endif
