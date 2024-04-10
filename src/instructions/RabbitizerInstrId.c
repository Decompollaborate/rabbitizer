/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrId.h"

#include <assert.h>

#include "generated/InstrId_Names_array.h"

/**
 * Check the word corresponds to a valid instruction.
 */
bool RabbitizerInstrId_isValid(RabbitizerInstrId uniqueId) {
    switch (uniqueId) {
        case RABBITIZER_INSTR_ID_cpu_INVALID:
        case RABBITIZER_INSTR_ID_rsp_INVALID:
        case RABBITIZER_INSTR_ID_r3000gte_INVALID:
        case RABBITIZER_INSTR_ID_r5900_INVALID:
        case RABBITIZER_INSTR_ID_r4000allegrex_INVALID:
        case RABBITIZER_INSTR_ID_cpu_MAX:
        case RABBITIZER_INSTR_ID_rsp_MAX:
        case RABBITIZER_INSTR_ID_r3000gte_MAX:
        case RABBITIZER_INSTR_ID_r5900_MAX:
        case RABBITIZER_INSTR_ID_r4000allegrex_MAX:
            // case RABBITIZER_INSTR_ID_ALL_MAX: Same as last MAX
            return false;

        default:
            return true;
    }
}

const char *RabbitizerInstrId_getOpcodeName(RabbitizerInstrId uniqueId) {
    const char *name;

    assert(uniqueId >= RABBITIZER_INSTR_ID_cpu_INVALID && uniqueId < RABBITIZER_INSTR_ID_ALL_MAX);

    name = RabbitizerInstrId_Names[uniqueId];
    assert(name != NULL);

    return name;
}
