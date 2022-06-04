/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "common/Utils.h"

#include <stdbool.h>


int32_t RabbitizerUtils_From2Complement(uint32_t number, int bits) {
    bool isNegative = number & (1 << (bits - 1));

    if (isNegative) {
        return -((~number + 1) & ((1 << bits) - 1));
    }

    return number;
}
