/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrId.h"

#include <assert.h>

//#include "instructions/RabbitizerInstr.h"


#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...) \
    [RABBITIZER_INSTR_##prefix##_##name] = #name

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...) \
    [RABBITIZER_INSTR_##prefix##_##name] = #altname


const char *RabbitizerInstrId_Names[] = {
    #include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
    #include "instructions/instr_id/RabbitizerInstrId_rsp.inc"
};

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME


const RabbitizerInstrId RabbitizerInstrId_NotEmitedByCompilers[] = {
    RABBITIZER_INSTR_CPU_ID_add,
    RABBITIZER_INSTR_CPU_ID_addi,
    RABBITIZER_INSTR_CPU_ID_mtc0,
    RABBITIZER_INSTR_CPU_ID_mfc0,
    RABBITIZER_INSTR_CPU_ID_eret,
    RABBITIZER_INSTR_CPU_ID_tlbp,
    RABBITIZER_INSTR_CPU_ID_tlbr,
    RABBITIZER_INSTR_CPU_ID_tlbwi,
    RABBITIZER_INSTR_CPU_ID_cache,
    0,
};


const char *RabbitizerInstrId_getOpcodeName(RabbitizerInstrId uniqueId) {
    assert(uniqueId >= RABBITIZER_INSTR_CPU_ID_INVALID && uniqueId < RABBITIZER_INSTR_ID_MAX);
    assert(uniqueId != RABBITIZER_INSTR_CPU_ID_MAX);
    assert(uniqueId != RABBITIZER_INSTR_RSP_ID_MAX);

    return RabbitizerInstrId_Names[uniqueId];
}
