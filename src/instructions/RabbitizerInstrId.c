/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrId.h"

#include <assert.h>

#include "InstrId_Names_array.table.h"

bool RabbitizerInstrId_isValid(RabbitizerInstrId uniqueId) {
    switch (uniqueId) {
        case RABBITIZER_INSTR_ID_cpu_INVALID:
        case RABBITIZER_INSTR_ID_rsp_INVALID:
        case RABBITIZER_INSTR_ID_r5900_INVALID:
        case RABBITIZER_INSTR_ID_cpu_MAX:
        case RABBITIZER_INSTR_ID_rsp_MAX:
        case RABBITIZER_INSTR_ID_r5900_MAX:
        // case RABBITIZER_INSTR_ID_ALL_MAX: Same as last MAX
            return false;

        default:
            return true;
    }
}

const char *RabbitizerInstrId_getOpcodeName(RabbitizerInstrId uniqueId) {
    assert(uniqueId >= RABBITIZER_INSTR_ID_cpu_INVALID && uniqueId < RABBITIZER_INSTR_ID_ALL_MAX);

    return RabbitizerInstrId_Names[uniqueId];
}
