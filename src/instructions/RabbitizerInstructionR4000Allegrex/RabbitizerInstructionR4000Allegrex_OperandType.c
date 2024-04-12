/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstructionR4000Allegrex.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

#include "common/Utils.h"
#include "common/RabbitizerConfig.h"
#include "instructions/RabbitizerRegister.h"

size_t RabbitizerOperandType_process_r4000allegrex_pos(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride,
                                                       UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_pos(self);

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_size(const RabbitizerInstruction *self, char *dst,
                                                        UNUSED const char *immOverride,
                                                        UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_size(self) + 1;

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_size_plus_pos(const RabbitizerInstruction *self, char *dst,
                                                                 UNUSED const char *immOverride,
                                                                 UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_size_plus_pos(self) + 1 - RAB_INSTR_R4000ALLEGREX_GET_pos(self);

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}

size_t RabbitizerOperandType_process_r4000allegrex_imm3(const RabbitizerInstruction *self, char *dst,
                                                       UNUSED const char *immOverride, UNUSED size_t immOverrideLength) {
    size_t totalSize = 0;
    uint32_t temp = RAB_INSTR_R4000ALLEGREX_GET_imm3(self);

// TODO: consider making this a proper configuration
#if 0
    if (temp < 10) {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    } else {
        RABUTILS_BUFFER_SPRINTF(dst, totalSize, "0x%x", temp);
    }
#endif
    RABUTILS_BUFFER_SPRINTF(dst, totalSize, "%i", temp);
    return totalSize;
}
