/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "enums_utils.h"
#include "instructions/RabbitizerInstrId.h"


#define RAB_DEF_OPCODE(prefix, caseBits, name, ...)                   { "InstrId", #prefix "_" #name, RABBITIZER_INSTR_ID_##prefix##_##name, false, NULL },
#define RAB_DEF_OPCODE_ALTNAME(prefix, caseBits, name, altname, ...)  RAB_DEF_OPCODE(prefix, caseBits, name, __VA_ARGS__)

RabbitizerEnumMetadata rabbitizer_enum_InstrId_enumvalues[] = {
    #include "tables/Opcode.inc"

    RAB_DEF_OPCODE(ALL, , MAX, )
    { 0 },
};

#undef RAB_DEF_OPCODE
#undef RAB_DEF_OPCODE_ALTNAME

static PyMethodDef rabbitizer_enum_InstrId_methods[] = {
    { 0 },
};

DEF_ENUM(InstrId, "")
