/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums_utils.h"
#include "instructions/RabbitizerInstrId.h"


#define RABBITIZER_DEF_INSTR_ID(prefix, name, ...)                   { "InstrId", #prefix "_" #name, RABBITIZER_INSTR_ID_##prefix##_##name, false, NULL }
#define RABBITIZER_DEF_INSTR_ID_ALTNAME(prefix, name, altname, ...)  { "InstrId", #prefix "_" #name, RABBITIZER_INSTR_ID_##prefix##_##name, false, NULL }

RabbitizerEnumMetadata rabbitizer_enum_InstrId_enumvalues[] = {
    #include "instructions/instr_id/RabbitizerInstrId_cpu.inc"
    RABBITIZER_DEF_INSTR_ID(cpu, MAX, ),

    #include "instructions/instr_id/RabbitizerInstrId_rsp.inc"
    RABBITIZER_DEF_INSTR_ID(rsp, MAX, ),

    RABBITIZER_DEF_INSTR_ID(ALL, MAX, ),
    { 0 },
};

#undef RABBITIZER_DEF_INSTR_ID
#undef RABBITIZER_DEF_INSTR_ID_ALTNAME

static PyMethodDef rabbitizer_enum_InstrId_methods[] = {
    { 0 },
};

DEF_ENUM(InstrId, "")
