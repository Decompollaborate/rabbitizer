/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "common/Utils.h"

#include <stdbool.h>
#include <string.h>


int32_t RabbitizerUtils_From2Complement(uint32_t number, int bits) {
    bool isNegative = number & (1 << (bits - 1));

    if (isNegative) {
        return -((~number + 1) & ((1 << bits) - 1));
    }

    return number;
}

size_t RabbitizerUtils_CharFill(char *dst, int count, char fillchar) {
    if (count <= 0) {
        return 0;
    }

    memset(dst, fillchar, count);

    return count;
}
