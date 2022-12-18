/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_REGISTER_H
#define RABBITIZER_REGISTER_H
#pragma once

#include <stdint.h>

#include "common/Utils.h"

#ifdef __cplusplus
extern "C" {
#endif


#include "Registers_enums.table.h"

extern const char *RabbitizerRegister_GprO32_Names[][2];
extern const char *RabbitizerRegister_GprN32_Names[][2];
extern const char *RabbitizerRegister_Cop0_Names[][2];
extern const char *RabbitizerRegister_Cop1O32_Names[][2];
extern const char *RabbitizerRegister_Cop1N32_Names[][2];
extern const char *RabbitizerRegister_Cop1N64_Names[][2];
extern const char *RabbitizerRegister_Cop1Control_Names[][2];
extern const char *RabbitizerRegister_Cop2_Names[][2];
extern const char *RabbitizerRegister_RspGpr_Names[][2];
extern const char *RabbitizerRegister_RspCop0_Names[][2];
extern const char *RabbitizerRegister_RspCop2_Names[][2];
extern const char *RabbitizerRegister_RspCop2Control_Names[][2];
extern const char *RabbitizerRegister_RspVector_Names[][2];
extern const char *RabbitizerRegister_R5900VF_Names[][2];
extern const char *RabbitizerRegister_R5900VI_Names[][2];


NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameGpr(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameCop0(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameCop1(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameCop1Control(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameCop2(uint8_t regValue);

NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameRspGpr(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameRspCop0(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameRspCop2(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameRspCop2Control(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameRspVector(uint8_t regValue);

NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameR5900VF(uint8_t regValue);
NODISCARD PURE RETURNS_NON_NULL
const char *RabbitizerRegister_getNameR5900VI(uint8_t regValue);


#ifdef __cplusplus
}
#endif

#endif
