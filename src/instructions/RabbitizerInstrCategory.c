/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrCategory.h"

//#include <stddef.h>
#include <string.h>

#define RABBITIZER_DEF_INSTR_CATEGORY(name) [RABBITIZER_INSTRCAT_##name] = #name

const char *const RabbitizerInstrCategory_Names[] = {
#include "instructions/InstrCategory.inc"
};

#undef RABBITIZER_DEF_INSTR_CATEGORY

RabbitizerInstrCategory RabbitizerInstrCategory_fromStr(const char *name) {
    if (name == NULL) {
        return -2;
    }

    for (size_t i = 0; i < RABBITIZER_INSTRCAT_MAX; i++) {
        if (strcmp(RabbitizerInstrCategory_Names[i], name) == 0) {
            return i;
        }
    }

    return -1;
}
