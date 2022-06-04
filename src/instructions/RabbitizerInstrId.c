/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "instructions/RabbitizerInstrId.h"

#include "instructions/RabbitizerInstr.h"


#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...) \
    [RABBITIZER_INSTR_##prefix##_##name] = #name

#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...) \
    [RABBITIZER_INSTR_##prefix##_##name] = #altname



const char *RabbitizerInstrId_Names[] = {
    #include "instructions/RabbitizerInstrId_cpu.inc"
    #include "instructions/RabbitizerInstrId_rsp.inc"
};


const char *RabbitizerInstr_GetOpcodeName(const RabbitizerInstr *self) {
    return RabbitizerInstrId_Names[self->uniqueId];
}
