/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_UTILS_H
#define RABBITIZER_UTILS_H

#include <stdint.h>


#define ARRAY_COUNT(arr) (sizeof(arr) / sizeof(arr[0]))


int32_t RabbitizerUtils_From2Complement(uint32_t number, int bits);


#endif
